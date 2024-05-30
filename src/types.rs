use by_address::ByAddress;
use either::Either;

use std::ops::Deref;
use std::{collections::HashMap, rc::Rc};

pub type Gen = ByAddress<Rc<()>>;

pub type GenBindings = HashMap<Gen, FTy>;
pub type GenReplace = HashMap<Gen, Gen>;

fn resolve(fty: &FTy, bindings: &GenBindings) -> Result<FTy, String> {
    let mut loop_fty = fty;
    loop {
        if let FTy::Gen(gen) = loop_fty {
            if let Some(new_fty) = bindings.get(gen) {
                if new_fty == fty {
                    return Err("resolving looped".into());
                }
                loop_fty = new_fty;
                continue;
            }
        }
        break;
    }
    Ok(loop_fty.clone())
}

fn resolve_and_pop(
    fty: &Gen,
    bindings: &mut GenBindings,
) -> Result<Either<Gen, (Gen, Ty)>, String> {
    let mut key: Gen = fty.clone();

    loop {
        if let Some(new_fty) = bindings.get(&key) {
            match new_fty {
                FTy::T(_) => break,
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
        Some(FTy::T(ty)) => Either::Right((key.clone(), ty)),
        None => Either::Left(key.clone()),
        _ => unreachable!(),
    })
    // Ok(key.clone())
}

#[derive(Clone, PartialEq, Eq)]
pub enum FTy {
    T(Ty),
    Gen(Gen),
}

impl std::fmt::Debug for FTy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::T(arg0) => f.debug_tuple("T").field(arg0).finish(),
            Self::Gen(arg0) => f
                .debug_tuple("Gen")
                .field(&(arg0.deref().deref() as *const ()))
                .finish(),
        }
    }
}

impl FTy {
    pub fn new_gen() -> FTy {
        FTy::Gen(ByAddress(Rc::new(())))
    }

    pub fn resolve(self, bindings: &GenBindings) -> Result<FTy, String> {
        match resolve(&self, bindings)? {
            FTy::Gen(_) => Ok(self),
            FTy::T(ty) => Ok(FTy::T(ty.resolve(bindings)?)),
        }
    }

    pub fn match_or_bind(self, other_fty: FTy, bindings: &mut GenBindings) -> Result<FTy, String> {
        match self {
            FTy::Gen(gen) => match resolve_and_pop(&gen, bindings)? {
                Either::Left(resolved_gen) => {
                    bindings.insert(resolved_gen, other_fty.clone());
                    Ok(other_fty)
                }
                Either::Right((key, ty)) => match other_fty {
                    FTy::T(other_ty) => {
                        let new_fty = FTy::T(ty.match_or_bind(other_ty, bindings)?);
                        bindings.insert(key, new_fty.clone());
                        Ok(new_fty)
                    }
                    FTy::Gen(_) => {
                        let new_fty = other_fty.match_or_bind(FTy::T(ty), bindings)?;
                        bindings.insert(key, new_fty.clone());
                        Ok(new_fty)
                    }
                },
            },
            FTy::T(ty) => match other_fty {
                FTy::T(stack_ty) => Ok(FTy::T(ty.match_or_bind(stack_ty, bindings)?)),
                FTy::Gen(_) => other_fty.match_or_bind(FTy::T(ty), bindings),
            },
        }
    }

    pub fn as_ty(&self) -> Result<Ty, String> {
        match self {
            FTy::T(ty) => Ok(ty.clone()),
            _ => Err("fty wasn't ty".into()),
        }
    }
}

impl From<Ty> for FTy {
    fn from(value: Ty) -> Self {
        FTy::T(value)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    from: Vec<FTy>,
    to: Vec<FTy>,
}

impl Function {
    pub fn new(from: &[FTy], to: &[FTy]) -> Self {
        Function {
            from: Vec::from(from),
            to: Vec::from(to),
        }
    }

    pub fn from(&self) -> &[FTy] {
        &self.from
    }

    pub fn to(&self) -> &[FTy] {
        &self.to
    }

    fn resolve(self, bindings: &GenBindings) -> Result<Function, String> {
        let mut from = vec![];
        for fty in self.from {
            from.push(resolve(&fty, bindings)?);
        }

        let mut to = vec![];
        for fty in self.to {
            to.push(resolve(&fty, bindings)?);
        }

        Ok(Function::new(&from, &to))
    }

    fn least_upper_bound(
        self,
        wanted_ty: Function,
        bindings: &mut GenBindings,
    ) -> Result<Function, String> {
        if self.from.len().overflowing_sub(wanted_ty.from.len())
            != self.to.len().overflowing_sub(wanted_ty.to.len())
        {
            return Err("Functions have incompatible length".into());
        }

        let mut from: Vec<FTy> = vec![];

        for pair in itertools::Itertools::zip_longest(self.from.into_iter(), wanted_ty.from) {
            match pair {
                itertools::EitherOrBoth::Both(self_arg, wanted_arg) => {
                    from.push(self_arg.match_or_bind(wanted_arg, bindings)?)
                }
                itertools::EitherOrBoth::Left(arg) => from.push(arg),
                itertools::EitherOrBoth::Right(arg) => from.push(arg),
            }
        }

        let mut to: Vec<FTy> = vec![];

        for pair in itertools::Itertools::zip_longest(self.to.into_iter(), wanted_ty.to) {
            match pair {
                itertools::EitherOrBoth::Both(self_arg, wanted_arg) => {
                    to.push(self_arg.match_or_bind(wanted_arg, bindings)?)
                }
                itertools::EitherOrBoth::Left(arg) => to.push(arg),
                itertools::EitherOrBoth::Right(arg) => to.push(arg),
            }
        }

        Ok(Function::new(&from, &to))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ty {
    Bool,
    Int,
    F(Box<Function>),
}

impl Ty {
    pub fn resolve(self, bindings: &GenBindings) -> Result<Ty, String> {
        match self {
            Ty::F(f) => Ok(Ty::F(Box::new(f.resolve(bindings)?))),
            _ => Ok(self.clone()),
        }
    }

    fn match_or_bind(self, wanted_ty: Ty, bindings: &mut GenBindings) -> Result<Ty, String> {
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
