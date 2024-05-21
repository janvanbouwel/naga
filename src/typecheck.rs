use crate::{
    ast::AstNode,
    builtins::initial_context,
    stacklike::{Context, ExecContext},
};

pub fn typecheck(ast: &Vec<AstNode>) -> Result<ExecContext, String> {
    let mut ctx = ExecContext::new(initial_context());

    for node in ast {
        println!("\n# {node:?}");

        match ctx.check_node(node)? {
            either::Either::Left(new_ctx) => ctx = new_ctx,
            either::Either::Right(_) => unreachable!(),
        };
    }

    Ok(ctx)
}

// #[test]
// fn test_bool_and() {
//     use StackMod::*;

//     let ast: Vec<AstNode> = vec![
//         Id("True".to_string()).into(),
//         Id("True".to_string()).into(),
//         Id("and".to_string()).into(),
//     ];
//     let ctx = typecheck(&ast).unwrap();
//     assert_eq!(ctx.stack, Stack::new(&[Ty::Bool]))
// }

// #[test]
// fn test_bool_not() {
//     use StackMod::*;

//     let ast: Vec<AstNode> = vec![Id("True".to_string()).into(), Id("not".to_string()).into()];
//     let stack = typecheck(&ast).unwrap().stack;
//     assert_eq!(stack, Stack::new(&[Ty::Bool]))
// }

// #[test]
// fn test_funcdef() {
//     let mut fd = FuncDef::new();
//     let and = initial_context().get("and").unwrap()();
//     let id = initial_context().get("id").unwrap()();

//     fd.apply(id).unwrap();
//     fd.apply(and).unwrap();

//     let f = fd.collect();
//     println!("fd {fd:?}");

//     println!("{:?}", f);

//     let mut stack = Stack::new(&[Ty::Bool, Ty::Bool]);
//     stack.apply(f).unwrap();
//     assert_eq!(stack, Stack::new(&[Ty::Bool]))
// }
