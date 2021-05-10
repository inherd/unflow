use std::collections::HashMap;

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::InputStream;
use antlr_rust::token_factory::ArenaCommonFactory;
use antlr_rust::tree::{ParseTree, ParseTreeVisitor, Visitable};
use serde::{Deserialize, Serialize};

use crate::{Config_declContext, DesignLexer, DesignParser, DesignParserContextType, DesignVisitor};
use crate::Config_declContextAttrs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Unflow {
    pub config: HashMap<String, String>
}

impl Default for Unflow {
    fn default() -> Self {
        Unflow {
            config: Default::default()
        }
    }
}

pub struct UnflowParser<'i> {
    #[allow(dead_code)]
    inputs: Vec<&'i str>,
    pub(crate) flow: Unflow,
}

pub fn parse<'input>(data: &str) -> Unflow {
    let tf = ArenaCommonFactory::default();
    let lexer = DesignLexer::new_with_token_factory(InputStream::new(data.into()), &tf);
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = DesignParser::new(token_source);
    let result = parser.start().expect("parsed unsuccessfully");

    let mut unflow = UnflowParser {
        inputs: vec![],
        flow: Default::default()
    };

    result.accept(&mut unflow);

    unflow.flow
}

impl<'i> ParseTreeVisitor<'i, DesignParserContextType> for UnflowParser<'i> {}

impl<'i> DesignVisitor<'i> for UnflowParser<'i> {
    fn visit_config_decl(&mut self, ctx: &Config_declContext<'i>) {
        self.flow.config.insert(
            ctx.config_key().unwrap().get_text(),
            ctx.config_value().unwrap().get_text(),
        );
    }
}