use std::collections::HashMap;

use crate::ast::AstNode;

pub fn compile(ast: &Vec<AstNode>) -> Result<String, ()> {
    let mut code: Vec<String> = vec!["stack = []".into()];

    let context = builtins_code();

    for node in ast {
        match node {
            AstNode::Apply => code.push("_ = stack.pop(); _()".into()),
            AstNode::Identifier(id) => match context.get(id.as_str()) {
                Some(res) => code.push(res.to_string()),
                None => return Err(()),
            },
            AstNode::Quote(id) => match context.get(id.as_str()) {
                Some(res) => code.push(std::format!("stack.append(lambda: {})", res)),
                None => return Err(()),
            },
        }
    }

    code.push("print(stack)".into());

    Ok(code.join("\n"))
}

fn builtins_code() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("True", "stack.append(True)"),
        ("False", "stack.append(False)"),
        ("and", "_ = stack.pop(); stack[-1] &= _"),
        ("id", ""),
        ("dup", "stack.append(stack[-1])"),
    ])
}

#[test]
fn test_builtins() {
    let builtins = crate::builtins::builtin_context();
    let code = builtins_code();

    for id in builtins.keys() {
        assert!(code.contains_key(id))
    }
}
