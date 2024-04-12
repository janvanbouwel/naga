use immutable_map::TreeMap;

use crate::{ast::AstNode, builtins::initial_context};

type GenericBindings = TreeMap<u32, Type>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stack(Vec<Type>);

impl Stack {
    fn new(tys: &[Type]) -> Self {
        Stack(Vec::from(tys))
    }

    // fn push(&self, ty: &Type) -> StackT {
    //     let mut tys = self.0.clone();
    //     tys.push(ty.clone());
    //     StackT(tys)
    // }

    fn push(&self, ty: &Type) -> Stack {
        let mut tys = self.0.clone();
        tys.push(ty.clone());
        Stack(tys)
    }

    fn split_last(&self) -> Option<(&Type, Stack)> {
        self.0
            .split_last()
            .map(|(ty, rest)| (ty, Stack::new(rest)))
    }

    fn take(
        &self,
        other: &[Type],
        generics: &GenericBindings,
    ) -> Result<(Stack, GenericBindings), String> {
        match other.split_last() {
            None => Ok((self.clone(), generics.clone())),
            Some((other_t, other_prev)) => match self.split_last() {
                None => Err("Cannot take from empty stack".into()),
                Some((stack_t, prev)) => match other_t {
                    Type::Gen(n) => match generics.get(n) {
                        None => {
                            prev.take(&other_prev, &generics.insert(n.clone(), stack_t.clone()))
                        }
                        Some(bound_t) => {
                            if bound_t != stack_t {
                                return Err(std::format!("Types didn't match"));
                            }
                            prev.take(&other_prev, generics)
                        }
                    },
                    _ => {
                        if stack_t != other_t {
                            return Err(std::format!(
                                "Types didn't match {:?} {:?}",
                                stack_t,
                                other_t
                            ));
                        };
                        prev.take(&other_prev, generics)
                    }
                },
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    from: Vec<Type>,
    to: Vec<Type>,
}

impl Function {
    pub fn new(from: &[Type], to: &[Type]) -> Self {
        Function {
            from: Vec::from(from),
            to: Vec::from(to),
        }
    }

    fn apply(
        &self,
        stack: &Stack,
        generics: &GenericBindings,
    ) -> Result<(Stack, GenericBindings), String> {
        let (stack, generics) = stack.take(&self.from, &generics)?;

        let mut stack = stack;
        for ty in self.to.clone() {
            match ty {
                Type::Gen(n) => match generics.get(&n) {
                    None => return Err("generic wasn't bound".into()),
                    Some(bound_t) => stack = stack.push(&bound_t),
                },
                _ => stack = stack.push(&ty),
            }
        }

        Ok((stack, generics))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Bool,
    Func(Function),
    Gen(u32),
}

pub fn typecheck(ast: &Vec<AstNode>) -> Result<(), String> {
    let mut stack: Stack = Stack::new(&[]);
    let mut context = initial_context();

    for node in ast {
        match node {
            AstNode::Identifier(id) => match context.get(id.as_str()) {
                Some(ft) => {
                    let generics = GenericBindings::new();

                    (stack, _) = ft.apply(&stack, &generics)?;

                    println!("# {}\t {:?}", id, stack);
                }
                None => return Err("Identifier not in context".into()),
            },
            AstNode::Quote(id) => match context.get(id.as_str()) {
                Some(ft) => {
                    stack = stack.push(&Type::Func(ft.clone()));

                    println!("# {}\t {:?}", id, stack);
                }
                None => return Err("Identifier not in context".into()),
            },
            AstNode::Apply => match stack.split_last() {
                Some((Type::Func(ft), stack_rest)) => {
                    let generics = GenericBindings::new();

                    (stack, _) = ft.apply(&stack_rest, &generics)?;

                    println!("# {}\t {:?}", "!", stack);
                }
                _ => return Err("Top of stack is not a function".into()),
            },
            AstNode::Bind(id) => match stack.split_last() {
                Some((Type::Func(ft), stack_rest)) => {
                    context.insert(id.clone(), ft.clone());

                    stack = stack_rest;
                    println!("# ${}\t {:?}", id, stack);
                    println!("# bound: {:?}", context.get(id).unwrap());
                }
                _ => return Err("Cannot only bind a function".into()),
            },
        }
    }

    println!("# {:?}", context);

    Ok(())
}
