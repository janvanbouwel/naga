

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "naga.pest"]
pub struct NagaParser;

enum StackT {
    Empty,
    Cons { t: Box<Type>, next: Box<StackT> },
}

struct FunctionT {
    from: StackT,
    to: StackT,
}

enum Type {
    Boolean,
    Integer,
    Stack(StackT),
    Function(FunctionT),
}

struct Function<'a> {
    code: &'a str,
    ty: FunctionT
}



fn parse(source: &str) -> Result<Vec<&str>, &str> {
    let mut code = vec!("stack = []");
    
    let pairs = NagaParser::parse(Rule::program, source).unwrap();
    for pair in pairs {
        match pair.as_rule() {
            Rule::tru => code.push("stack.append(True)"),
            Rule::fals => code.push("stack.append(False)"),
            Rule::identifier => match pair.as_str() {
                "and" => code.push("_ = stack.pop(); stack[-1] &= _"),
                _ => return Err(""),
            },
            Rule::EOI => (),
            _ => println!("{}", pair),
        }
    }

    code.push("print(stack)");
    Ok(code)
}

fn main() {
    let source = "# hello   
    True False and";
    
    let code = parse(source).expect("");

    println!("{}", code.join("\n"));
}
