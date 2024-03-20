use crate::{ast::AstNode, builtins::builtin_context};

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

    fn append(&self, other: &StackT) -> StackT {
        let mut tys = self.0.clone();
        tys.extend_from_slice(&other.0);
        StackT(tys)
    }

    fn split_last(&self) -> Option<(&Type, StackT)> {
        self.0
            .split_last()
            .map(|(ty, rest)| (ty, StackT::new(rest)))
    }

    fn take(self, other: &StackT) -> Result<StackT, String> {
        match other.split_last() {
            None => Ok(self),
            Some((other_t, other_prev)) => match self.split_last() {
                None => Err("Cannot take from empty stack".into()),
                Some((t, prev)) => {
                    if t != other_t {
                        return Err("Types did not match".into());
                    };
                    prev.take(&other_prev)
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
    let mut stack: StackT = StackT::new(&[]);
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
