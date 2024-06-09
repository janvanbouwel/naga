use std::collections::HashMap;

use crate::ast::{AstNode, StackMod};

fn append(value: &str) -> String {
    std::format!("stack.append({value})")
}

fn indent(value: &str) -> String {
    value.replace('\n', "\n\t")
}

pub fn compile(ast: &Vec<AstNode>) -> Result<String, String> {
    let mut fd_stack: Vec<Vec<String>> = vec![];
    let mut code: Vec<String> = vec!["stack = []".into()];

    let mut store = builtins_code();

    let mut f_count = 0;

    for node in ast {
        match node {
            AstNode::S(stackmod) => match stackmod {
                StackMod::Arity(_, _) => {}
                StackMod::Apply => code.push("stack.pop()()".into()),
                StackMod::Id(id) => match store.get(id) {
                    Some(res) => code.push(res.to_string()),
                    None => return Err(std::format!("did not find identifier {id} in context")),
                },
                StackMod::Quote(id) => match store.get(id) {
                    Some(res) => code.push(std::format!(
                        "def quote():\n\t{}\n{}",
                        indent(res),
                        append("quote")
                    )),
                    None => return Err(std::format!("did not find identifier {id} in context")),
                },
                StackMod::Bind(id) => {
                    let f_name = std::format!("_{f_count}_{id}");
                    f_count += 1;
                    store.insert(id.clone(), std::format!("{f_name}()"));
                    code.push(std::format!("{f_name} = stack.pop()"))
                }
                StackMod::Int(int) => code.push(append(int)),
            },
            AstNode::OpenFunc => {
                code.push("def fn():".to_string());
                fd_stack.push(std::mem::take(&mut code));
            }
            AstNode::CloseFunc => {
                let res = indent(&std::mem::replace(&mut code, fd_stack.pop().unwrap()).join("\n"));
                code.push(std::format!("\t{res}"));
                code.push(append("fn"))
            }
        }
    }

    code.push("print(stack)".into());

    Ok(code.join("\n"))
}

fn builtins_code() -> HashMap<String, String> {
    let op = |f: &str| append(&std::format!("stack.pop(-2) {f} stack.pop()"));
    [
        ("'".to_string(), "pass".to_string()),
        (
            "?".to_string(),
            "[test, case_t, case_f] = stack[-3:]\nstack[-3:]=[case_t if test else case_f]"
                .to_string(),
        ),
        ("dup".to_string(), append("stack[-1]")),
        ("_".to_string(), "stack.pop()".to_string()),
        ("=".to_string(), op("==")),
        ("add".to_string(), op("+")),
        ("sub".to_string(), op("-")),
        ("mul".to_string(), op("*")),
    ]
    .into()
}

#[test]
fn test_builtins() {
    let builtins = crate::builtins::initial_context();
    let code = builtins_code();

    assert_eq!(code.len(), builtins.len());

    for id in builtins.keys() {
        assert!(code.contains_key(id.as_str()))
    }
}
