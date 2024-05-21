use std::collections::HashMap;

use crate::ast::{AstNode, StackMod};

pub fn compile(ast: &Vec<AstNode>) -> Result<String, &str> {
    let mut fd_stack: Vec<Vec<String>> = vec![];
    let mut code: Vec<String> = vec!["stack = []".into()];

    let mut store = builtins_code();

    for node in ast {
        match node {
            AstNode::S(stackmod) => match stackmod {
                StackMod::Arity(_, _) => {}
                StackMod::Apply => code.push("stack.pop()()".into()),
                StackMod::Id(id) => match store.get(id) {
                    Some(res) => code.push(res.to_string()),
                    None => return Err("did not find identifier in context"),
                },
                StackMod::Quote(id) => match store.get(id) {
                    Some(res) => code.push(std::format!("stack.append(lambda: [{}])", res)),
                    None => return Err("did not find identifier in context"),
                },
                StackMod::Bind(id) => {
                    store.insert(id.clone(), std::format!("__{id}()"));
                    code.push(std::format!("__{} = stack.pop()", id))
                }
            },
            AstNode::OpenFunc => fd_stack.push(std::mem::replace(&mut code, vec![])),
            AstNode::CloseFunc => {
                let res = std::mem::replace(&mut code, fd_stack.pop().unwrap())
                    .join(",\n")
                    .replace("\n", "\n\t");
                code.push(std::format!("stack.append(lambda: [\n\t{res}\n])"))
            }
        }
    }

    code.push("print(stack)".into());

    Ok(code.join("\n"))
}

fn builtins_code() -> HashMap<String, String> {
    [
        ("'".to_string(), "".to_string()),
        ("id".to_string(), "".to_string()),
        (
            "?".to_string(),
            "stack.pop() if stack.pop(-3) else stack.pop(-2)".to_string(),
        ),
        ("True".to_string(), "stack.append(True)".to_string()),
        ("False".to_string(), "stack.append(False)".to_string()),
        (
            "and".to_string(),
            "stack.append(stack.pop() and stack.pop())".to_string(),
        ),
        (
            "not".to_string(),
            "stack.append(not stack.pop())".to_string(),
        ),
        ("dup".to_string(), "stack.append(stack[-1])".to_string()),
        ("drop".to_string(), "stack.pop()".to_string()),
        (
            "eq".to_string(),
            "stack.append(stack.pop() == stack.pop())".to_string(),
        ),
    ]
    .into()
}

#[test]
fn test_builtins() {
    let builtins = crate::builtins::initial_context();
    let code = builtins_code();

    for id in builtins.keys() {
        assert!(code.contains_key(id.as_str()))
    }
}
