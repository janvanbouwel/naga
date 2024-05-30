use nagalang::{ast::parse, compile::compile, typecheck::typecheck};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let source = match args.get(1) {
        Some(arg) => std::fs::read_to_string(arg).expect("Failed reading source file"),
        None => " 
        0 1 ="
            .to_string(),
    };

    let ast = parse(&source).expect("failed to parse");
    typecheck(&ast).expect("Typechecking failed");
    let code = compile(&ast).expect("failed to compile");

    std::fs::write("out.py", &code).expect("writing failed");

    println!("{}", code);
}
