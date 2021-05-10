#![feature(try_blocks)]

pub use grammar::*;
pub mod grammar;


#[cfg(test)]
mod tests {
    use crate::{DesignLexer, DesignParserContextType, design_parser, DesignParserContext, DesignParser, DesignListener};
    use antlr_rust::InputStream;
    use antlr_rust::tree::ParseTreeListener;
    use antlr_rust::rule_context::CustomRuleContext;
    use antlr_rust::common_token_stream::CommonTokenStream;

    #[test]
    fn should_parse_config() {
        let data = r#"
config: 123
flow login {

}

"#;

        struct Listener {}

        impl<'input> ParseTreeListener<'input, DesignParserContextType> for Listener {
            fn enter_every_rule(&mut self, ctx: &dyn DesignParserContext<'input>) {
                println!(
                    "rule entered {}",
                    design_parser::ruleNames
                        .get(ctx.get_rule_index())
                        .unwrap_or(&"error")
                )
            }
        }
        impl<'input> DesignListener<'input> for Listener {}

        let lexer = DesignLexer::new(InputStream::new(&*data));
        let token_source = CommonTokenStream::new(lexer);
        let mut parser = DesignParser::new(token_source);
        parser.add_parse_listener(Box::new(Listener {}));
        let result = parser.start();
        println!("{:?}", result);
    }
}