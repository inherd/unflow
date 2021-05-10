use std::collections::HashMap;

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::InputStream;
use antlr_rust::tree::ParseTreeListener;

use crate::{DesignLexer, DesignParserContextType, design_parser, DesignParserContext, DesignParser, DesignListener};

pub struct UnflowParser {
    pub config: HashMap<String, String>,
}

impl UnflowParser {
    pub fn parse(data: &str) {
        let lexer = DesignLexer::new(InputStream::new(&*data));
        let token_source = CommonTokenStream::new(lexer);
        let mut parser = DesignParser::new(token_source);
        let unflow = UnflowParser { config: Default::default() };
        parser.add_parse_listener(Box::new(unflow));
        let _ = parser.start();
    }
}


impl<'input> ParseTreeListener<'input, DesignParserContextType> for UnflowParser {
    fn enter_every_rule(&mut self, ctx: &dyn DesignParserContext<'input>) {
        println!(
            "rule entered {}",
            design_parser::ruleNames
                .get(ctx.get_rule_index())
                .unwrap_or(&"error")
        )
    }
}

impl<'input> DesignListener<'input> for UnflowParser {}
