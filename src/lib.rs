#![feature(try_blocks)]

pub use grammar::*;
pub mod grammar;

pub mod unflow_parser;

#[cfg(test)]
mod tests {
    use crate::unflow_parser::UnflowParser;

    #[test]
    fn should_parse_config() {
        let data = r#"project: DesignDSL
feature: "design basic dsl"
type: web
width: 1080px

"#;

        UnflowParser::parse(data);
    }
}