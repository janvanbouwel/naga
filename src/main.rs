

use tree_sitter;

fn main() {
    let code = "# hello   
    (1 2 +) $b () (   )
    a ' 'abc aa '$a 1 $b #abmqklsdjf
    TRUE id FALSE ? 5.0";
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(tree_sitter_lang::language())
        .expect("Error loading lang grammar");
    let tree = parser.parse(code, None).unwrap();
    println!("{}", tree.root_node().to_sexp())
}
