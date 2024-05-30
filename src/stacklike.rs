use std::{collections::HashMap, fmt::Debug, rc::Rc};

use either::Either::{self, Left, Right};

use crate::{
    ast::{AstNode, StackMod},
    types::{FTy, Function, GenBindings, Ty},
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

        fty.match_or_bind(stack_ty.clone().into(), bindings)?;
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

        for ftype in function.to() {
            self.push(ftype.clone().resolve(&bindings)?.as_ty().unwrap());
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
    fn push(&mut self, ty: FTy) {
        self.0.push(ty);
    }

    fn pop(&mut self) -> Option<FTy> {
        self.0.pop()
    }

    fn replace(&mut self, bindings: &GenBindings) -> Result<(), String> {
        for fty in self.0.iter_mut() {
            match fty {
                FTy::Gen(gen) => {
                    if let Some(new_fty) = bindings.get(gen) {
                        *fty = new_fty.clone()
                    }
                }
                FTy::T(ty) => *ty = ty.clone().resolve(bindings)?,
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FuncDef {
    gen_count: u32,
    from: FDStack,
    to: FDStack,
}

impl FuncDef {
    pub fn collect(mut self) -> Function {
        self.from.0.reverse();
        Function::new(
            &[],
            &[FTy::T(Ty::F(Box::new(Function::new(
                &self.from.0,
                &self.to.0,
            ))))],
        )
    }

    fn take_one(&mut self, fty: FTy, bindings: &mut GenBindings) -> Result<(), String> {
        match self.to.pop() {
            Some(stack_ty) => {
                fty.match_or_bind(stack_ty.clone(), bindings)?;
                Ok(())
            }
            None => {
                self.from.push(fty);
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
        self.from.replace(&bindings)?;
        for ftype in function.to() {
            self.to.push(ftype.clone().resolve(&bindings)?);
        }

        Ok(())
    }

    fn pop_function(&mut self) -> Result<Function, String> {
        if let Some(FTy::T(Ty::F(f))) = self.to.pop() {
            Ok(*f)
        } else {
            Err("Top of stack was not function".into())
        }
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
            let wanted_f = Ty::F(Box::new(Function::new(&from_t, &to_t)));

            println!("# {:?}", wanted_f);

            stack.apply(Function::new(
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
                stack.apply(Function::new(&[], &[FTy::T(Ty::F(Box::new(ft())))]))?;

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
        StackMod::Int(_) => stack.apply(Function::new(&[], &[Ty::Int.into()]))?,
    }
    Ok(())
}
