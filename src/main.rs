use nagalang::{ast::parse, compile::compile, typecheck::typecheck};

fn main() {
    let source = "# hello  
    ( eq not ) $xor
    True True xor
    True False xor
    False True xor
    False False xor
    ( True ) ! 
    True 
    'dup
    1->2 
    ! and 'and $new_and True new_and";

    let ast = parse(source).expect("failed to parse");
    typecheck(&ast).expect("Typechecking failed");
    let code = compile(&ast).expect("failed to compile");

    println!("{}", code);
}
