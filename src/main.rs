extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "ident.pest"]
pub struct IdentParser;

fn main() {
    let pairs = IdentParser::parse(Rule::start, "id: 123").unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        for inner_pair in pair.into_inner() {
            println!("{:?}", inner_pair.as_rule());
        }
    }
}