use std::collections::HashMap;

use crate::ast::{AstNode, StackMod};

fn append(value: &str) -> String {
    std::format!("stack.append({value})")
}

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
                    Some(res) => code.push(append(&std::format!("lambda: [{res}]"))),
                    None => return Err("did not find identifier in context"),
                },
                StackMod::Bind(id) => {
                    store.insert(id.clone(), std::format!("__{id}()"));
                    code.push(std::format!("__{} = stack.pop()", id))
                }
                StackMod::Int(int) => code.push(append(int)),
            },
            AstNode::OpenFunc => fd_stack.push(std::mem::replace(&mut code, vec![])),
            AstNode::CloseFunc => {
                let res = std::mem::replace(&mut code, fd_stack.pop().unwrap())
                    .join(",\n")
                    .replace("\n", "\n\t");
                code.push(append(&std::format!("lambda: [\n\t{res}\n]")))
            }
        }
    }

    code.push("print(stack)".into());

    Ok(code.join("\n"))
}

fn builtins_code() -> HashMap<String, String> {
    let op = |f: &str| append(&std::format!("stack.pop(-2) {f} stack.pop()"));
    [
        ("'".to_string(), "".to_string()),
        ("id".to_string(), "".to_string()),
        (
            "?".to_string(),
            "stack.pop() if stack.pop(-3) else stack.pop(-2)".to_string(),
        ),
        ("True".to_string(), append("True")),
        ("False".to_string(), append("False")),
        ("and".to_string(), op("and")),
        ("not".to_string(), append("not stack.pop()")),
        ("dup".to_string(), append("stack[-1]")),
        ("drop".to_string(), "stack.pop()".to_string()),
        ("eq".to_string(), op("==")),
        ("add".to_string(), op("+")),
        ("sub".to_string(), op("-")),
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
