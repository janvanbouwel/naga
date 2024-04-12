use std::collections::HashMap;

use crate::ast::AstNode;

pub fn compile(ast: &Vec<AstNode>) -> Result<String, ()> {
    let mut code: Vec<String> = vec!["stack = []".into()];

    let mut context = builtins_code();

    for node in ast {
        match node {
            AstNode::Apply => code.push("_ = stack.pop(); _()".into()),
            AstNode::Id(id) => match context.get(id) {
                Some(res) => code.push(res.to_string()),
                None => return Err(()),
            },
            AstNode::Quote(id) => match context.get(id) {
                Some(res) => code.push(std::format!("stack.append(lambda: {})", res)),
                None => return Err(()),
            },
            AstNode::Bind(id) => {
                context.insert(id.clone(), std::format!("{}()", id));
                code.push(std::format!("{} = stack.pop()", id))
            }
        }
    }

    code.push("print(stack)".into());

    Ok(code.join("\n"))
}

fn builtins_code() -> HashMap<String, String> {
    [
        ("True".to_string(), "stack.append(True)".to_string()),
        ("False".to_string(), "stack.append(False)".to_string()),
        (
            "and".to_string(),
            "stack.append(stack.pop() and stack.pop())".to_string(),
        ),
        ("id".to_string(), "".to_string()),
        ("dup".to_string(), "stack.append(stack[-1])".to_string()),
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
