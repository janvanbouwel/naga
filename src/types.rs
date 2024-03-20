use crate::{ast::AstNode, builtins::builtin_context};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StackT {
    Empty,
    Cons { ty: Box<Type>, prev: Box<StackT> },
}

impl StackT {
    fn new(tys: &[Type]) -> Self {
        let mut stack = StackT::Empty;

        for ty in tys.iter().rev() {
            stack = stack.push(ty);
        }
        stack
    }

    fn push(self, ty: &Type) -> StackT {
        StackT::Cons {
            ty: Box::new(ty.clone()),
            prev: Box::new(self),
        }
    }

    fn append(self, other: &StackT) -> StackT {
        match other {
            StackT::Empty => self,
            StackT::Cons { ty, prev } => self.append(prev).push(ty),
        }
    }

    fn take(self, other: &StackT) -> Result<StackT, String> {
        match other {
            StackT::Empty => Ok(self),
            StackT::Cons {
                ty: other_t,
                prev: other_prev,
            } => match self {
                StackT::Empty => Err("Cannot take from empty stack".into()),
                StackT::Cons { ty: t, prev } => {
                    if t != *other_t {
                        return Err("Types did not match".into());
                    };
                    prev.take(other_prev)
                }
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
}

pub fn typecheck(ast: &Vec<AstNode>) -> Result<(), String> {
    let mut stack: StackT = StackT::Empty;
    let context = builtin_context();

    for node in ast {
        match node {
            AstNode::Identifier(id) => match context.get(id.as_str()) {
                Some(ftl) => {
                    let ft = ftl();
                    stack = stack.take(&ft.from)?.append(&ft.to).clone();
                }
                None => return Err("Identifier not in context".into()),
            },
        }
        println!("# {:?}", stack);
    }

    Ok(())
}
