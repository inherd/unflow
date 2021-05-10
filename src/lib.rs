#![feature(try_blocks)]

pub use grammar::*;
pub mod grammar;

pub mod unflow_parser;

#[cfg(test)]
mod tests {
    use crate::unflow_parser::UnflowParser;

    #[test]
    fn should_parse_config() {
        let data = r#"
config: 123
flow login {

}

"#;

        UnflowParser::parse(data);
    }
}