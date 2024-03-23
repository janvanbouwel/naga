use nagalang::{ast::parse, compile::compile, types::typecheck};

fn main() {
    let source = "# hello   
    True 'dup ! False and and";

    let ast = parse(source).expect("failed to parse");
    typecheck(&ast).expect("Typechecking failed");
    let code = compile(&ast).expect("failed to compile");

    println!("{}", code);
}
