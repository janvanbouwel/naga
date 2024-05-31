use by_address::ByAddress;
use either::Either;

use std::iter::zip;
use std::ops::Deref;

use std::{collections::HashMap, rc::Rc};

pub type Gen = ByAddress<Rc<()>>;

fn new_gen() -> Gen {
    ByAddress(Rc::new(()))
}

pub type GenBindings = HashMap<Gen, FTy>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bound {
    pub upper: Option<Ty>,
    pub lower: Option<Ty>,
}

impl Bound {
    fn from_upper(ty: Ty) -> Bound {
        Bound {
            upper: Some(ty),
            lower: None,
        }
    }

    pub fn from_lower(ty: Ty) -> Bound {
        Bound {
            upper: None,
            lower: Some(ty),
        }
    }

    fn upper(&mut self, ty: Ty, bindings: &mut GenBindings) -> Result<(), String> {
        match self.upper.take() {
            Some(ub) => self.lower = Some(ub.greatest_lower_bound(ty, bindings)?),
            None => self.upper = Some(ty),
        }
        self.clone().sound(bindings)
    }

    fn lower(&mut self, ty: Ty, bindings: &mut GenBindings) -> Result<(), String> {
        match self.lower.take() {
            Some(lb) => self.lower = Some(lb.least_upper_bound(ty, bindings)?),
            None => self.lower = Some(ty),
        }
        self.clone().sound(bindings)
    }

    fn merge(&mut self, bound: Bound, bindings: &mut GenBindings) -> Result<(), String> {
        if let Some(ty) = bound.upper {
            self.upper(ty, bindings)?;
        }

        if let Some(ty) = bound.lower {
            self.lower(ty, bindings)?;
        }

        self.clone().sound(bindings)
    }

    pub fn resolve_gen(self, bindings: &GenBindings) -> Result<Bound, String> {
        Ok(Bound {
            upper: self.upper.map(|ub| ub.resolve_gen(bindings)).transpose()?,
            lower: self.lower.map(|lb| lb.resolve_gen(bindings)).transpose()?,
        })
    }

    pub fn as_ty(self) -> Result<Ty, String> {
        match self.lower {
            Some(ty) => Ok(ty),
            None => Err("No lower bound".into()),
        }
    }

    fn sound(self, bindings: &mut GenBindings) -> Result<(), String> {
        if let Some(ub) = self.upper {
            if let Some(lb) = self.lower {
                let lub = ub
                    .clone()
                    .least_upper_bound(lb, bindings)?
                    .resolve_gen(bindings)?;
                let rub = ub.clone().resolve_gen(bindings)?;

                if rub != lub {
                    return Err("not sound".into());
                }
            }
        }
        Ok(())
    }
}

fn resolve(gen: &Gen, bindings: &GenBindings) -> Result<FTy, String> {
    let mut key = gen;
    loop {
        if let Some(fty) = bindings.get(key) {
            match fty {
                FTy::B(_bound) => return Ok(fty.clone()),
                FTy::Gen(new_key) => {
                    if new_key == gen {
                        return Err(std::format!(
                            "resolving looped key {gen:?} bindings {bindings:?}"
                        ));
                    }
                    key = new_key;
                    continue;
                }
            }
        }
        break;
    }
    Ok(FTy::Gen(key.clone()))
}

fn resolve_and_pop(
    fty: &Gen,
    bindings: &mut GenBindings,
) -> Result<Either<Gen, (Gen, Bound)>, String> {
    let mut key: Gen = fty.clone();

    loop {
        if let Some(new_fty) = bindings.get(&key) {
            match new_fty {
                FTy::B(_) => break,
                FTy::Gen(new_gen) => {
                    if new_gen == fty {
                        return Err("resolving looped".into());
                    }
                    key = new_gen.clone();
                    continue;
                }
            }
        }

        break;
    }

    Ok(match bindings.remove(&key) {
        Some(FTy::B(ty)) => Either::Right((key.clone(), ty)),
        None => Either::Left(key.clone()),
        _ => unreachable!(),
    })
    // Ok(key.clone())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenOrTy {
    T(Ty),
    Gen(Gen),
}

impl GenOrTy {
    pub fn new_gen() -> GenOrTy {
        GenOrTy::Gen(new_gen())
    }
}

impl From<Ty> for GenOrTy {
    fn from(value: Ty) -> Self {
        GenOrTy::T(value)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum FTy {
    B(Bound),
    Gen(Gen),
}

impl std::fmt::Debug for FTy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::B(arg0) => f.debug_tuple("B").field(arg0).finish(),
            Self::Gen(arg0) => f
                .debug_tuple("Gen")
                .field(&(arg0.deref().deref() as *const ()))
                .finish(),
        }
    }
}

impl FTy {
    pub fn new_gen() -> FTy {
        FTy::Gen(new_gen())
    }

    pub fn resolve_gen(self, bindings: &GenBindings) -> Result<FTy, String> {
        match self {
            FTy::B(bound) => Ok(FTy::B(bound.resolve_gen(bindings)?)),
            FTy::Gen(gen) => match resolve(&gen, bindings)? {
                FTy::B(bound) => Ok(FTy::B(bound.resolve_gen(bindings)?)),
                FTy::Gen(key) => Ok(FTy::Gen(key)),
            },
        }
    }

    pub fn merge(self, other_fty: FTy, bindings: &mut GenBindings) -> Result<FTy, String> {
        match self {
            FTy::Gen(gen) => match resolve_and_pop(&gen, bindings)? {
                Either::Left(key) => match other_fty {
                    FTy::B(other_bound) => {
                        bindings.insert(key.clone(), FTy::B(other_bound.clone()));
                        Ok(FTy::Gen(key))
                    }
                    FTy::Gen(other_gen) => match resolve_and_pop(&other_gen, bindings)? {
                        Either::Left(other_key) => {
                            if key != other_key {
                                bindings.insert(key.clone(), FTy::Gen(other_key));
                            }
                            Ok(FTy::Gen(key))
                        }
                        Either::Right((other_key, other_bound)) => {
                            bindings.insert(other_key.clone(), FTy::B(other_bound));
                            bindings.insert(key, FTy::Gen(other_key.clone()));
                            Ok(FTy::Gen(other_key))
                        }
                    },
                },
                Either::Right((key, mut bound)) => match other_fty {
                    FTy::B(other_bound) => {
                        bound.merge(other_bound.clone(), bindings)?;
                        bindings.insert(key.clone(), FTy::B(bound));
                        Ok(FTy::Gen(key))
                    }
                    FTy::Gen(other_gen) => match resolve_and_pop(&other_gen, bindings)? {
                        Either::Left(other_key) => {
                            bindings.insert(other_key, FTy::Gen(key.clone()));
                            Ok(FTy::Gen(key))
                        }
                        Either::Right((other_key, other_bound)) => {
                            bindings.insert(other_key, FTy::Gen(key.clone()));

                            bound.merge(other_bound, bindings)?;
                            bindings.insert(key.clone(), FTy::B(bound));
                            Ok(FTy::Gen(key))
                        }
                    },
                },
            },
            FTy::B(ref bound) => match other_fty {
                FTy::B(stack_bound) => {
                    let mut new_bound = bound.clone();
                    new_bound.merge(stack_bound.clone(), bindings)?;
                    Ok(FTy::B(new_bound))
                }
                FTy::Gen(_) => other_fty.merge(self, bindings),
            },
        }
    }

    pub fn as_bound(self) -> Result<Bound, String> {
        match self {
            FTy::B(bound) => Ok(bound),
            FTy::Gen(_) => Err("fty was generic".into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    from: Vec<FTy>,
    to: Vec<FTy>,
}

impl Function {
    pub fn new(from: Vec<FTy>, to: Vec<FTy>) -> Function {
        Function { from, to }
    }

    pub fn from_fty(from: &[GenOrTy], to: &[GenOrTy]) -> Self {
        Function {
            from: from
                .iter()
                .map(|fty: &GenOrTy| match fty {
                    GenOrTy::T(ty) => FTy::B(Bound::from_upper(ty.clone())),
                    GenOrTy::Gen(gen) => FTy::Gen(gen.clone()),
                })
                .collect(),

            to: to
                .iter()
                .map(|fty: &GenOrTy| match fty {
                    GenOrTy::T(ty) => FTy::B(Bound::from_lower(ty.clone())),
                    GenOrTy::Gen(gen) => FTy::Gen(gen.clone()),
                })
                .collect(),
        }
    }

    pub fn from(&self) -> &[FTy] {
        &self.from
    }

    pub fn to(&self) -> &[FTy] {
        &self.to
    }

    fn resolve_gen(self, bindings: &GenBindings) -> Result<Function, String> {
        let mut from = vec![];
        for fty in self.from {
            from.push(fty.resolve_gen(bindings)?);
        }

        let mut to = vec![];
        for fty in self.to {
            to.push(fty.resolve_gen(bindings)?);
        }

        Ok(Function::new(from, to))
    }

    fn greatest_lower_bound(
        self,
        other: Function,
        bindings: &mut GenBindings,
    ) -> Result<Function, String> {
        if self.from.len().overflowing_sub(other.from.len())
            != self.to.len().overflowing_sub(other.to.len())
        {
            return Err("Functions have incompatible length".into());
        }

        let mut from: Vec<FTy> = vec![];
        let mut rem_from: Vec<FTy> = vec![];

        for pair in itertools::Itertools::zip_longest(self.from.into_iter(), other.from) {
            match pair {
                itertools::EitherOrBoth::Both(self_arg, wanted_arg) => {
                    from.push(self_arg.merge(wanted_arg, bindings)?)
                }
                itertools::EitherOrBoth::Left(arg) => rem_from.push(arg),
                itertools::EitherOrBoth::Right(arg) => rem_from.push(arg),
            }
        }

        let mut to: Vec<FTy> = vec![];
        let mut rem_to: Vec<FTy> = vec![];

        for pair in itertools::Itertools::zip_longest(self.to.into_iter(), other.to) {
            match pair {
                itertools::EitherOrBoth::Both(self_arg, wanted_arg) => {
                    to.push(self_arg.merge(wanted_arg, bindings)?);
                }
                itertools::EitherOrBoth::Left(arg) => rem_to.push(arg),
                itertools::EitherOrBoth::Right(arg) => rem_to.push(arg),
            }
        }

        let mut rem = vec![];
        for (from, to) in zip(rem_from, rem_to) {
            rem.push(from.merge(to, bindings)?);
        }

        Ok(Function::new(from, to))
    }

    fn least_upper_bound(
        self,
        other: Function,
        bindings: &mut GenBindings,
    ) -> Result<Function, String> {
        if self.from.len().overflowing_sub(other.from.len())
            != self.to.len().overflowing_sub(other.to.len())
        {
            return Err("Functions have incompatible length".into());
        }

        let mut from: Vec<FTy> = vec![];
        let mut rem_from: Vec<FTy> = vec![];

        for pair in itertools::Itertools::zip_longest(self.from.into_iter(), other.from) {
            match pair {
                itertools::EitherOrBoth::Both(self_arg, wanted_arg) => {
                    from.push(self_arg.merge(wanted_arg, bindings)?)
                }
                itertools::EitherOrBoth::Left(arg) => rem_from.push(arg),
                itertools::EitherOrBoth::Right(arg) => rem_from.push(arg),
            }
        }

        let mut to: Vec<FTy> = vec![];
        let mut rem_to: Vec<FTy> = vec![];

        for pair in itertools::Itertools::zip_longest(self.to.into_iter(), other.to) {
            match pair {
                itertools::EitherOrBoth::Both(self_arg, wanted_arg) => {
                    to.push(self_arg.merge(wanted_arg, bindings)?);
                }
                itertools::EitherOrBoth::Left(arg) => rem_to.push(arg),
                itertools::EitherOrBoth::Right(arg) => rem_to.push(arg),
            }
        }

        let mut rem = vec![];
        for (from, to) in zip(rem_from, rem_to) {
            rem.push(from.merge(to, bindings)?);
        }

        from.extend_from_slice(&rem);
        to.extend(rem);

        Ok(Function::new(from, to))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ty {
    Bool,
    Int,
    F(Box<Function>),
}

impl Ty {
    pub fn resolve_gen(self, bindings: &GenBindings) -> Result<Ty, String> {
        match self {
            Ty::F(f) => Ok(Ty::F(Box::new(f.resolve_gen(bindings)?))),
            _ => Ok(self.clone()),
        }
    }

    fn greatest_lower_bound(self, wanted_ty: Ty, bindings: &mut GenBindings) -> Result<Ty, String> {
        match self {
            Self::F(f) => {
                if let Ty::F(other_f) = wanted_ty {
                    Ok(Ty::F(Box::new(f.greatest_lower_bound(*other_f, bindings)?)))
                } else {
                    Err(std::format!(
                        "Types didn't match {:?} {:?}",
                        Ty::F(f),
                        wanted_ty
                    ))
                }
            }
            _ => {
                if self != wanted_ty {
                    Err(std::format!(
                        "Types didn't match {:?} {:?}",
                        self,
                        wanted_ty
                    ))
                } else {
                    Ok(wanted_ty)
                }
            }
        }
    }

    fn least_upper_bound(self, wanted_ty: Ty, bindings: &mut GenBindings) -> Result<Ty, String> {
        match self {
            Self::F(f) => {
                if let Ty::F(other_f) = wanted_ty {
                    Ok(Ty::F(Box::new(f.least_upper_bound(*other_f, bindings)?)))
                } else {
                    Err(std::format!(
                        "Types didn't match {:?} {:?}",
                        Ty::F(f),
                        wanted_ty
                    ))
                }
            }
            _ => {
                if self != wanted_ty {
                    Err(std::format!(
                        "Types didn't match {:?} {:?}",
                        self,
                        wanted_ty
                    ))
                } else {
                    Ok(wanted_ty)
                }
            }
        }
    }
}
