use nagalang::{ast::parse, compile::compile, typecheck::typecheck};

fn main() {
    let source = "# hello  
    ( ( not ) ( id ) test ! ) $xor
    ( xor not ) $xnor
    True dup xor
    True dup xnor";

    let ast = parse(source).expect("failed to parse");
    typecheck(&ast).expect("Typechecking failed");
    let code = compile(&ast).expect("failed to compile");

    println!("{}", code);
}
