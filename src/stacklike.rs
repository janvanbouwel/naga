use std::{collections::HashMap, fmt::Debug, rc::Rc};

use either::Either::{self, Left, Right};

use crate::{
    ast::{AstNode, StackMod},
    types::{Bound, FTy, Function, GenBindings, Ty},
};

pub type Store = HashMap<String, Rc<dyn Fn() -> Function>>;

pub trait StackLike: Debug + Default
where
    Self: Sized + Clone,
{
    fn apply(&mut self, function: Function) -> Result<(), String>;

    fn pop_function(&mut self) -> Result<Function, String>;
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Stack(Vec<Ty>);

impl Stack {
    pub fn new(tys: &[Ty]) -> Self {
        Stack(Vec::from(tys))
    }

    fn push(&mut self, ty: Ty) {
        self.0.push(ty);
    }

    pub fn pop(&mut self) -> Option<Ty> {
        self.0.pop()
    }

    fn take_one(&mut self, fty: FTy, bindings: &mut GenBindings) -> Result<(), String> {
        let stack_ty = self
            .pop()
            .ok_or("Cannot take from empty stack".to_string())?;

        fty.merge(FTy::B(Bound::from_lower(stack_ty)), bindings)?;
        Ok(())
    }

    fn take_all(&mut self, types: &[FTy], bindings: &mut GenBindings) -> Result<(), String> {
        if let Some((ty, tys)) = types.split_last() {
            self.take_one(ty.clone(), bindings)?;
            self.take_all(tys, bindings)
        } else {
            Ok(())
        }
    }
}

impl StackLike for Stack {
    fn apply(&mut self, function: Function) -> Result<(), String> {
        let mut bindings = GenBindings::new();
        self.take_all(function.from(), &mut bindings)?;

        println!("# bindings {bindings:?}");

        for fty in function.to() {
            self.push(fty.clone().resolve_gen(&bindings)?.as_ty()?);
            // self.push(ftype.clone().resolve(&bindings)?.as_ty().unwrap());
            println!("# {self:?}");
        }

        Ok(())
    }

    fn pop_function(&mut self) -> Result<Function, String> {
        if let Some(Ty::F(f)) = self.0.pop() {
            Ok(*f)
        } else {
            Err("Top of stack was not function".into())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct FDStack(Vec<FTy>);

impl FDStack {
    fn push(&mut self, fty: FTy) {
        self.0.push(fty);
    }

    fn pop(&mut self) -> Option<FTy> {
        self.0.pop()
    }

    fn replace(&mut self, bindings: &GenBindings, swap: bool) -> Result<(), String> {
        for fty in self.0.iter_mut() {
            if let FTy::Gen(_) = fty {
                match fty.clone().resolve_gen(bindings)? {
                    FTy::B(mut bound) => {
                        if bound.upper.is_some() {
                            unreachable!(
                                "{}",
                                std::format!("funcdef from has upper bound: {bound:?}")
                            )
                        }
                        if bound.lower.is_none() {
                            unreachable!("more uh oh")
                        }
                        if swap {
                            std::mem::swap(&mut bound.upper, &mut bound.lower);
                        }
                        *fty = FTy::B(bound);
                    }
                    FTy::Gen(new_gen) => *fty = FTy::Gen(new_gen),
                    FTy::T(ty) => *fty = FTy::T(ty),
                }
            }
        }
        Ok(())
    }

    fn collect_from(mut self) -> Vec<FTy> {
        self.0.reverse();
        for fty in self.0.iter_mut() {
            if let FTy::B(bound) = fty {
                if bound.upper.is_none() {
                    *fty = FTy::T(bound.clone().as_ty().unwrap())
                } // TODO only keep upper?
            }
        }
        self.0
    }

    fn collect_to(mut self) -> Vec<FTy> {
        for fty in self.0.iter_mut() {
            if let FTy::B(bound) = fty {
                bound.upper = None;
                *fty = bound.clone().as_ty().unwrap().into();
            }
        }
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FuncDef {
    gen_count: u32,
    from: FDStack,
    to: FDStack,
}

impl FuncDef {
    pub fn collect(self) -> Function {
        Function::from_fty(
            &[],
            &[Ty::F(Box::new(Function::new(
                self.from.collect_from(),
                self.to.collect_to(),
            )))
            .into()],
        )
    }

    fn take_one(&mut self, fty: FTy, bindings: &mut GenBindings) -> Result<(), String> {
        match self.to.pop() {
            Some(stack_ty) => {
                fty.merge(stack_ty, bindings)?;
                Ok(())
            }
            None => {
                self.from.push(fty.clone());
                Ok(())
            }
        }
    }

    fn take_all(&mut self, types: &[FTy], bindings: &mut GenBindings) -> Result<(), String> {
        if let Some((ty, tys)) = types.split_last() {
            self.take_one(ty.clone(), bindings)?;
            self.take_all(tys, bindings)
        } else {
            Ok(())
        }
    }
}

impl StackLike for FuncDef {
    fn apply(&mut self, function: Function) -> Result<(), String> {
        println!("# funcdef apply {self:?}");
        println!("#  function: {function:?}");
        let mut bindings = GenBindings::new();
        self.take_all(function.from(), &mut bindings)?;

        println!("# bindings: {bindings:?}");
        self.from.replace(&bindings, true)?;
        self.to.replace(&bindings, false)?;
        for fty in function.to() {
            println!("# to fty: {fty:?}");
            self.to.push(fty.clone().resolve_gen(&bindings)?);
        }

        Ok(())
    }

    fn pop_function(&mut self) -> Result<Function, String> {
        match self.to.pop() {
            Some(FTy::B(bound)) => {
                if let Ty::F(f) = bound.as_ty()? {
                    return Ok(*f);
                }
            }
            Some(FTy::T(Ty::F(f))) => return Ok(*f),
            _ => {}
        }
        Err("Top of stack was not function".into())
    }
}

pub trait Context: Sized {
    fn check_node(self, node: &AstNode) -> Result<Either<Self, Function>, String>;
}

enum ContextType {
    FD(FDContext),
}

impl Context for ContextType {
    fn check_node(self, node: &AstNode) -> Result<Either<ContextType, Function>, String> {
        match self {
            ContextType::FD(fd) => match fd.check_node(node)? {
                Either::Left(ctx) => Ok(Left(Self::FD(ctx))),
                Either::Right(f) => Ok(Right(f)),
            },
        }
    }
}
pub struct BaseContext<S: StackLike> {
    stack: S,
    store: Store,
    child_context: Option<Box<ContextType>>,
}

impl<S: StackLike> BaseContext<S> {
    pub fn new(store: Store) -> Self {
        Self {
            stack: S::default(),
            store,
            child_context: None,
        }
    }
}

pub type ExecContext = BaseContext<Stack>;

impl Context for ExecContext {
    fn check_node(mut self, node: &AstNode) -> Result<Either<ExecContext, Function>, String> {
        match self.child_context.take() {
            Some(ctx) => match ctx.check_node(node)? {
                Left(new_ctx) => self.child_context = Some(Box::new(new_ctx)),
                Right(f) => {
                    self.stack.apply(f)?;
                }
            },
            None => match node {
                AstNode::S(stackmod) => mod_stack(stackmod, &mut self.stack, &mut self.store)?,
                AstNode::OpenFunc => {
                    self.child_context = Some(Box::new(ContextType::FD(FDContext::new(
                        self.store.clone(),
                    ))));
                }
                AstNode::CloseFunc => return Err("Not in function context".into()),
            },
        }
        Ok(Left(self))
    }
}

pub type FDContext = BaseContext<FuncDef>;

impl Context for FDContext {
    fn check_node(mut self, node: &AstNode) -> Result<Either<FDContext, Function>, String> {
        match self.child_context.take() {
            Some(ctx) => match ctx.check_node(node)? {
                Left(new_ctx) => self.child_context = Some(Box::new(new_ctx)),
                Right(f) => {
                    self.stack.apply(f)?;
                }
            },
            None => match node {
                AstNode::S(stackmod) => {
                    mod_stack(stackmod, &mut self.stack, &mut self.store)?;
                }
                AstNode::OpenFunc => {
                    self.child_context = Some(Box::new(ContextType::FD(FDContext::new(
                        self.store.clone(),
                    ))));
                }
                AstNode::CloseFunc => return Ok(Right(self.stack.collect())),
            },
        }
        Ok(Left(self))
    }
}

fn mod_stack(
    stackmod: &StackMod,
    stack: &mut impl StackLike,
    store: &mut Store,
) -> Result<(), String> {
    match stackmod {
        StackMod::Arity(from, to) => {
            println!("# arity from {} to {}", from, to);

            let from_t: Vec<FTy> = (0..*from).map(|_| FTy::new_gen()).collect();
            let to_t: Vec<FTy> = (*from..(from + to)).map(|_| FTy::new_gen()).collect();
            let wanted_f = Ty::F(Box::new(Function::from_fty(&from_t, &to_t)));

            println!("# {:?}", wanted_f);

            stack.apply(Function::from_fty(
                &[wanted_f.clone().into()],
                &[wanted_f.clone().into()],
            ))?;
        }
        StackMod::Id(id) => match store.get(id.as_str()) {
            Some(ft) => {
                stack.apply(ft())?;

                println!("# {}\t {:?}", id, stack);
            }
            None => return Err("Identifier not in stack".into()),
        },
        StackMod::Quote(id) => match store.get(id.as_str()) {
            Some(ft) => {
                stack.apply(Function::from_fty(&[], &[Ty::F(Box::new(ft())).into()]))?;

                println!("# '{}\t {:?}", id, stack);
            }
            None => return Err("Identifier not in stack".into()),
        },
        StackMod::Apply => {
            let f = stack.pop_function()?;
            stack.apply(f)?;
        }
        StackMod::Bind(id) => {
            let f = Box::new(stack.pop_function()?);

            store.insert(id.clone(), Rc::new(move || *f.clone()));

            println!("# ${}\t {:?}", id, stack);
            println!("# bound: {:?}", store.get(id).unwrap()());
        }
        StackMod::Int(_) => stack.apply(Function::from_fty(&[], &[Ty::Int.into()]))?,
    }
    Ok(())
}
