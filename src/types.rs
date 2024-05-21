use by_address::ByAddress;

use std::ops::Deref;
use std::{collections::HashMap, rc::Rc};

pub type Gen = ByAddress<Rc<()>>;

pub type GenBindings = HashMap<Gen, FTy>;

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

    pub fn match_or_bind(&self, stack_fty: FTy, bindings: &mut GenBindings) -> Result<(), String> {
        match resolve(self, bindings)? {
            FTy::Gen(gen) => {
                bindings.insert(gen, stack_fty);
                Ok(())
            }
            FTy::T(ty) => match resolve(&stack_fty, bindings)? {
                FTy::T(stack_ty) => ty.match_or_bind(&stack_ty, bindings),
                FTy::Gen(gen) => {
                    bindings.insert(gen, FTy::T(ty));
                    Ok(())
                }
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

    fn match_or_bind(
        &self,
        wanted_ty: &Function,
        bindings: &mut GenBindings,
    ) -> Result<(), String> {
        // if self.from.len().overflowing_sub(wanted_ty.from.len())
        //     != self.to.len().overflowing_sub(wanted_ty.to.len())
        // {
        //     return Err("Functions have incompatible length".into());
        // }
        if self.from.len() != wanted_ty.from.len() {
            return Err("Functions don't have same from length".into());
        }

        if self.to.len() != wanted_ty.to.len() {
            return Err("Functions don't have same to length".into());
        }

        for (self_arg, wanted_arg) in std::iter::zip(self.from.clone(), wanted_ty.from.clone()) {
            self_arg.match_or_bind(wanted_arg, bindings)?;
        }

        for (self_arg, wanted_arg) in std::iter::zip(self.to.clone(), wanted_ty.to.clone()) {
            self_arg.match_or_bind(wanted_arg, bindings)?;
        }

        Ok(())
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

    fn match_or_bind(&self, wanted_ty: &Ty, bindings: &mut GenBindings) -> Result<(), String> {
        match self {
            Self::F(f) => {
                if let Ty::F(other_f) = wanted_ty {
                    f.match_or_bind(other_f, bindings)
                } else {
                    Err(std::format!(
                        "Types didn't match {:?} {:?}",
                        self,
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
                    Ok(())
                }
            }
        }
    }
}
