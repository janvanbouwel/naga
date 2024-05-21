use pest::{error::Error, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "naga.pest"]
pub struct NagaParser;

#[derive(Debug)]
pub enum AstNode {
    S(StackMod),
    OpenFunc,
    CloseFunc,
}

#[derive(Debug)]
pub enum StackMod {
    Id(String),
    Arity(u32, u32),
    Quote(String),
    Bind(String),
    Apply,
}

impl From<StackMod> for AstNode {
    fn from(value: StackMod) -> Self {
        AstNode::S(value)
    }
}

pub fn parse(source: &str) -> Result<Vec<AstNode>, Box<Error<Rule>>> {
    let mut ast: Vec<AstNode> = vec![];

    let pairs = NagaParser::parse(Rule::program, source)?;

    for pair in pairs {
        match pair.as_rule() {
            Rule::arity => {
                let mut inner = pair.into_inner();
                let from: u32 = inner.next().unwrap().as_str().parse().unwrap();
                let to: u32 = inner.next().unwrap().as_str().parse().unwrap();
                ast.push(StackMod::Arity(from, to).into());
            }
            Rule::open_func => ast.push(AstNode::OpenFunc),
            Rule::close_func => ast.push(AstNode::CloseFunc),
            Rule::apply => ast.push(StackMod::Apply.into()),
            Rule::quote => match pair.into_inner().next() {
                None => ast.push(StackMod::Quote("'".into()).into()),
                Some(pair) => ast.push(StackMod::Quote(pair.as_str().into()).into()),
            },
            Rule::bind => {
                ast.push(StackMod::Bind(pair.into_inner().next().unwrap().as_str().into()).into())
            }
            Rule::identifier => ast.push(StackMod::Id(pair.as_str().into()).into()),
            Rule::EOI => {}
            _ => unreachable!(),
        }
    }

    Ok(ast)
}
