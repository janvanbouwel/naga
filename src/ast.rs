use pest::{error::Error, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "naga.pest"]
pub struct NagaParser;

pub enum AstNode {
    Identifier(String),
}

pub fn parse(source: &str) -> Result<Vec<AstNode>, Box<Error<Rule>>> {
    let mut ast = vec![];

    let pairs = NagaParser::parse(Rule::program, source)?;

    for pair in pairs {
        match pair.as_rule() {
            Rule::identifier => ast.push(AstNode::Identifier(pair.as_str().into())),
            Rule::EOI => {}
            _ => unreachable!(),
        }
    }

    Ok(ast)
}
