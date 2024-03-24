use pest::{error::Error, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "naga.pest"]
pub struct NagaParser;

pub enum AstNode {
    Identifier(String),
    Quote(String),
    Bind(String),
    Apply,
}

pub fn parse(source: &str) -> Result<Vec<AstNode>, Box<Error<Rule>>> {
    let mut ast = vec![];

    let pairs = NagaParser::parse(Rule::program, source)?;

    for pair in pairs {
        match pair.as_rule() {
            Rule::apply => ast.push(AstNode::Apply),
            Rule::quote => match pair.into_inner().next() {
                None => ast.push(AstNode::Quote("id".into())),
                Some(pair) => ast.push(AstNode::Quote(pair.as_str().into())),
            },
            Rule::bind => ast.push(AstNode::Bind(
                pair.into_inner().next().unwrap().as_str().into(),
            )),
            Rule::identifier => ast.push(AstNode::Identifier(pair.as_str().into())),
            Rule::EOI => {}
            _ => unreachable!(),
        }
    }

    Ok(ast)
}
