use immutable_map::TreeMap;

use crate::{ast::AstNode, builtins::builtin_context};

type GenericBindings = TreeMap<usize, Type>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackT(Vec<Type>);

impl StackT {
    fn new(tys: &[Type]) -> Self {
        StackT(Vec::from(tys))
    }

    // fn push(&self, ty: &Type) -> StackT {
    //     let mut tys = self.0.clone();
    //     tys.push(ty.clone());
    //     StackT(tys)
    // }

    fn append(&self, other: &StackT, generics: GenericBindings) -> Result<StackT, String> {
        let mut tys = self.0.clone();
        for ty in &other.0 {
            match ty {
                Type::Generic(n) => match generics.get(n) {
                    None => return Err("generic wasn't bound".into()),
                    Some(bound_t) => tys.push(bound_t.clone()),
                },
                _ => tys.push(ty.clone()),
            }
        }
        Ok(StackT(tys))
    }

    fn split_last(&self) -> Option<(&Type, StackT)> {
        self.0
            .split_last()
            .map(|(ty, rest)| (ty, StackT::new(rest)))
    }

    fn take(
        &self,
        other: &StackT,
        generics: GenericBindings,
    ) -> Result<(StackT, GenericBindings), String> {
        match other.split_last() {
            None => Ok((self.clone(), generics)),
            Some((other_t, other_prev)) => match self.split_last() {
                None => Err("Cannot take from empty stack".into()),
                Some((stack_t, prev)) => match other_t {
                    Type::Generic(n) => match generics.get(n) {
                        None => prev.take(&other_prev, generics.insert(n.clone(), stack_t.clone())),
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
pub struct FunctionT {
    from: StackT,
    to: StackT,
}

impl FunctionT {
    pub fn new(from: &[Type], to: &[Type]) -> Self {
        FunctionT {
            from: StackT::new(from),
            to: StackT::new(to),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Boolean,
    Stack(StackT),
    Function(FunctionT),
    Generic(usize),
}

pub fn typecheck(ast: &Vec<AstNode>) -> Result<(), String> {
    let mut stack: StackT = StackT::new(&[]);
    let context = builtin_context();

    for node in ast {
        match node {
            AstNode::Identifier(id) => match context.get(id.as_str()) {
                Some(ftl) => {
                    let ft = ftl();
                    let mut generics = GenericBindings::new();
                    (stack, generics) = stack.take(&ft.from, generics)?;
                    stack = stack.append(&ft.to, generics)?.clone();

                    println!("# {}   {:?}", id, stack);
                }
                None => return Err("Identifier not in context".into()),
            },
        }
    }

    Ok(())
}
