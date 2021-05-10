use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::InputStream;
use antlr_rust::tree::{Visitable, ParseTree, ParseTreeVisitor};

use crate::{DesignLexer, DesignParserContextType, DesignParser, Config_declContext, DesignVisitor};
use antlr_rust::token_factory::ArenaCommonFactory;

pub struct UnflowParser<'i, T>(pub(crate) Vec<&'i str>, pub(crate) T);

pub fn parse<'input>(data: &str) {
    let tf = ArenaCommonFactory::default();
    let lexer = DesignLexer::new_with_token_factory(InputStream::new(data.into()), &tf);
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = DesignParser::new(token_source);
    let result = parser.start().expect("parsed unsuccessfully");

    let mut test = 5;
    let mut unflow = UnflowParser(vec![], &mut test);
    result.accept(&mut unflow);
}

impl<'i, T> ParseTreeVisitor<'i, DesignParserContextType> for UnflowParser<'i, T> {}

impl<'i, T> DesignVisitor<'i> for UnflowParser<'i, T> {
    fn visit_config_decl(&mut self, ctx: &Config_declContext<'i>) {
        println!("{:?}", ctx.get_text());
    }
}