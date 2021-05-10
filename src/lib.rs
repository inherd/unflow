extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "ident.pest"]
pub struct IdentParser;

fn parse(str: &str) {
    let pairs = IdentParser::parse(Rule::start, str).unwrap_or_else(|e| panic!("{}", e));

    println!("{:?}", pairs);
    for pair in pairs {
        for inner_pair in pair.into_inner() {
            println!("{:?}", inner_pair.as_rule());
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::parse;

    #[test]
    fn should_parse_config() {
        parse("
id: 123
length: 123px
width: 999
keywords: hello, worlds
");
    }

    #[test]
    fn should_parse_flow() {
        parse("flow login { }");
    }

    #[test]
    fn should_parse_utf8_flow() {
        parse("flow 登录 {
  see LoginForm
}");
    }
}