// Generated from src/grammar/Design.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::designlistener::*;
use super::designvisitor::*;

use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const T__0:isize=1; 
		pub const T__1:isize=2; 
		pub const T__2:isize=3; 
		pub const T__3:isize=4; 
		pub const T__4:isize=5; 
		pub const T__5:isize=6; 
		pub const T__6:isize=7; 
		pub const GOTO_KEY:isize=8; 
		pub const SHOW_KEY:isize=9; 
		pub const FLOW:isize=10; 
		pub const SEE:isize=11; 
		pub const DO:isize=12; 
		pub const REACT:isize=13; 
		pub const WITHTEXT:isize=14; 
		pub const ANIMATE:isize=15; 
		pub const REPEAT:isize=16; 
		pub const REPEAT_TIMES:isize=17; 
		pub const LAYOUT:isize=18; 
		pub const POSITION:isize=19; 
		pub const PAGE:isize=20; 
		pub const COMPONENT:isize=21; 
		pub const STYLE:isize=22; 
		pub const LIBRARY:isize=23; 
		pub const STRING_LITERAL:isize=24; 
		pub const WS:isize=25; 
		pub const COMMENT:isize=26; 
		pub const LINE_COMMENT:isize=27; 
		pub const EmptyLine:isize=28; 
		pub const Space:isize=29; 
		pub const NewLine:isize=30; 
		pub const LPAREN:isize=31; 
		pub const RPAREN:isize=32; 
		pub const LBRACE:isize=33; 
		pub const RBRACE:isize=34; 
		pub const LBRACK:isize=35; 
		pub const RBRACK:isize=36; 
		pub const Quote:isize=37; 
		pub const SingleQuote:isize=38; 
		pub const COLON:isize=39; 
		pub const DOT:isize=40; 
		pub const COMMA:isize=41; 
		pub const LETTER:isize=42; 
		pub const IDENTIFIER:isize=43; 
		pub const DIGITS:isize=44; 
		pub const DIGITS_IDENTIFIER:isize=45; 
		pub const DECIMAL_LITERAL:isize=46; 
		pub const FLOAT_LITERAL:isize=47;
	pub const RULE_start:usize = 0; 
	pub const RULE_comment:usize = 1; 
	pub const RULE_config_decl:usize = 2; 
	pub const RULE_config_key:usize = 3; 
	pub const RULE_config_value:usize = 4; 
	pub const RULE_unit:usize = 5; 
	pub const RULE_declarations:usize = 6; 
	pub const RULE_flow_decl:usize = 7; 
	pub const RULE_interaction_decl:usize = 8; 
	pub const RULE_see_decl:usize = 9; 
	pub const RULE_do_decl:usize = 10; 
	pub const RULE_react_decl:usize = 11; 
	pub const RULE_animate_decl:usize = 12; 
	pub const RULE_react_action:usize = 13; 
	pub const RULE_goto_action:usize = 14; 
	pub const RULE_show_action:usize = 15; 
	pub const RULE_action_name:usize = 16; 
	pub const RULE_component_value:usize = 17; 
	pub const RULE_component_name:usize = 18; 
	pub const RULE_scene_name:usize = 19; 
	pub const RULE_animate_name:usize = 20; 
	pub const RULE_page_decl:usize = 21; 
	pub const RULE_component_decl:usize = 22; 
	pub const RULE_component_body_decl:usize = 23; 
	pub const RULE_layout_decl:usize = 24; 
	pub const RULE_layout_row:usize = 25; 
	pub const RULE_layout_lines:usize = 26; 
	pub const RULE_layout_line:usize = 27; 
	pub const RULE_component_use_decl:usize = 28; 
	pub const RULE_component_layout_value:usize = 29; 
	pub const RULE_style_decl:usize = 30; 
	pub const RULE_style_name:usize = 31; 
	pub const RULE_style_body:usize = 32; 
	pub const RULE_library_decl:usize = 33; 
	pub const RULE_library_exp:usize = 34; 
	pub const RULE_keyValue:usize = 35; 
	pub const RULE_preset_key:usize = 36; 
	pub const RULE_preset_value:usize = 37; 
	pub const RULE_preset_array:usize = 38; 
	pub const RULE_preset_call:usize = 39; 
	pub const RULE_library_name:usize = 40;
	pub const ruleNames: [&'static str; 41] =  [
		"start", "comment", "config_decl", "config_key", "config_value", "unit", 
		"declarations", "flow_decl", "interaction_decl", "see_decl", "do_decl", 
		"react_decl", "animate_decl", "react_action", "goto_action", "show_action", 
		"action_name", "component_value", "component_name", "scene_name", "animate_name", 
		"page_decl", "component_decl", "component_body_decl", "layout_decl", "layout_row", 
		"layout_lines", "layout_line", "component_use_decl", "component_layout_value", 
		"style_decl", "style_name", "style_body", "library_decl", "library_exp", 
		"keyValue", "preset_key", "preset_value", "preset_array", "preset_call", 
		"library_name"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;42] = [
		None, Some("'rem'"), Some("'px'"), Some("'em'"), Some("'-'"), Some("'|'"), 
		Some("';'"), Some("'='"), None, None, None, None, None, None, None, None, 
		Some("'repeat'"), None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, Some("'('"), Some("')'"), Some("'{'"), Some("'}'"), 
		Some("'['"), Some("']'"), Some("'\"'"), Some("'''"), Some("':'"), Some("'.'"), 
		Some("','")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;48]  = [
		None, None, None, None, None, None, None, None, Some("GOTO_KEY"), Some("SHOW_KEY"), 
		Some("FLOW"), Some("SEE"), Some("DO"), Some("REACT"), Some("WITHTEXT"), 
		Some("ANIMATE"), Some("REPEAT"), Some("REPEAT_TIMES"), Some("LAYOUT"), 
		Some("POSITION"), Some("PAGE"), Some("COMPONENT"), Some("STYLE"), Some("LIBRARY"), 
		Some("STRING_LITERAL"), Some("WS"), Some("COMMENT"), Some("LINE_COMMENT"), 
		Some("EmptyLine"), Some("Space"), Some("NewLine"), Some("LPAREN"), Some("RPAREN"), 
		Some("LBRACE"), Some("RBRACE"), Some("LBRACK"), Some("RBRACK"), Some("Quote"), 
		Some("SingleQuote"), Some("COLON"), Some("DOT"), Some("COMMA"), Some("LETTER"), 
		Some("IDENTIFIER"), Some("DIGITS"), Some("DIGITS_IDENTIFIER"), Some("DECIMAL_LITERAL"), 
		Some("FLOAT_LITERAL")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,DesignParserExt, I, DesignParserContextType , dyn DesignListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;

  pub type LocalTokenFactory<'input> = antlr_rust::token_factory::ArenaCommonFactory<'input>;

pub type DesignTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, DesignParserContextType , dyn DesignListener<'input> + 'a>;

/// Parser for Design grammar
pub struct DesignParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","2");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				DesignParserExt{
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> DesignParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> DesignParser<'input, I, DefaultErrorStrategy<'input,DesignParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for DesignParser
pub trait DesignParserContext<'input>:
	for<'x> Listenable<dyn DesignListener<'input> + 'x > + 
	for<'x> Visitable<dyn DesignVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=DesignParserContextType>
{}

impl<'input, 'x, T> VisitableDyn<T> for dyn DesignParserContext<'input> + 'input
where
    T: DesignVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn DesignVisitor<'input> + 'x))
    }
}

impl<'input> DesignParserContext<'input> for TerminalNode<'input,DesignParserContextType> {}
impl<'input> DesignParserContext<'input> for ErrorNode<'input,DesignParserContextType> {}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn DesignParserContext<'input> + 'input{}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn DesignListener<'input> + 'input{}

pub struct DesignParserContextType;
antlr_rust::type_id!{DesignParserContextType}

impl<'input> ParserNodeType<'input> for DesignParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn DesignParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct DesignParserExt{
}

impl DesignParserExt{
}


impl<'input> TokenAware<'input> for DesignParserExt{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for DesignParserExt{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for DesignParserExt{
	fn get_grammar_file_name(&self) -> & str{ "Design.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}
//------------------- start ----------------
pub type StartContextAll<'input> = StartContext<'input>;


pub type StartContext<'input> = BaseParserRuleContext<'input,StartContextExt<'input>>;

#[derive(Clone)]
pub struct StartContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for StartContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for StartContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_start(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_start(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for StartContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_start(self);
	}
}

impl<'input> CustomRuleContext<'input> for StartContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start }
}
antlr_rust::type_id!{StartContextExt<'a>}

impl<'input> StartContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StartContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StartContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StartContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<StartContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn comment_all(&self) ->  Vec<Rc<CommentContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn comment(&self, i: usize) -> Option<Rc<CommentContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn declarations_all(&self) ->  Vec<Rc<DeclarationsContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn declarations(&self, i: usize) -> Option<Rc<DeclarationsContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> StartContextAttrs<'input> for StartContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start(&mut self,)
	-> Result<Rc<StartContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StartContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_start);
        let mut _localctx: Rc<StartContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(86);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << FLOW) | (1usize << LAYOUT) | (1usize << PAGE) | (1usize << COMPONENT) | (1usize << STYLE) | (1usize << LIBRARY) | (1usize << IDENTIFIER))) != 0) {
				{
				recog.base.set_state(84);
				recog.err_handler.sync(&mut recog.base)?;
				match  recog.interpreter.adaptive_predict(0,&mut recog.base)? {
					1 =>{
						{
						/*InvokeRule comment*/
						recog.base.set_state(82);
						recog.comment()?;

						}
					}
				,
					2 =>{
						{
						/*InvokeRule declarations*/
						recog.base.set_state(83);
						recog.declarations()?;

						}
					}

					_ => {}
				}
				}
				recog.base.set_state(88);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(89);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- comment ----------------
pub type CommentContextAll<'input> = CommentContext<'input>;


pub type CommentContext<'input> = BaseParserRuleContext<'input,CommentContextExt<'input>>;

#[derive(Clone)]
pub struct CommentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for CommentContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for CommentContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_comment(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_comment(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for CommentContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_comment(self);
	}
}

impl<'input> CustomRuleContext<'input> for CommentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_comment }
	//fn type_rule_index() -> usize where Self: Sized { RULE_comment }
}
antlr_rust::type_id!{CommentContextExt<'a>}

impl<'input> CommentContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CommentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CommentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CommentContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<CommentContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> CommentContextAttrs<'input> for CommentContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn comment(&mut self,)
	-> Result<Rc<CommentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CommentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_comment);
        let mut _localctx: Rc<CommentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(91);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- config_decl ----------------
pub type Config_declContextAll<'input> = Config_declContext<'input>;


pub type Config_declContext<'input> = BaseParserRuleContext<'input,Config_declContextExt<'input>>;

#[derive(Clone)]
pub struct Config_declContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Config_declContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Config_declContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_config_decl(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_config_decl(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Config_declContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_config_decl(self);
	}
}

impl<'input> CustomRuleContext<'input> for Config_declContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_config_decl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_config_decl }
}
antlr_rust::type_id!{Config_declContextExt<'a>}

impl<'input> Config_declContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Config_declContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Config_declContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Config_declContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Config_declContextExt<'input>>{

fn config_key(&self) -> Option<Rc<Config_keyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn config_value(&self) -> Option<Rc<Config_valueContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Config_declContextAttrs<'input> for Config_declContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn config_decl(&mut self,)
	-> Result<Rc<Config_declContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Config_declContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_config_decl);
        let mut _localctx: Rc<Config_declContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule config_key*/
			recog.base.set_state(93);
			recog.config_key()?;

			recog.base.set_state(94);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule config_value*/
			recog.base.set_state(95);
			recog.config_value()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- config_key ----------------
pub type Config_keyContextAll<'input> = Config_keyContext<'input>;


pub type Config_keyContext<'input> = BaseParserRuleContext<'input,Config_keyContextExt<'input>>;

#[derive(Clone)]
pub struct Config_keyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Config_keyContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Config_keyContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_config_key(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_config_key(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Config_keyContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_config_key(self);
	}
}

impl<'input> CustomRuleContext<'input> for Config_keyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_config_key }
	//fn type_rule_index() -> usize where Self: Sized { RULE_config_key }
}
antlr_rust::type_id!{Config_keyContextExt<'a>}

impl<'input> Config_keyContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Config_keyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Config_keyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Config_keyContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Config_keyContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> Config_keyContextAttrs<'input> for Config_keyContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn config_key(&mut self,)
	-> Result<Rc<Config_keyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Config_keyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_config_key);
        let mut _localctx: Rc<Config_keyContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(97);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- config_value ----------------
pub type Config_valueContextAll<'input> = Config_valueContext<'input>;


pub type Config_valueContext<'input> = BaseParserRuleContext<'input,Config_valueContextExt<'input>>;

#[derive(Clone)]
pub struct Config_valueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Config_valueContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Config_valueContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_config_value(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_config_value(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Config_valueContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_config_value(self);
	}
}

impl<'input> CustomRuleContext<'input> for Config_valueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_config_value }
	//fn type_rule_index() -> usize where Self: Sized { RULE_config_value }
}
antlr_rust::type_id!{Config_valueContextExt<'a>}

impl<'input> Config_valueContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Config_valueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Config_valueContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Config_valueContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Config_valueContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DIGITS_IDENTIFIER
/// Returns `None` if there is no child corresponding to token DIGITS_IDENTIFIER
fn DIGITS_IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(DIGITS_IDENTIFIER, 0)
}
fn unit(&self) -> Option<Rc<UnitContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DECIMAL_LITERAL
/// Returns `None` if there is no child corresponding to token DECIMAL_LITERAL
fn DECIMAL_LITERAL(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(DECIMAL_LITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token FLOAT_LITERAL
/// Returns `None` if there is no child corresponding to token FLOAT_LITERAL
fn FLOAT_LITERAL(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(FLOAT_LITERAL, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token IDENTIFIER in current rule
fn IDENTIFIER_all(&self) -> Vec<Rc<TerminalNode<'input,DesignParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENTIFIER, starting from 0.
/// Returns `None` if number of children corresponding to token IDENTIFIER is less or equal than `i`.
fn IDENTIFIER(&self, i: usize) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, i)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(COMMA, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_LITERAL
/// Returns `None` if there is no child corresponding to token STRING_LITERAL
fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL, 0)
}

}

impl<'input> Config_valueContextAttrs<'input> for Config_valueContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn config_value(&mut self,)
	-> Result<Rc<Config_valueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Config_valueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_config_value);
        let mut _localctx: Rc<Config_valueContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(117);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 DIGITS_IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(99);
					recog.base.match_token(DIGITS_IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(101);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__0) | (1usize << T__1) | (1usize << T__2))) != 0) {
						{
						/*InvokeRule unit*/
						recog.base.set_state(100);
						recog.unit()?;

						}
					}

					}
				}

			 DECIMAL_LITERAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(103);
					recog.base.match_token(DECIMAL_LITERAL,&mut recog.err_handler)?;

					recog.base.set_state(105);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__0) | (1usize << T__1) | (1usize << T__2))) != 0) {
						{
						/*InvokeRule unit*/
						recog.base.set_state(104);
						recog.unit()?;

						}
					}

					}
				}

			 FLOAT_LITERAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(107);
					recog.base.match_token(FLOAT_LITERAL,&mut recog.err_handler)?;

					recog.base.set_state(109);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__0) | (1usize << T__1) | (1usize << T__2))) != 0) {
						{
						/*InvokeRule unit*/
						recog.base.set_state(108);
						recog.unit()?;

						}
					}

					}
				}

			 IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(111);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(114);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==COMMA {
						{
						recog.base.set_state(112);
						recog.base.match_token(COMMA,&mut recog.err_handler)?;

						recog.base.set_state(113);
						recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

						}
					}

					}
				}

			 STRING_LITERAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(116);
					recog.base.match_token(STRING_LITERAL,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- unit ----------------
pub type UnitContextAll<'input> = UnitContext<'input>;


pub type UnitContext<'input> = BaseParserRuleContext<'input,UnitContextExt<'input>>;

#[derive(Clone)]
pub struct UnitContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for UnitContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for UnitContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_unit(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_unit(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for UnitContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_unit(self);
	}
}

impl<'input> CustomRuleContext<'input> for UnitContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unit }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unit }
}
antlr_rust::type_id!{UnitContextExt<'a>}

impl<'input> UnitContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<UnitContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,UnitContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait UnitContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<UnitContextExt<'input>>{


}

impl<'input> UnitContextAttrs<'input> for UnitContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn unit(&mut self,)
	-> Result<Rc<UnitContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = UnitContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_unit);
        let mut _localctx: Rc<UnitContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(119);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__0) | (1usize << T__1) | (1usize << T__2))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- declarations ----------------
pub type DeclarationsContextAll<'input> = DeclarationsContext<'input>;


pub type DeclarationsContext<'input> = BaseParserRuleContext<'input,DeclarationsContextExt<'input>>;

#[derive(Clone)]
pub struct DeclarationsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for DeclarationsContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for DeclarationsContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_declarations(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_declarations(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for DeclarationsContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_declarations(self);
	}
}

impl<'input> CustomRuleContext<'input> for DeclarationsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_declarations }
	//fn type_rule_index() -> usize where Self: Sized { RULE_declarations }
}
antlr_rust::type_id!{DeclarationsContextExt<'a>}

impl<'input> DeclarationsContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DeclarationsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DeclarationsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DeclarationsContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<DeclarationsContextExt<'input>>{

fn config_decl(&self) -> Option<Rc<Config_declContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn flow_decl(&self) -> Option<Rc<Flow_declContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn page_decl(&self) -> Option<Rc<Page_declContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn style_decl(&self) -> Option<Rc<Style_declContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn component_decl(&self) -> Option<Rc<Component_declContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn library_decl(&self) -> Option<Rc<Library_declContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn layout_decl(&self) -> Option<Rc<Layout_declContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DeclarationsContextAttrs<'input> for DeclarationsContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn declarations(&mut self,)
	-> Result<Rc<DeclarationsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DeclarationsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_declarations);
        let mut _localctx: Rc<DeclarationsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(128);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule config_decl*/
					recog.base.set_state(121);
					recog.config_decl()?;

					}
				}

			 FLOW 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule flow_decl*/
					recog.base.set_state(122);
					recog.flow_decl()?;

					}
				}

			 PAGE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule page_decl*/
					recog.base.set_state(123);
					recog.page_decl()?;

					}
				}

			 STYLE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule style_decl*/
					recog.base.set_state(124);
					recog.style_decl()?;

					}
				}

			 COMPONENT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule component_decl*/
					recog.base.set_state(125);
					recog.component_decl()?;

					}
				}

			 LIBRARY 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule library_decl*/
					recog.base.set_state(126);
					recog.library_decl()?;

					}
				}

			 LAYOUT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule layout_decl*/
					recog.base.set_state(127);
					recog.layout_decl()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- flow_decl ----------------
pub type Flow_declContextAll<'input> = Flow_declContext<'input>;


pub type Flow_declContext<'input> = BaseParserRuleContext<'input,Flow_declContextExt<'input>>;

#[derive(Clone)]
pub struct Flow_declContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Flow_declContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Flow_declContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_flow_decl(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_flow_decl(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Flow_declContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_flow_decl(self);
	}
}

impl<'input> CustomRuleContext<'input> for Flow_declContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_flow_decl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_flow_decl }
}
antlr_rust::type_id!{Flow_declContextExt<'a>}

impl<'input> Flow_declContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Flow_declContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Flow_declContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Flow_declContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Flow_declContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token FLOW
/// Returns `None` if there is no child corresponding to token FLOW
fn FLOW(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(FLOW, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn interaction_decl_all(&self) ->  Vec<Rc<Interaction_declContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn interaction_decl(&self, i: usize) -> Option<Rc<Interaction_declContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Flow_declContextAttrs<'input> for Flow_declContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn flow_decl(&mut self,)
	-> Result<Rc<Flow_declContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Flow_declContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_flow_decl);
        let mut _localctx: Rc<Flow_declContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(130);
			recog.base.match_token(FLOW,&mut recog.err_handler)?;

			recog.base.set_state(131);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(132);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(136);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << SEE) | (1usize << DO) | (1usize << REACT))) != 0) {
				{
				{
				/*InvokeRule interaction_decl*/
				recog.base.set_state(133);
				recog.interaction_decl()?;

				}
				}
				recog.base.set_state(138);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(139);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- interaction_decl ----------------
pub type Interaction_declContextAll<'input> = Interaction_declContext<'input>;


pub type Interaction_declContext<'input> = BaseParserRuleContext<'input,Interaction_declContextExt<'input>>;

#[derive(Clone)]
pub struct Interaction_declContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Interaction_declContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Interaction_declContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_interaction_decl(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_interaction_decl(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Interaction_declContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_interaction_decl(self);
	}
}

impl<'input> CustomRuleContext<'input> for Interaction_declContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_interaction_decl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_interaction_decl }
}
antlr_rust::type_id!{Interaction_declContextExt<'a>}

impl<'input> Interaction_declContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Interaction_declContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Interaction_declContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Interaction_declContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Interaction_declContextExt<'input>>{

fn see_decl(&self) -> Option<Rc<See_declContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn do_decl(&self) -> Option<Rc<Do_declContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn react_decl(&self) -> Option<Rc<React_declContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Interaction_declContextAttrs<'input> for Interaction_declContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn interaction_decl(&mut self,)
	-> Result<Rc<Interaction_declContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Interaction_declContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_interaction_decl);
        let mut _localctx: Rc<Interaction_declContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(144);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 SEE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule see_decl*/
					recog.base.set_state(141);
					recog.see_decl()?;

					}
				}

			 DO 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule do_decl*/
					recog.base.set_state(142);
					recog.do_decl()?;

					}
				}

			 REACT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule react_decl*/
					recog.base.set_state(143);
					recog.react_decl()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- see_decl ----------------
pub type See_declContextAll<'input> = See_declContext<'input>;


pub type See_declContext<'input> = BaseParserRuleContext<'input,See_declContextExt<'input>>;

#[derive(Clone)]
pub struct See_declContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for See_declContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for See_declContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_see_decl(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_see_decl(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for See_declContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_see_decl(self);
	}
}

impl<'input> CustomRuleContext<'input> for See_declContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_see_decl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_see_decl }
}
antlr_rust::type_id!{See_declContextExt<'a>}

impl<'input> See_declContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<See_declContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,See_declContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait See_declContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<See_declContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SEE
/// Returns `None` if there is no child corresponding to token SEE
fn SEE(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(SEE, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_LITERAL
/// Returns `None` if there is no child corresponding to token STRING_LITERAL
fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}
fn component_name(&self) -> Option<Rc<Component_nameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> See_declContextAttrs<'input> for See_declContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn see_decl(&mut self,)
	-> Result<Rc<See_declContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = See_declContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_see_decl);
        let mut _localctx: Rc<See_declContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(146);
			recog.base.match_token(SEE,&mut recog.err_handler)?;

			recog.base.set_state(151);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IDENTIFIER 
				=> {
					{
					recog.base.set_state(147);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}

			 STRING_LITERAL 
				=> {
					{
					recog.base.set_state(148);
					recog.base.match_token(STRING_LITERAL,&mut recog.err_handler)?;

					recog.base.set_state(149);
					recog.base.match_token(DOT,&mut recog.err_handler)?;

					/*InvokeRule component_name*/
					recog.base.set_state(150);
					recog.component_name()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- do_decl ----------------
pub type Do_declContextAll<'input> = Do_declContext<'input>;


pub type Do_declContext<'input> = BaseParserRuleContext<'input,Do_declContextExt<'input>>;

#[derive(Clone)]
pub struct Do_declContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Do_declContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Do_declContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_do_decl(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_do_decl(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Do_declContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_do_decl(self);
	}
}

impl<'input> CustomRuleContext<'input> for Do_declContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_do_decl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_do_decl }
}
antlr_rust::type_id!{Do_declContextExt<'a>}

impl<'input> Do_declContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Do_declContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Do_declContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Do_declContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Do_declContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DO
/// Returns `None` if there is no child corresponding to token DO
fn DO(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(DO, 0)
}
/// Retrieves first TerminalNode corresponding to token LBRACK
/// Returns `None` if there is no child corresponding to token LBRACK
fn LBRACK(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(LBRACK, 0)
}
fn action_name(&self) -> Option<Rc<Action_nameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RBRACK
/// Returns `None` if there is no child corresponding to token RBRACK
fn RBRACK(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(RBRACK, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_LITERAL
/// Returns `None` if there is no child corresponding to token STRING_LITERAL
fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}
fn component_name(&self) -> Option<Rc<Component_nameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Do_declContextAttrs<'input> for Do_declContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn do_decl(&mut self,)
	-> Result<Rc<Do_declContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Do_declContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_do_decl);
        let mut _localctx: Rc<Do_declContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(153);
			recog.base.match_token(DO,&mut recog.err_handler)?;

			recog.base.set_state(154);
			recog.base.match_token(LBRACK,&mut recog.err_handler)?;

			/*InvokeRule action_name*/
			recog.base.set_state(155);
			recog.action_name()?;

			recog.base.set_state(156);
			recog.base.match_token(RBRACK,&mut recog.err_handler)?;

			recog.base.set_state(157);
			recog.base.match_token(STRING_LITERAL,&mut recog.err_handler)?;

			recog.base.set_state(158);
			recog.base.match_token(DOT,&mut recog.err_handler)?;

			/*InvokeRule component_name*/
			recog.base.set_state(159);
			recog.component_name()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- react_decl ----------------
pub type React_declContextAll<'input> = React_declContext<'input>;


pub type React_declContext<'input> = BaseParserRuleContext<'input,React_declContextExt<'input>>;

#[derive(Clone)]
pub struct React_declContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for React_declContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for React_declContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_react_decl(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_react_decl(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for React_declContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_react_decl(self);
	}
}

impl<'input> CustomRuleContext<'input> for React_declContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_react_decl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_react_decl }
}
antlr_rust::type_id!{React_declContextExt<'a>}

impl<'input> React_declContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<React_declContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,React_declContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait React_declContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<React_declContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token REACT
/// Returns `None` if there is no child corresponding to token REACT
fn REACT(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(REACT, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn react_action(&self) -> Option<Rc<React_actionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn scene_name(&self) -> Option<Rc<Scene_nameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn animate_decl(&self) -> Option<Rc<Animate_declContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> React_declContextAttrs<'input> for React_declContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn react_decl(&mut self,)
	-> Result<Rc<React_declContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = React_declContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_react_decl);
        let mut _localctx: Rc<React_declContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(161);
			recog.base.match_token(REACT,&mut recog.err_handler)?;

			recog.base.set_state(163);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==IDENTIFIER {
				{
				/*InvokeRule scene_name*/
				recog.base.set_state(162);
				recog.scene_name()?;

				}
			}

			recog.base.set_state(165);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule react_action*/
			recog.base.set_state(166);
			recog.react_action()?;

			recog.base.set_state(168);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==WITHTEXT {
				{
				/*InvokeRule animate_decl*/
				recog.base.set_state(167);
				recog.animate_decl()?;

				}
			}

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- animate_decl ----------------
pub type Animate_declContextAll<'input> = Animate_declContext<'input>;


pub type Animate_declContext<'input> = BaseParserRuleContext<'input,Animate_declContextExt<'input>>;

#[derive(Clone)]
pub struct Animate_declContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Animate_declContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Animate_declContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_animate_decl(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_animate_decl(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Animate_declContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_animate_decl(self);
	}
}

impl<'input> CustomRuleContext<'input> for Animate_declContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_animate_decl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_animate_decl }
}
antlr_rust::type_id!{Animate_declContextExt<'a>}

impl<'input> Animate_declContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Animate_declContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Animate_declContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Animate_declContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Animate_declContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token WITHTEXT
/// Returns `None` if there is no child corresponding to token WITHTEXT
fn WITHTEXT(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(WITHTEXT, 0)
}
/// Retrieves first TerminalNode corresponding to token ANIMATE
/// Returns `None` if there is no child corresponding to token ANIMATE
fn ANIMATE(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(ANIMATE, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn animate_name(&self) -> Option<Rc<Animate_nameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}

}

impl<'input> Animate_declContextAttrs<'input> for Animate_declContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn animate_decl(&mut self,)
	-> Result<Rc<Animate_declContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Animate_declContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_animate_decl);
        let mut _localctx: Rc<Animate_declContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(170);
			recog.base.match_token(WITHTEXT,&mut recog.err_handler)?;

			recog.base.set_state(171);
			recog.base.match_token(ANIMATE,&mut recog.err_handler)?;

			recog.base.set_state(172);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			/*InvokeRule animate_name*/
			recog.base.set_state(173);
			recog.animate_name()?;

			recog.base.set_state(174);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- react_action ----------------
pub type React_actionContextAll<'input> = React_actionContext<'input>;


pub type React_actionContext<'input> = BaseParserRuleContext<'input,React_actionContextExt<'input>>;

#[derive(Clone)]
pub struct React_actionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for React_actionContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for React_actionContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_react_action(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_react_action(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for React_actionContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_react_action(self);
	}
}

impl<'input> CustomRuleContext<'input> for React_actionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_react_action }
	//fn type_rule_index() -> usize where Self: Sized { RULE_react_action }
}
antlr_rust::type_id!{React_actionContextExt<'a>}

impl<'input> React_actionContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<React_actionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,React_actionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait React_actionContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<React_actionContextExt<'input>>{

fn goto_action(&self) -> Option<Rc<Goto_actionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn show_action(&self) -> Option<Rc<Show_actionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> React_actionContextAttrs<'input> for React_actionContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn react_action(&mut self,)
	-> Result<Rc<React_actionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = React_actionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_react_action);
        let mut _localctx: Rc<React_actionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(178);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 GOTO_KEY 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule goto_action*/
					recog.base.set_state(176);
					recog.goto_action()?;

					}
				}

			 SHOW_KEY 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule show_action*/
					recog.base.set_state(177);
					recog.show_action()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- goto_action ----------------
pub type Goto_actionContextAll<'input> = Goto_actionContext<'input>;


pub type Goto_actionContext<'input> = BaseParserRuleContext<'input,Goto_actionContextExt<'input>>;

#[derive(Clone)]
pub struct Goto_actionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Goto_actionContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Goto_actionContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_goto_action(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_goto_action(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Goto_actionContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_goto_action(self);
	}
}

impl<'input> CustomRuleContext<'input> for Goto_actionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_goto_action }
	//fn type_rule_index() -> usize where Self: Sized { RULE_goto_action }
}
antlr_rust::type_id!{Goto_actionContextExt<'a>}

impl<'input> Goto_actionContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Goto_actionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Goto_actionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Goto_actionContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Goto_actionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token GOTO_KEY
/// Returns `None` if there is no child corresponding to token GOTO_KEY
fn GOTO_KEY(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(GOTO_KEY, 0)
}
fn component_name(&self) -> Option<Rc<Component_nameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Goto_actionContextAttrs<'input> for Goto_actionContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn goto_action(&mut self,)
	-> Result<Rc<Goto_actionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Goto_actionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_goto_action);
        let mut _localctx: Rc<Goto_actionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(180);
			recog.base.match_token(GOTO_KEY,&mut recog.err_handler)?;

			/*InvokeRule component_name*/
			recog.base.set_state(181);
			recog.component_name()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- show_action ----------------
pub type Show_actionContextAll<'input> = Show_actionContext<'input>;


pub type Show_actionContext<'input> = BaseParserRuleContext<'input,Show_actionContextExt<'input>>;

#[derive(Clone)]
pub struct Show_actionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Show_actionContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Show_actionContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_show_action(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_show_action(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Show_actionContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_show_action(self);
	}
}

impl<'input> CustomRuleContext<'input> for Show_actionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_show_action }
	//fn type_rule_index() -> usize where Self: Sized { RULE_show_action }
}
antlr_rust::type_id!{Show_actionContextExt<'a>}

impl<'input> Show_actionContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Show_actionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Show_actionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Show_actionContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Show_actionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SHOW_KEY
/// Returns `None` if there is no child corresponding to token SHOW_KEY
fn SHOW_KEY(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(SHOW_KEY, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_LITERAL
/// Returns `None` if there is no child corresponding to token STRING_LITERAL
fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}
fn component_name(&self) -> Option<Rc<Component_nameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Show_actionContextAttrs<'input> for Show_actionContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn show_action(&mut self,)
	-> Result<Rc<Show_actionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Show_actionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_show_action);
        let mut _localctx: Rc<Show_actionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(183);
			recog.base.match_token(SHOW_KEY,&mut recog.err_handler)?;

			recog.base.set_state(184);
			recog.base.match_token(STRING_LITERAL,&mut recog.err_handler)?;

			recog.base.set_state(185);
			recog.base.match_token(DOT,&mut recog.err_handler)?;

			/*InvokeRule component_name*/
			recog.base.set_state(186);
			recog.component_name()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- action_name ----------------
pub type Action_nameContextAll<'input> = Action_nameContext<'input>;


pub type Action_nameContext<'input> = BaseParserRuleContext<'input,Action_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Action_nameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Action_nameContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Action_nameContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_action_name(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_action_name(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Action_nameContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_action_name(self);
	}
}

impl<'input> CustomRuleContext<'input> for Action_nameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_action_name }
	//fn type_rule_index() -> usize where Self: Sized { RULE_action_name }
}
antlr_rust::type_id!{Action_nameContextExt<'a>}

impl<'input> Action_nameContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Action_nameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Action_nameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Action_nameContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Action_nameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> Action_nameContextAttrs<'input> for Action_nameContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn action_name(&mut self,)
	-> Result<Rc<Action_nameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Action_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_action_name);
        let mut _localctx: Rc<Action_nameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(188);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- component_value ----------------
pub type Component_valueContextAll<'input> = Component_valueContext<'input>;


pub type Component_valueContext<'input> = BaseParserRuleContext<'input,Component_valueContextExt<'input>>;

#[derive(Clone)]
pub struct Component_valueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Component_valueContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Component_valueContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_component_value(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_component_value(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Component_valueContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_component_value(self);
	}
}

impl<'input> CustomRuleContext<'input> for Component_valueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_component_value }
	//fn type_rule_index() -> usize where Self: Sized { RULE_component_value }
}
antlr_rust::type_id!{Component_valueContextExt<'a>}

impl<'input> Component_valueContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Component_valueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Component_valueContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Component_valueContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Component_valueContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> Component_valueContextAttrs<'input> for Component_valueContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn component_value(&mut self,)
	-> Result<Rc<Component_valueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Component_valueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_component_value);
        let mut _localctx: Rc<Component_valueContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(190);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- component_name ----------------
pub type Component_nameContextAll<'input> = Component_nameContext<'input>;


pub type Component_nameContext<'input> = BaseParserRuleContext<'input,Component_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Component_nameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Component_nameContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Component_nameContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_component_name(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_component_name(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Component_nameContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_component_name(self);
	}
}

impl<'input> CustomRuleContext<'input> for Component_nameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_component_name }
	//fn type_rule_index() -> usize where Self: Sized { RULE_component_name }
}
antlr_rust::type_id!{Component_nameContextExt<'a>}

impl<'input> Component_nameContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Component_nameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Component_nameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Component_nameContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Component_nameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> Component_nameContextAttrs<'input> for Component_nameContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn component_name(&mut self,)
	-> Result<Rc<Component_nameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Component_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_component_name);
        let mut _localctx: Rc<Component_nameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(192);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- scene_name ----------------
pub type Scene_nameContextAll<'input> = Scene_nameContext<'input>;


pub type Scene_nameContext<'input> = BaseParserRuleContext<'input,Scene_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Scene_nameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Scene_nameContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Scene_nameContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_scene_name(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_scene_name(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Scene_nameContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_scene_name(self);
	}
}

impl<'input> CustomRuleContext<'input> for Scene_nameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scene_name }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scene_name }
}
antlr_rust::type_id!{Scene_nameContextExt<'a>}

impl<'input> Scene_nameContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Scene_nameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Scene_nameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Scene_nameContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Scene_nameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> Scene_nameContextAttrs<'input> for Scene_nameContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn scene_name(&mut self,)
	-> Result<Rc<Scene_nameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Scene_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_scene_name);
        let mut _localctx: Rc<Scene_nameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(194);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- animate_name ----------------
pub type Animate_nameContextAll<'input> = Animate_nameContext<'input>;


pub type Animate_nameContext<'input> = BaseParserRuleContext<'input,Animate_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Animate_nameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Animate_nameContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Animate_nameContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_animate_name(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_animate_name(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Animate_nameContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_animate_name(self);
	}
}

impl<'input> CustomRuleContext<'input> for Animate_nameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_animate_name }
	//fn type_rule_index() -> usize where Self: Sized { RULE_animate_name }
}
antlr_rust::type_id!{Animate_nameContextExt<'a>}

impl<'input> Animate_nameContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Animate_nameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Animate_nameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Animate_nameContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Animate_nameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> Animate_nameContextAttrs<'input> for Animate_nameContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn animate_name(&mut self,)
	-> Result<Rc<Animate_nameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Animate_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_animate_name);
        let mut _localctx: Rc<Animate_nameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(196);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- page_decl ----------------
pub type Page_declContextAll<'input> = Page_declContext<'input>;


pub type Page_declContext<'input> = BaseParserRuleContext<'input,Page_declContextExt<'input>>;

#[derive(Clone)]
pub struct Page_declContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Page_declContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Page_declContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_page_decl(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_page_decl(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Page_declContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_page_decl(self);
	}
}

impl<'input> CustomRuleContext<'input> for Page_declContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_page_decl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_page_decl }
}
antlr_rust::type_id!{Page_declContextExt<'a>}

impl<'input> Page_declContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Page_declContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Page_declContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Page_declContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Page_declContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PAGE
/// Returns `None` if there is no child corresponding to token PAGE
fn PAGE(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(PAGE, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn component_body_decl_all(&self) ->  Vec<Rc<Component_body_declContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn component_body_decl(&self, i: usize) -> Option<Rc<Component_body_declContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Page_declContextAttrs<'input> for Page_declContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn page_decl(&mut self,)
	-> Result<Rc<Page_declContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Page_declContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_page_decl);
        let mut _localctx: Rc<Page_declContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(198);
			recog.base.match_token(PAGE,&mut recog.err_handler)?;

			recog.base.set_state(199);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(200);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(204);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==IDENTIFIER {
				{
				{
				/*InvokeRule component_body_decl*/
				recog.base.set_state(201);
				recog.component_body_decl()?;

				}
				}
				recog.base.set_state(206);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(207);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- component_decl ----------------
pub type Component_declContextAll<'input> = Component_declContext<'input>;


pub type Component_declContext<'input> = BaseParserRuleContext<'input,Component_declContextExt<'input>>;

#[derive(Clone)]
pub struct Component_declContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Component_declContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Component_declContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_component_decl(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_component_decl(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Component_declContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_component_decl(self);
	}
}

impl<'input> CustomRuleContext<'input> for Component_declContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_component_decl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_component_decl }
}
antlr_rust::type_id!{Component_declContextExt<'a>}

impl<'input> Component_declContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Component_declContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Component_declContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Component_declContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Component_declContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COMPONENT
/// Returns `None` if there is no child corresponding to token COMPONENT
fn COMPONENT(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(COMPONENT, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn component_body_decl_all(&self) ->  Vec<Rc<Component_body_declContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn component_body_decl(&self, i: usize) -> Option<Rc<Component_body_declContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Component_declContextAttrs<'input> for Component_declContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn component_decl(&mut self,)
	-> Result<Rc<Component_declContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Component_declContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_component_decl);
        let mut _localctx: Rc<Component_declContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(209);
			recog.base.match_token(COMPONENT,&mut recog.err_handler)?;

			recog.base.set_state(210);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(211);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(215);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==IDENTIFIER {
				{
				{
				/*InvokeRule component_body_decl*/
				recog.base.set_state(212);
				recog.component_body_decl()?;

				}
				}
				recog.base.set_state(217);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(218);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- component_body_decl ----------------
#[derive(Debug)]
pub enum Component_body_declContextAll<'input>{
	Component_body_configContext(Component_body_configContext<'input>),
	Component_body_nameContext(Component_body_nameContext<'input>),
Error(Component_body_declContext<'input>)
}
antlr_rust::type_id!{Component_body_declContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for Component_body_declContextAll<'input>{}

impl<'input> DesignParserContext<'input> for Component_body_declContextAll<'input>{}

impl<'input> Deref for Component_body_declContextAll<'input>{
	type Target = dyn Component_body_declContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use Component_body_declContextAll::*;
		match self{
			Component_body_configContext(inner) => inner,
			Component_body_nameContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Component_body_declContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn DesignVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Component_body_declContextAll<'input>{
    fn enter(&self, listener: &mut (dyn DesignListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn DesignListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type Component_body_declContext<'input> = BaseParserRuleContext<'input,Component_body_declContextExt<'input>>;

#[derive(Clone)]
pub struct Component_body_declContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Component_body_declContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Component_body_declContext<'input>{
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Component_body_declContext<'input>{
}

impl<'input> CustomRuleContext<'input> for Component_body_declContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_component_body_decl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_component_body_decl }
}
antlr_rust::type_id!{Component_body_declContextExt<'a>}

impl<'input> Component_body_declContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Component_body_declContextAll<'input>> {
		Rc::new(
		Component_body_declContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Component_body_declContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait Component_body_declContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Component_body_declContextExt<'input>>{


}

impl<'input> Component_body_declContextAttrs<'input> for Component_body_declContext<'input>{}

pub type Component_body_configContext<'input> = BaseParserRuleContext<'input,Component_body_configContextExt<'input>>;

pub trait Component_body_configContextAttrs<'input>: DesignParserContext<'input>{
	fn config_key(&self) -> Option<Rc<Config_keyContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token COLON
	/// Returns `None` if there is no child corresponding to token COLON
	fn COLON(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
		self.get_token(COLON, 0)
	}
	fn config_value(&self) -> Option<Rc<Config_valueContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> Component_body_configContextAttrs<'input> for Component_body_configContext<'input>{}

pub struct Component_body_configContextExt<'input>{
	base:Component_body_declContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{Component_body_configContextExt<'a>}

impl<'input> DesignParserContext<'input> for Component_body_configContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Component_body_configContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_component_body_config(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_component_body_config(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Component_body_configContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_component_body_config(self);
	}
}

impl<'input> CustomRuleContext<'input> for Component_body_configContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_component_body_decl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_component_body_decl }
}

impl<'input> Borrow<Component_body_declContextExt<'input>> for Component_body_configContext<'input>{
	fn borrow(&self) -> &Component_body_declContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Component_body_declContextExt<'input>> for Component_body_configContext<'input>{
	fn borrow_mut(&mut self) -> &mut Component_body_declContextExt<'input> { &mut self.base }
}

impl<'input> Component_body_declContextAttrs<'input> for Component_body_configContext<'input> {}

impl<'input> Component_body_configContextExt<'input>{
	fn new(ctx: &dyn Component_body_declContextAttrs<'input>) -> Rc<Component_body_declContextAll<'input>>  {
		Rc::new(
			Component_body_declContextAll::Component_body_configContext(
				BaseParserRuleContext::copy_from(ctx,Component_body_configContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type Component_body_nameContext<'input> = BaseParserRuleContext<'input,Component_body_nameContextExt<'input>>;

pub trait Component_body_nameContextAttrs<'input>: DesignParserContext<'input>{
	fn component_name_all(&self) ->  Vec<Rc<Component_nameContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn component_name(&self, i: usize) -> Option<Rc<Component_nameContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
	fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,DesignParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
	/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
	fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
		self.get_token(COMMA, i)
	}
}

impl<'input> Component_body_nameContextAttrs<'input> for Component_body_nameContext<'input>{}

pub struct Component_body_nameContextExt<'input>{
	base:Component_body_declContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{Component_body_nameContextExt<'a>}

impl<'input> DesignParserContext<'input> for Component_body_nameContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Component_body_nameContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_component_body_name(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_component_body_name(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Component_body_nameContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_component_body_name(self);
	}
}

impl<'input> CustomRuleContext<'input> for Component_body_nameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_component_body_decl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_component_body_decl }
}

impl<'input> Borrow<Component_body_declContextExt<'input>> for Component_body_nameContext<'input>{
	fn borrow(&self) -> &Component_body_declContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Component_body_declContextExt<'input>> for Component_body_nameContext<'input>{
	fn borrow_mut(&mut self) -> &mut Component_body_declContextExt<'input> { &mut self.base }
}

impl<'input> Component_body_declContextAttrs<'input> for Component_body_nameContext<'input> {}

impl<'input> Component_body_nameContextExt<'input>{
	fn new(ctx: &dyn Component_body_declContextAttrs<'input>) -> Rc<Component_body_declContextAll<'input>>  {
		Rc::new(
			Component_body_declContextAll::Component_body_nameContext(
				BaseParserRuleContext::copy_from(ctx,Component_body_nameContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn component_body_decl(&mut self,)
	-> Result<Rc<Component_body_declContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Component_body_declContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_component_body_decl);
        let mut _localctx: Rc<Component_body_declContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(232);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(17,&mut recog.base)? {
				1 =>{
					let tmp = Component_body_nameContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					/*InvokeRule component_name*/
					recog.base.set_state(220);
					recog.component_name()?;

					recog.base.set_state(225);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==COMMA {
						{
						{
						recog.base.set_state(221);
						recog.base.match_token(COMMA,&mut recog.err_handler)?;

						/*InvokeRule component_name*/
						recog.base.set_state(222);
						recog.component_name()?;

						}
						}
						recog.base.set_state(227);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}
			,
				2 =>{
					let tmp = Component_body_configContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					/*InvokeRule config_key*/
					recog.base.set_state(228);
					recog.config_key()?;

					recog.base.set_state(229);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					/*InvokeRule config_value*/
					recog.base.set_state(230);
					recog.config_value()?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- layout_decl ----------------
pub type Layout_declContextAll<'input> = Layout_declContext<'input>;


pub type Layout_declContext<'input> = BaseParserRuleContext<'input,Layout_declContextExt<'input>>;

#[derive(Clone)]
pub struct Layout_declContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Layout_declContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Layout_declContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_layout_decl(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_layout_decl(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Layout_declContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_layout_decl(self);
	}
}

impl<'input> CustomRuleContext<'input> for Layout_declContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_layout_decl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_layout_decl }
}
antlr_rust::type_id!{Layout_declContextExt<'a>}

impl<'input> Layout_declContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Layout_declContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Layout_declContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Layout_declContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Layout_declContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LAYOUT
/// Returns `None` if there is no child corresponding to token LAYOUT
fn LAYOUT(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(LAYOUT, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn layout_row_all(&self) ->  Vec<Rc<Layout_rowContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn layout_row(&self, i: usize) -> Option<Rc<Layout_rowContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Layout_declContextAttrs<'input> for Layout_declContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn layout_decl(&mut self,)
	-> Result<Rc<Layout_declContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Layout_declContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_layout_decl);
        let mut _localctx: Rc<Layout_declContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(234);
			recog.base.match_token(LAYOUT,&mut recog.err_handler)?;

			recog.base.set_state(235);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(236);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(240);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__3 || _la==T__4 {
				{
				{
				/*InvokeRule layout_row*/
				recog.base.set_state(237);
				recog.layout_row()?;

				}
				}
				recog.base.set_state(242);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(243);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- layout_row ----------------
pub type Layout_rowContextAll<'input> = Layout_rowContext<'input>;


pub type Layout_rowContext<'input> = BaseParserRuleContext<'input,Layout_rowContextExt<'input>>;

#[derive(Clone)]
pub struct Layout_rowContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Layout_rowContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Layout_rowContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_layout_row(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_layout_row(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Layout_rowContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_layout_row(self);
	}
}

impl<'input> CustomRuleContext<'input> for Layout_rowContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_layout_row }
	//fn type_rule_index() -> usize where Self: Sized { RULE_layout_row }
}
antlr_rust::type_id!{Layout_rowContextExt<'a>}

impl<'input> Layout_rowContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Layout_rowContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Layout_rowContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Layout_rowContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Layout_rowContextExt<'input>>{

fn layout_lines(&self) -> Option<Rc<Layout_linesContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Layout_rowContextAttrs<'input> for Layout_rowContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn layout_row(&mut self,)
	-> Result<Rc<Layout_rowContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Layout_rowContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_layout_row);
        let mut _localctx: Rc<Layout_rowContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			recog.base.set_state(255);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__3 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(245);
					recog.base.match_token(T__3,&mut recog.err_handler)?;

					recog.base.set_state(249);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(19,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							recog.base.set_state(246);
							recog.base.match_token(T__3,&mut recog.err_handler)?;

							}
							} 
						}
						recog.base.set_state(251);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(19,&mut recog.base)?;
					}
					}
				}

			 T__4 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule layout_lines*/
					recog.base.set_state(252);
					recog.layout_lines()?;

					recog.base.set_state(253);
					recog.base.match_token(T__4,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- layout_lines ----------------
pub type Layout_linesContextAll<'input> = Layout_linesContext<'input>;


pub type Layout_linesContext<'input> = BaseParserRuleContext<'input,Layout_linesContextExt<'input>>;

#[derive(Clone)]
pub struct Layout_linesContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Layout_linesContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Layout_linesContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_layout_lines(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_layout_lines(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Layout_linesContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_layout_lines(self);
	}
}

impl<'input> CustomRuleContext<'input> for Layout_linesContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_layout_lines }
	//fn type_rule_index() -> usize where Self: Sized { RULE_layout_lines }
}
antlr_rust::type_id!{Layout_linesContextExt<'a>}

impl<'input> Layout_linesContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Layout_linesContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Layout_linesContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Layout_linesContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Layout_linesContextExt<'input>>{

fn layout_line_all(&self) ->  Vec<Rc<Layout_lineContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn layout_line(&self, i: usize) -> Option<Rc<Layout_lineContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Layout_linesContextAttrs<'input> for Layout_linesContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn layout_lines(&mut self,)
	-> Result<Rc<Layout_linesContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Layout_linesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_layout_lines);
        let mut _localctx: Rc<Layout_linesContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule layout_line*/
			recog.base.set_state(257);
			recog.layout_line()?;

			recog.base.set_state(261);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(21,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule layout_line*/
					recog.base.set_state(258);
					recog.layout_line()?;

					}
					} 
				}
				recog.base.set_state(263);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(21,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- layout_line ----------------
pub type Layout_lineContextAll<'input> = Layout_lineContext<'input>;


pub type Layout_lineContext<'input> = BaseParserRuleContext<'input,Layout_lineContextExt<'input>>;

#[derive(Clone)]
pub struct Layout_lineContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Layout_lineContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Layout_lineContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_layout_line(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_layout_line(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Layout_lineContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_layout_line(self);
	}
}

impl<'input> CustomRuleContext<'input> for Layout_lineContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_layout_line }
	//fn type_rule_index() -> usize where Self: Sized { RULE_layout_line }
}
antlr_rust::type_id!{Layout_lineContextExt<'a>}

impl<'input> Layout_lineContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Layout_lineContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Layout_lineContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Layout_lineContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Layout_lineContextExt<'input>>{

fn component_use_decl(&self) -> Option<Rc<Component_use_declContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Layout_lineContextAttrs<'input> for Layout_lineContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn layout_line(&mut self,)
	-> Result<Rc<Layout_lineContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Layout_lineContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_layout_line);
        let mut _localctx: Rc<Layout_lineContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(264);
			recog.base.match_token(T__4,&mut recog.err_handler)?;

			/*InvokeRule component_use_decl*/
			recog.base.set_state(265);
			recog.component_use_decl()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- component_use_decl ----------------
pub type Component_use_declContextAll<'input> = Component_use_declContext<'input>;


pub type Component_use_declContext<'input> = BaseParserRuleContext<'input,Component_use_declContextExt<'input>>;

#[derive(Clone)]
pub struct Component_use_declContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Component_use_declContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Component_use_declContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_component_use_decl(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_component_use_decl(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Component_use_declContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_component_use_decl(self);
	}
}

impl<'input> CustomRuleContext<'input> for Component_use_declContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_component_use_decl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_component_use_decl }
}
antlr_rust::type_id!{Component_use_declContextExt<'a>}

impl<'input> Component_use_declContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Component_use_declContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Component_use_declContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Component_use_declContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Component_use_declContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DECIMAL_LITERAL
/// Returns `None` if there is no child corresponding to token DECIMAL_LITERAL
fn DECIMAL_LITERAL(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(DECIMAL_LITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token POSITION
/// Returns `None` if there is no child corresponding to token POSITION
fn POSITION(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(POSITION, 0)
}
fn component_name(&self) -> Option<Rc<Component_nameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn component_layout_value(&self) -> Option<Rc<Component_layout_valueContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_LITERAL
/// Returns `None` if there is no child corresponding to token STRING_LITERAL
fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL, 0)
}

}

impl<'input> Component_use_declContextAttrs<'input> for Component_use_declContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn component_use_decl(&mut self,)
	-> Result<Rc<Component_use_declContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Component_use_declContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_component_use_decl);
        let mut _localctx: Rc<Component_use_declContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(277);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 DECIMAL_LITERAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(267);
					recog.base.match_token(DECIMAL_LITERAL,&mut recog.err_handler)?;

					}
				}

			 POSITION 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(268);
					recog.base.match_token(POSITION,&mut recog.err_handler)?;

					}
				}

			 IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule component_name*/
					recog.base.set_state(269);
					recog.component_name()?;

					recog.base.set_state(274);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LPAREN {
						{
						recog.base.set_state(270);
						recog.base.match_token(LPAREN,&mut recog.err_handler)?;

						/*InvokeRule component_layout_value*/
						recog.base.set_state(271);
						recog.component_layout_value()?;

						recog.base.set_state(272);
						recog.base.match_token(RPAREN,&mut recog.err_handler)?;

						}
					}

					}
				}

			 STRING_LITERAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(276);
					recog.base.match_token(STRING_LITERAL,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- component_layout_value ----------------
pub type Component_layout_valueContextAll<'input> = Component_layout_valueContext<'input>;


pub type Component_layout_valueContext<'input> = BaseParserRuleContext<'input,Component_layout_valueContextExt<'input>>;

#[derive(Clone)]
pub struct Component_layout_valueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Component_layout_valueContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Component_layout_valueContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_component_layout_value(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_component_layout_value(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Component_layout_valueContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_component_layout_value(self);
	}
}

impl<'input> CustomRuleContext<'input> for Component_layout_valueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_component_layout_value }
	//fn type_rule_index() -> usize where Self: Sized { RULE_component_layout_value }
}
antlr_rust::type_id!{Component_layout_valueContextExt<'a>}

impl<'input> Component_layout_valueContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Component_layout_valueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Component_layout_valueContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Component_layout_valueContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Component_layout_valueContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DIGITS_IDENTIFIER
/// Returns `None` if there is no child corresponding to token DIGITS_IDENTIFIER
fn DIGITS_IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(DIGITS_IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token POSITION
/// Returns `None` if there is no child corresponding to token POSITION
fn POSITION(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(POSITION, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_LITERAL
/// Returns `None` if there is no child corresponding to token STRING_LITERAL
fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL, 0)
}

}

impl<'input> Component_layout_valueContextAttrs<'input> for Component_layout_valueContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn component_layout_value(&mut self,)
	-> Result<Rc<Component_layout_valueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Component_layout_valueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_component_layout_value);
        let mut _localctx: Rc<Component_layout_valueContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(279);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << POSITION) | (1usize << STRING_LITERAL) | (1usize << DIGITS_IDENTIFIER))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- style_decl ----------------
pub type Style_declContextAll<'input> = Style_declContext<'input>;


pub type Style_declContext<'input> = BaseParserRuleContext<'input,Style_declContextExt<'input>>;

#[derive(Clone)]
pub struct Style_declContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Style_declContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Style_declContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_style_decl(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_style_decl(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Style_declContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_style_decl(self);
	}
}

impl<'input> CustomRuleContext<'input> for Style_declContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_style_decl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_style_decl }
}
antlr_rust::type_id!{Style_declContextExt<'a>}

impl<'input> Style_declContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Style_declContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Style_declContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Style_declContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Style_declContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STYLE
/// Returns `None` if there is no child corresponding to token STYLE
fn STYLE(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(STYLE, 0)
}
fn style_name(&self) -> Option<Rc<Style_nameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
fn style_body(&self) -> Option<Rc<Style_bodyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}

}

impl<'input> Style_declContextAttrs<'input> for Style_declContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn style_decl(&mut self,)
	-> Result<Rc<Style_declContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Style_declContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_style_decl);
        let mut _localctx: Rc<Style_declContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(281);
			recog.base.match_token(STYLE,&mut recog.err_handler)?;

			/*InvokeRule style_name*/
			recog.base.set_state(282);
			recog.style_name()?;

			recog.base.set_state(283);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			/*InvokeRule style_body*/
			recog.base.set_state(284);
			recog.style_body()?;

			recog.base.set_state(285);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- style_name ----------------
pub type Style_nameContextAll<'input> = Style_nameContext<'input>;


pub type Style_nameContext<'input> = BaseParserRuleContext<'input,Style_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Style_nameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Style_nameContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Style_nameContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_style_name(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_style_name(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Style_nameContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_style_name(self);
	}
}

impl<'input> CustomRuleContext<'input> for Style_nameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_style_name }
	//fn type_rule_index() -> usize where Self: Sized { RULE_style_name }
}
antlr_rust::type_id!{Style_nameContextExt<'a>}

impl<'input> Style_nameContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Style_nameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Style_nameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Style_nameContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Style_nameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> Style_nameContextAttrs<'input> for Style_nameContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn style_name(&mut self,)
	-> Result<Rc<Style_nameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Style_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_style_name);
        let mut _localctx: Rc<Style_nameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(287);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- style_body ----------------
pub type Style_bodyContextAll<'input> = Style_bodyContext<'input>;


pub type Style_bodyContext<'input> = BaseParserRuleContext<'input,Style_bodyContextExt<'input>>;

#[derive(Clone)]
pub struct Style_bodyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Style_bodyContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Style_bodyContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_style_body(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_style_body(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Style_bodyContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_style_body(self);
	}
}

impl<'input> CustomRuleContext<'input> for Style_bodyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_style_body }
	//fn type_rule_index() -> usize where Self: Sized { RULE_style_body }
}
antlr_rust::type_id!{Style_bodyContextExt<'a>}

impl<'input> Style_bodyContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Style_bodyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Style_bodyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Style_bodyContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Style_bodyContextExt<'input>>{

fn config_decl_all(&self) ->  Vec<Rc<Config_declContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn config_decl(&self, i: usize) -> Option<Rc<Config_declContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Style_bodyContextAttrs<'input> for Style_bodyContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn style_body(&mut self,)
	-> Result<Rc<Style_bodyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Style_bodyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_style_body);
        let mut _localctx: Rc<Style_bodyContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(294);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==IDENTIFIER {
				{
				{
				/*InvokeRule config_decl*/
				recog.base.set_state(289);
				recog.config_decl()?;

				recog.base.set_state(290);
				recog.base.match_token(T__5,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(296);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- library_decl ----------------
pub type Library_declContextAll<'input> = Library_declContext<'input>;


pub type Library_declContext<'input> = BaseParserRuleContext<'input,Library_declContextExt<'input>>;

#[derive(Clone)]
pub struct Library_declContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Library_declContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Library_declContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_library_decl(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_library_decl(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Library_declContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_library_decl(self);
	}
}

impl<'input> CustomRuleContext<'input> for Library_declContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_library_decl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_library_decl }
}
antlr_rust::type_id!{Library_declContextExt<'a>}

impl<'input> Library_declContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Library_declContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Library_declContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Library_declContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Library_declContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LIBRARY
/// Returns `None` if there is no child corresponding to token LIBRARY
fn LIBRARY(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(LIBRARY, 0)
}
fn library_name(&self) -> Option<Rc<Library_nameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn library_exp_all(&self) ->  Vec<Rc<Library_expContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn library_exp(&self, i: usize) -> Option<Rc<Library_expContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Library_declContextAttrs<'input> for Library_declContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn library_decl(&mut self,)
	-> Result<Rc<Library_declContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Library_declContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_library_decl);
        let mut _localctx: Rc<Library_declContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(297);
			recog.base.match_token(LIBRARY,&mut recog.err_handler)?;

			/*InvokeRule library_name*/
			recog.base.set_state(298);
			recog.library_name()?;

			recog.base.set_state(299);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(303);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==IDENTIFIER {
				{
				{
				/*InvokeRule library_exp*/
				recog.base.set_state(300);
				recog.library_exp()?;

				}
				}
				recog.base.set_state(305);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(306);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- library_exp ----------------
#[derive(Debug)]
pub enum Library_expContextAll<'input>{
	Library_objectContext(Library_objectContext<'input>),
	Library_configContext(Library_configContext<'input>),
Error(Library_expContext<'input>)
}
antlr_rust::type_id!{Library_expContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for Library_expContextAll<'input>{}

impl<'input> DesignParserContext<'input> for Library_expContextAll<'input>{}

impl<'input> Deref for Library_expContextAll<'input>{
	type Target = dyn Library_expContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use Library_expContextAll::*;
		match self{
			Library_objectContext(inner) => inner,
			Library_configContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Library_expContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn DesignVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Library_expContextAll<'input>{
    fn enter(&self, listener: &mut (dyn DesignListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn DesignListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type Library_expContext<'input> = BaseParserRuleContext<'input,Library_expContextExt<'input>>;

#[derive(Clone)]
pub struct Library_expContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Library_expContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Library_expContext<'input>{
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Library_expContext<'input>{
}

impl<'input> CustomRuleContext<'input> for Library_expContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_library_exp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_library_exp }
}
antlr_rust::type_id!{Library_expContextExt<'a>}

impl<'input> Library_expContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Library_expContextAll<'input>> {
		Rc::new(
		Library_expContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Library_expContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait Library_expContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Library_expContextExt<'input>>{


}

impl<'input> Library_expContextAttrs<'input> for Library_expContext<'input>{}

pub type Library_objectContext<'input> = BaseParserRuleContext<'input,Library_objectContextExt<'input>>;

pub trait Library_objectContextAttrs<'input>: DesignParserContext<'input>{
	fn preset_key(&self) -> Option<Rc<Preset_keyContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token LBRACE
	/// Returns `None` if there is no child corresponding to token LBRACE
	fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
		self.get_token(LBRACE, 0)
	}
	/// Retrieves first TerminalNode corresponding to token RBRACE
	/// Returns `None` if there is no child corresponding to token RBRACE
	fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
		self.get_token(RBRACE, 0)
	}
	fn keyValue_all(&self) ->  Vec<Rc<KeyValueContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn keyValue(&self, i: usize) -> Option<Rc<KeyValueContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> Library_objectContextAttrs<'input> for Library_objectContext<'input>{}

pub struct Library_objectContextExt<'input>{
	base:Library_expContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{Library_objectContextExt<'a>}

impl<'input> DesignParserContext<'input> for Library_objectContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Library_objectContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_library_object(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_library_object(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Library_objectContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_library_object(self);
	}
}

impl<'input> CustomRuleContext<'input> for Library_objectContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_library_exp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_library_exp }
}

impl<'input> Borrow<Library_expContextExt<'input>> for Library_objectContext<'input>{
	fn borrow(&self) -> &Library_expContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Library_expContextExt<'input>> for Library_objectContext<'input>{
	fn borrow_mut(&mut self) -> &mut Library_expContextExt<'input> { &mut self.base }
}

impl<'input> Library_expContextAttrs<'input> for Library_objectContext<'input> {}

impl<'input> Library_objectContextExt<'input>{
	fn new(ctx: &dyn Library_expContextAttrs<'input>) -> Rc<Library_expContextAll<'input>>  {
		Rc::new(
			Library_expContextAll::Library_objectContext(
				BaseParserRuleContext::copy_from(ctx,Library_objectContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type Library_configContext<'input> = BaseParserRuleContext<'input,Library_configContextExt<'input>>;

pub trait Library_configContextAttrs<'input>: DesignParserContext<'input>{
	fn preset_key(&self) -> Option<Rc<Preset_keyContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn preset_value(&self) -> Option<Rc<Preset_valueContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn preset_array(&self) -> Option<Rc<Preset_arrayContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> Library_configContextAttrs<'input> for Library_configContext<'input>{}

pub struct Library_configContextExt<'input>{
	base:Library_expContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{Library_configContextExt<'a>}

impl<'input> DesignParserContext<'input> for Library_configContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Library_configContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_library_config(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_library_config(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Library_configContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_library_config(self);
	}
}

impl<'input> CustomRuleContext<'input> for Library_configContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_library_exp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_library_exp }
}

impl<'input> Borrow<Library_expContextExt<'input>> for Library_configContext<'input>{
	fn borrow(&self) -> &Library_expContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Library_expContextExt<'input>> for Library_configContext<'input>{
	fn borrow_mut(&mut self) -> &mut Library_expContextExt<'input> { &mut self.base }
}

impl<'input> Library_expContextAttrs<'input> for Library_configContext<'input> {}

impl<'input> Library_configContextExt<'input>{
	fn new(ctx: &dyn Library_expContextAttrs<'input>) -> Rc<Library_expContextAll<'input>>  {
		Rc::new(
			Library_expContextAll::Library_configContext(
				BaseParserRuleContext::copy_from(ctx,Library_configContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn library_exp(&mut self,)
	-> Result<Rc<Library_expContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Library_expContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_library_exp);
        let mut _localctx: Rc<Library_expContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(327);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(29,&mut recog.base)? {
				1 =>{
					let tmp = Library_configContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					/*InvokeRule preset_key*/
					recog.base.set_state(308);
					recog.preset_key()?;

					recog.base.set_state(309);
					recog.base.match_token(T__6,&mut recog.err_handler)?;

					recog.base.set_state(312);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 STRING_LITERAL | IDENTIFIER | DIGITS_IDENTIFIER | DECIMAL_LITERAL |
					 FLOAT_LITERAL 
						=> {
							{
							/*InvokeRule preset_value*/
							recog.base.set_state(310);
							recog.preset_value()?;

							}
						}

					 LBRACK 
						=> {
							{
							/*InvokeRule preset_array*/
							recog.base.set_state(311);
							recog.preset_array()?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					recog.base.set_state(315);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__5 {
						{
						recog.base.set_state(314);
						recog.base.match_token(T__5,&mut recog.err_handler)?;

						}
					}

					}
				}
			,
				2 =>{
					let tmp = Library_objectContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					/*InvokeRule preset_key*/
					recog.base.set_state(317);
					recog.preset_key()?;

					recog.base.set_state(318);
					recog.base.match_token(LBRACE,&mut recog.err_handler)?;

					recog.base.set_state(322);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==IDENTIFIER {
						{
						{
						/*InvokeRule keyValue*/
						recog.base.set_state(319);
						recog.keyValue()?;

						}
						}
						recog.base.set_state(324);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(325);
					recog.base.match_token(RBRACE,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- keyValue ----------------
pub type KeyValueContextAll<'input> = KeyValueContext<'input>;


pub type KeyValueContext<'input> = BaseParserRuleContext<'input,KeyValueContextExt<'input>>;

#[derive(Clone)]
pub struct KeyValueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for KeyValueContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for KeyValueContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_keyValue(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_keyValue(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for KeyValueContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_keyValue(self);
	}
}

impl<'input> CustomRuleContext<'input> for KeyValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_keyValue }
	//fn type_rule_index() -> usize where Self: Sized { RULE_keyValue }
}
antlr_rust::type_id!{KeyValueContextExt<'a>}

impl<'input> KeyValueContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<KeyValueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,KeyValueContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait KeyValueContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<KeyValueContextExt<'input>>{

fn preset_key(&self) -> Option<Rc<Preset_keyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn preset_value(&self) -> Option<Rc<Preset_valueContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> KeyValueContextAttrs<'input> for KeyValueContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn keyValue(&mut self,)
	-> Result<Rc<KeyValueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = KeyValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 70, RULE_keyValue);
        let mut _localctx: Rc<KeyValueContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule preset_key*/
			recog.base.set_state(329);
			recog.preset_key()?;

			recog.base.set_state(330);
			recog.base.match_token(T__6,&mut recog.err_handler)?;

			/*InvokeRule preset_value*/
			recog.base.set_state(331);
			recog.preset_value()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- preset_key ----------------
pub type Preset_keyContextAll<'input> = Preset_keyContext<'input>;


pub type Preset_keyContext<'input> = BaseParserRuleContext<'input,Preset_keyContextExt<'input>>;

#[derive(Clone)]
pub struct Preset_keyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Preset_keyContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Preset_keyContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_preset_key(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_preset_key(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Preset_keyContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_preset_key(self);
	}
}

impl<'input> CustomRuleContext<'input> for Preset_keyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_preset_key }
	//fn type_rule_index() -> usize where Self: Sized { RULE_preset_key }
}
antlr_rust::type_id!{Preset_keyContextExt<'a>}

impl<'input> Preset_keyContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Preset_keyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Preset_keyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Preset_keyContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Preset_keyContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> Preset_keyContextAttrs<'input> for Preset_keyContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn preset_key(&mut self,)
	-> Result<Rc<Preset_keyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Preset_keyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_preset_key);
        let mut _localctx: Rc<Preset_keyContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(333);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- preset_value ----------------
pub type Preset_valueContextAll<'input> = Preset_valueContext<'input>;


pub type Preset_valueContext<'input> = BaseParserRuleContext<'input,Preset_valueContextExt<'input>>;

#[derive(Clone)]
pub struct Preset_valueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Preset_valueContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Preset_valueContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_preset_value(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_preset_value(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Preset_valueContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_preset_value(self);
	}
}

impl<'input> CustomRuleContext<'input> for Preset_valueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_preset_value }
	//fn type_rule_index() -> usize where Self: Sized { RULE_preset_value }
}
antlr_rust::type_id!{Preset_valueContextExt<'a>}

impl<'input> Preset_valueContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Preset_valueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Preset_valueContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Preset_valueContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Preset_valueContextExt<'input>>{

fn config_value(&self) -> Option<Rc<Config_valueContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Preset_valueContextAttrs<'input> for Preset_valueContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn preset_value(&mut self,)
	-> Result<Rc<Preset_valueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Preset_valueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 74, RULE_preset_value);
        let mut _localctx: Rc<Preset_valueContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule config_value*/
			recog.base.set_state(335);
			recog.config_value()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- preset_array ----------------
pub type Preset_arrayContextAll<'input> = Preset_arrayContext<'input>;


pub type Preset_arrayContext<'input> = BaseParserRuleContext<'input,Preset_arrayContextExt<'input>>;

#[derive(Clone)]
pub struct Preset_arrayContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Preset_arrayContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Preset_arrayContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_preset_array(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_preset_array(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Preset_arrayContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_preset_array(self);
	}
}

impl<'input> CustomRuleContext<'input> for Preset_arrayContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_preset_array }
	//fn type_rule_index() -> usize where Self: Sized { RULE_preset_array }
}
antlr_rust::type_id!{Preset_arrayContextExt<'a>}

impl<'input> Preset_arrayContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Preset_arrayContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Preset_arrayContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Preset_arrayContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Preset_arrayContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACK
/// Returns `None` if there is no child corresponding to token LBRACK
fn LBRACK(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(LBRACK, 0)
}
fn preset_call_all(&self) ->  Vec<Rc<Preset_callContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn preset_call(&self, i: usize) -> Option<Rc<Preset_callContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token RBRACK
/// Returns `None` if there is no child corresponding to token RBRACK
fn RBRACK(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(RBRACK, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,DesignParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> Preset_arrayContextAttrs<'input> for Preset_arrayContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn preset_array(&mut self,)
	-> Result<Rc<Preset_arrayContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Preset_arrayContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 76, RULE_preset_array);
        let mut _localctx: Rc<Preset_arrayContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(337);
			recog.base.match_token(LBRACK,&mut recog.err_handler)?;

			/*InvokeRule preset_call*/
			recog.base.set_state(338);
			recog.preset_call()?;

			recog.base.set_state(343);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(339);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule preset_call*/
				recog.base.set_state(340);
				recog.preset_call()?;

				}
				}
				recog.base.set_state(345);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(346);
			recog.base.match_token(RBRACK,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- preset_call ----------------
pub type Preset_callContextAll<'input> = Preset_callContext<'input>;


pub type Preset_callContext<'input> = BaseParserRuleContext<'input,Preset_callContextExt<'input>>;

#[derive(Clone)]
pub struct Preset_callContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Preset_callContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Preset_callContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_preset_call(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_preset_call(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Preset_callContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_preset_call(self);
	}
}

impl<'input> CustomRuleContext<'input> for Preset_callContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_preset_call }
	//fn type_rule_index() -> usize where Self: Sized { RULE_preset_call }
}
antlr_rust::type_id!{Preset_callContextExt<'a>}

impl<'input> Preset_callContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Preset_callContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Preset_callContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Preset_callContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Preset_callContextExt<'input>>{

fn library_name(&self) -> Option<Rc<Library_nameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> Preset_callContextAttrs<'input> for Preset_callContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn preset_call(&mut self,)
	-> Result<Rc<Preset_callContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Preset_callContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 78, RULE_preset_call);
        let mut _localctx: Rc<Preset_callContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule library_name*/
			recog.base.set_state(348);
			recog.library_name()?;

			recog.base.set_state(349);
			recog.base.match_token(DOT,&mut recog.err_handler)?;

			recog.base.set_state(350);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- library_name ----------------
pub type Library_nameContextAll<'input> = Library_nameContext<'input>;


pub type Library_nameContext<'input> = BaseParserRuleContext<'input,Library_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Library_nameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for Library_nameContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for Library_nameContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_library_name(self);
	}
	fn exit(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.exit_library_name(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn DesignVisitor<'input> + 'a> for Library_nameContext<'input>{
	fn accept(&self,visitor: &mut (dyn DesignVisitor<'input> + 'a)) {
		visitor.visit_library_name(self);
	}
}

impl<'input> CustomRuleContext<'input> for Library_nameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_library_name }
	//fn type_rule_index() -> usize where Self: Sized { RULE_library_name }
}
antlr_rust::type_id!{Library_nameContextExt<'a>}

impl<'input> Library_nameContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Library_nameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Library_nameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Library_nameContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<Library_nameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> Library_nameContextAttrs<'input> for Library_nameContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn library_name(&mut self,)
	-> Result<Rc<Library_nameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Library_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 80, RULE_library_name);
        let mut _localctx: Rc<Library_nameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(352);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x31\u{165}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x04\
	\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\x20\x09\
	\x20\x04\x21\x09\x21\x04\x22\x09\x22\x04\x23\x09\x23\x04\x24\x09\x24\x04\
	\x25\x09\x25\x04\x26\x09\x26\x04\x27\x09\x27\x04\x28\x09\x28\x04\x29\x09\
	\x29\x04\x2a\x09\x2a\x03\x02\x03\x02\x07\x02\x57\x0a\x02\x0c\x02\x0e\x02\
	\x5a\x0b\x02\x03\x02\x03\x02\x03\x03\x03\x03\x03\x04\x03\x04\x03\x04\x03\
	\x04\x03\x05\x03\x05\x03\x06\x03\x06\x05\x06\x68\x0a\x06\x03\x06\x03\x06\
	\x05\x06\x6c\x0a\x06\x03\x06\x03\x06\x05\x06\x70\x0a\x06\x03\x06\x03\x06\
	\x03\x06\x05\x06\x75\x0a\x06\x03\x06\x05\x06\x78\x0a\x06\x03\x07\x03\x07\
	\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x05\x08\u{83}\x0a\
	\x08\x03\x09\x03\x09\x03\x09\x03\x09\x07\x09\u{89}\x0a\x09\x0c\x09\x0e\x09\
	\u{8c}\x0b\x09\x03\x09\x03\x09\x03\x0a\x03\x0a\x03\x0a\x05\x0a\u{93}\x0a\
	\x0a\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x05\x0b\u{9a}\x0a\x0b\x03\x0c\
	\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0d\x03\x0d\
	\x05\x0d\u{a6}\x0a\x0d\x03\x0d\x03\x0d\x03\x0d\x05\x0d\u{ab}\x0a\x0d\x03\
	\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0f\x03\x0f\x05\x0f\u{b5}\
	\x0a\x0f\x03\x10\x03\x10\x03\x10\x03\x11\x03\x11\x03\x11\x03\x11\x03\x11\
	\x03\x12\x03\x12\x03\x13\x03\x13\x03\x14\x03\x14\x03\x15\x03\x15\x03\x16\
	\x03\x16\x03\x17\x03\x17\x03\x17\x03\x17\x07\x17\u{cd}\x0a\x17\x0c\x17\x0e\
	\x17\u{d0}\x0b\x17\x03\x17\x03\x17\x03\x18\x03\x18\x03\x18\x03\x18\x07\x18\
	\u{d8}\x0a\x18\x0c\x18\x0e\x18\u{db}\x0b\x18\x03\x18\x03\x18\x03\x19\x03\
	\x19\x03\x19\x07\x19\u{e2}\x0a\x19\x0c\x19\x0e\x19\u{e5}\x0b\x19\x03\x19\
	\x03\x19\x03\x19\x03\x19\x05\x19\u{eb}\x0a\x19\x03\x1a\x03\x1a\x03\x1a\x03\
	\x1a\x07\x1a\u{f1}\x0a\x1a\x0c\x1a\x0e\x1a\u{f4}\x0b\x1a\x03\x1a\x03\x1a\
	\x03\x1b\x03\x1b\x07\x1b\u{fa}\x0a\x1b\x0c\x1b\x0e\x1b\u{fd}\x0b\x1b\x03\
	\x1b\x03\x1b\x03\x1b\x05\x1b\u{102}\x0a\x1b\x03\x1c\x03\x1c\x07\x1c\u{106}\
	\x0a\x1c\x0c\x1c\x0e\x1c\u{109}\x0b\x1c\x03\x1d\x03\x1d\x03\x1d\x03\x1e\
	\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x05\x1e\u{115}\x0a\x1e\
	\x03\x1e\x05\x1e\u{118}\x0a\x1e\x03\x1f\x03\x1f\x03\x20\x03\x20\x03\x20\
	\x03\x20\x03\x20\x03\x20\x03\x21\x03\x21\x03\x22\x03\x22\x03\x22\x07\x22\
	\u{127}\x0a\x22\x0c\x22\x0e\x22\u{12a}\x0b\x22\x03\x23\x03\x23\x03\x23\x03\
	\x23\x07\x23\u{130}\x0a\x23\x0c\x23\x0e\x23\u{133}\x0b\x23\x03\x23\x03\x23\
	\x03\x24\x03\x24\x03\x24\x03\x24\x05\x24\u{13b}\x0a\x24\x03\x24\x05\x24\
	\u{13e}\x0a\x24\x03\x24\x03\x24\x03\x24\x07\x24\u{143}\x0a\x24\x0c\x24\x0e\
	\x24\u{146}\x0b\x24\x03\x24\x03\x24\x05\x24\u{14a}\x0a\x24\x03\x25\x03\x25\
	\x03\x25\x03\x25\x03\x26\x03\x26\x03\x27\x03\x27\x03\x28\x03\x28\x03\x28\
	\x03\x28\x07\x28\u{158}\x0a\x28\x0c\x28\x0e\x28\u{15b}\x0b\x28\x03\x28\x03\
	\x28\x03\x29\x03\x29\x03\x29\x03\x29\x03\x2a\x03\x2a\x03\x2a\x02\x02\x2b\
	\x02\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\x24\
	\x26\x28\x2a\x2c\x2e\x30\x32\x34\x36\x38\x3a\x3c\x3e\x40\x42\x44\x46\x48\
	\x4a\x4c\x4e\x50\x52\x02\x04\x03\x02\x03\x05\x05\x02\x15\x15\x1a\x1a\x2f\
	\x2f\x02\u{165}\x02\x58\x03\x02\x02\x02\x04\x5d\x03\x02\x02\x02\x06\x5f\
	\x03\x02\x02\x02\x08\x63\x03\x02\x02\x02\x0a\x77\x03\x02\x02\x02\x0c\x79\
	\x03\x02\x02\x02\x0e\u{82}\x03\x02\x02\x02\x10\u{84}\x03\x02\x02\x02\x12\
	\u{92}\x03\x02\x02\x02\x14\u{94}\x03\x02\x02\x02\x16\u{9b}\x03\x02\x02\x02\
	\x18\u{a3}\x03\x02\x02\x02\x1a\u{ac}\x03\x02\x02\x02\x1c\u{b4}\x03\x02\x02\
	\x02\x1e\u{b6}\x03\x02\x02\x02\x20\u{b9}\x03\x02\x02\x02\x22\u{be}\x03\x02\
	\x02\x02\x24\u{c0}\x03\x02\x02\x02\x26\u{c2}\x03\x02\x02\x02\x28\u{c4}\x03\
	\x02\x02\x02\x2a\u{c6}\x03\x02\x02\x02\x2c\u{c8}\x03\x02\x02\x02\x2e\u{d3}\
	\x03\x02\x02\x02\x30\u{ea}\x03\x02\x02\x02\x32\u{ec}\x03\x02\x02\x02\x34\
	\u{101}\x03\x02\x02\x02\x36\u{103}\x03\x02\x02\x02\x38\u{10a}\x03\x02\x02\
	\x02\x3a\u{117}\x03\x02\x02\x02\x3c\u{119}\x03\x02\x02\x02\x3e\u{11b}\x03\
	\x02\x02\x02\x40\u{121}\x03\x02\x02\x02\x42\u{128}\x03\x02\x02\x02\x44\u{12b}\
	\x03\x02\x02\x02\x46\u{149}\x03\x02\x02\x02\x48\u{14b}\x03\x02\x02\x02\x4a\
	\u{14f}\x03\x02\x02\x02\x4c\u{151}\x03\x02\x02\x02\x4e\u{153}\x03\x02\x02\
	\x02\x50\u{15e}\x03\x02\x02\x02\x52\u{162}\x03\x02\x02\x02\x54\x57\x05\x04\
	\x03\x02\x55\x57\x05\x0e\x08\x02\x56\x54\x03\x02\x02\x02\x56\x55\x03\x02\
	\x02\x02\x57\x5a\x03\x02\x02\x02\x58\x56\x03\x02\x02\x02\x58\x59\x03\x02\
	\x02\x02\x59\x5b\x03\x02\x02\x02\x5a\x58\x03\x02\x02\x02\x5b\x5c\x07\x02\
	\x02\x03\x5c\x03\x03\x02\x02\x02\x5d\x5e\x07\x2d\x02\x02\x5e\x05\x03\x02\
	\x02\x02\x5f\x60\x05\x08\x05\x02\x60\x61\x07\x29\x02\x02\x61\x62\x05\x0a\
	\x06\x02\x62\x07\x03\x02\x02\x02\x63\x64\x07\x2d\x02\x02\x64\x09\x03\x02\
	\x02\x02\x65\x67\x07\x2f\x02\x02\x66\x68\x05\x0c\x07\x02\x67\x66\x03\x02\
	\x02\x02\x67\x68\x03\x02\x02\x02\x68\x78\x03\x02\x02\x02\x69\x6b\x07\x30\
	\x02\x02\x6a\x6c\x05\x0c\x07\x02\x6b\x6a\x03\x02\x02\x02\x6b\x6c\x03\x02\
	\x02\x02\x6c\x78\x03\x02\x02\x02\x6d\x6f\x07\x31\x02\x02\x6e\x70\x05\x0c\
	\x07\x02\x6f\x6e\x03\x02\x02\x02\x6f\x70\x03\x02\x02\x02\x70\x78\x03\x02\
	\x02\x02\x71\x74\x07\x2d\x02\x02\x72\x73\x07\x2b\x02\x02\x73\x75\x07\x2d\
	\x02\x02\x74\x72\x03\x02\x02\x02\x74\x75\x03\x02\x02\x02\x75\x78\x03\x02\
	\x02\x02\x76\x78\x07\x1a\x02\x02\x77\x65\x03\x02\x02\x02\x77\x69\x03\x02\
	\x02\x02\x77\x6d\x03\x02\x02\x02\x77\x71\x03\x02\x02\x02\x77\x76\x03\x02\
	\x02\x02\x78\x0b\x03\x02\x02\x02\x79\x7a\x09\x02\x02\x02\x7a\x0d\x03\x02\
	\x02\x02\x7b\u{83}\x05\x06\x04\x02\x7c\u{83}\x05\x10\x09\x02\x7d\u{83}\x05\
	\x2c\x17\x02\x7e\u{83}\x05\x3e\x20\x02\x7f\u{83}\x05\x2e\x18\x02\u{80}\u{83}\
	\x05\x44\x23\x02\u{81}\u{83}\x05\x32\x1a\x02\u{82}\x7b\x03\x02\x02\x02\u{82}\
	\x7c\x03\x02\x02\x02\u{82}\x7d\x03\x02\x02\x02\u{82}\x7e\x03\x02\x02\x02\
	\u{82}\x7f\x03\x02\x02\x02\u{82}\u{80}\x03\x02\x02\x02\u{82}\u{81}\x03\x02\
	\x02\x02\u{83}\x0f\x03\x02\x02\x02\u{84}\u{85}\x07\x0c\x02\x02\u{85}\u{86}\
	\x07\x2d\x02\x02\u{86}\u{8a}\x07\x23\x02\x02\u{87}\u{89}\x05\x12\x0a\x02\
	\u{88}\u{87}\x03\x02\x02\x02\u{89}\u{8c}\x03\x02\x02\x02\u{8a}\u{88}\x03\
	\x02\x02\x02\u{8a}\u{8b}\x03\x02\x02\x02\u{8b}\u{8d}\x03\x02\x02\x02\u{8c}\
	\u{8a}\x03\x02\x02\x02\u{8d}\u{8e}\x07\x24\x02\x02\u{8e}\x11\x03\x02\x02\
	\x02\u{8f}\u{93}\x05\x14\x0b\x02\u{90}\u{93}\x05\x16\x0c\x02\u{91}\u{93}\
	\x05\x18\x0d\x02\u{92}\u{8f}\x03\x02\x02\x02\u{92}\u{90}\x03\x02\x02\x02\
	\u{92}\u{91}\x03\x02\x02\x02\u{93}\x13\x03\x02\x02\x02\u{94}\u{99}\x07\x0d\
	\x02\x02\u{95}\u{9a}\x07\x2d\x02\x02\u{96}\u{97}\x07\x1a\x02\x02\u{97}\u{98}\
	\x07\x2a\x02\x02\u{98}\u{9a}\x05\x26\x14\x02\u{99}\u{95}\x03\x02\x02\x02\
	\u{99}\u{96}\x03\x02\x02\x02\u{9a}\x15\x03\x02\x02\x02\u{9b}\u{9c}\x07\x0e\
	\x02\x02\u{9c}\u{9d}\x07\x25\x02\x02\u{9d}\u{9e}\x05\x22\x12\x02\u{9e}\u{9f}\
	\x07\x26\x02\x02\u{9f}\u{a0}\x07\x1a\x02\x02\u{a0}\u{a1}\x07\x2a\x02\x02\
	\u{a1}\u{a2}\x05\x26\x14\x02\u{a2}\x17\x03\x02\x02\x02\u{a3}\u{a5}\x07\x0f\
	\x02\x02\u{a4}\u{a6}\x05\x28\x15\x02\u{a5}\u{a4}\x03\x02\x02\x02\u{a5}\u{a6}\
	\x03\x02\x02\x02\u{a6}\u{a7}\x03\x02\x02\x02\u{a7}\u{a8}\x07\x29\x02\x02\
	\u{a8}\u{aa}\x05\x1c\x0f\x02\u{a9}\u{ab}\x05\x1a\x0e\x02\u{aa}\u{a9}\x03\
	\x02\x02\x02\u{aa}\u{ab}\x03\x02\x02\x02\u{ab}\x19\x03\x02\x02\x02\u{ac}\
	\u{ad}\x07\x10\x02\x02\u{ad}\u{ae}\x07\x11\x02\x02\u{ae}\u{af}\x07\x21\x02\
	\x02\u{af}\u{b0}\x05\x2a\x16\x02\u{b0}\u{b1}\x07\x22\x02\x02\u{b1}\x1b\x03\
	\x02\x02\x02\u{b2}\u{b5}\x05\x1e\x10\x02\u{b3}\u{b5}\x05\x20\x11\x02\u{b4}\
	\u{b2}\x03\x02\x02\x02\u{b4}\u{b3}\x03\x02\x02\x02\u{b5}\x1d\x03\x02\x02\
	\x02\u{b6}\u{b7}\x07\x0a\x02\x02\u{b7}\u{b8}\x05\x26\x14\x02\u{b8}\x1f\x03\
	\x02\x02\x02\u{b9}\u{ba}\x07\x0b\x02\x02\u{ba}\u{bb}\x07\x1a\x02\x02\u{bb}\
	\u{bc}\x07\x2a\x02\x02\u{bc}\u{bd}\x05\x26\x14\x02\u{bd}\x21\x03\x02\x02\
	\x02\u{be}\u{bf}\x07\x2d\x02\x02\u{bf}\x23\x03\x02\x02\x02\u{c0}\u{c1}\x07\
	\x2d\x02\x02\u{c1}\x25\x03\x02\x02\x02\u{c2}\u{c3}\x07\x2d\x02\x02\u{c3}\
	\x27\x03\x02\x02\x02\u{c4}\u{c5}\x07\x2d\x02\x02\u{c5}\x29\x03\x02\x02\x02\
	\u{c6}\u{c7}\x07\x2d\x02\x02\u{c7}\x2b\x03\x02\x02\x02\u{c8}\u{c9}\x07\x16\
	\x02\x02\u{c9}\u{ca}\x07\x2d\x02\x02\u{ca}\u{ce}\x07\x23\x02\x02\u{cb}\u{cd}\
	\x05\x30\x19\x02\u{cc}\u{cb}\x03\x02\x02\x02\u{cd}\u{d0}\x03\x02\x02\x02\
	\u{ce}\u{cc}\x03\x02\x02\x02\u{ce}\u{cf}\x03\x02\x02\x02\u{cf}\u{d1}\x03\
	\x02\x02\x02\u{d0}\u{ce}\x03\x02\x02\x02\u{d1}\u{d2}\x07\x24\x02\x02\u{d2}\
	\x2d\x03\x02\x02\x02\u{d3}\u{d4}\x07\x17\x02\x02\u{d4}\u{d5}\x07\x2d\x02\
	\x02\u{d5}\u{d9}\x07\x23\x02\x02\u{d6}\u{d8}\x05\x30\x19\x02\u{d7}\u{d6}\
	\x03\x02\x02\x02\u{d8}\u{db}\x03\x02\x02\x02\u{d9}\u{d7}\x03\x02\x02\x02\
	\u{d9}\u{da}\x03\x02\x02\x02\u{da}\u{dc}\x03\x02\x02\x02\u{db}\u{d9}\x03\
	\x02\x02\x02\u{dc}\u{dd}\x07\x24\x02\x02\u{dd}\x2f\x03\x02\x02\x02\u{de}\
	\u{e3}\x05\x26\x14\x02\u{df}\u{e0}\x07\x2b\x02\x02\u{e0}\u{e2}\x05\x26\x14\
	\x02\u{e1}\u{df}\x03\x02\x02\x02\u{e2}\u{e5}\x03\x02\x02\x02\u{e3}\u{e1}\
	\x03\x02\x02\x02\u{e3}\u{e4}\x03\x02\x02\x02\u{e4}\u{eb}\x03\x02\x02\x02\
	\u{e5}\u{e3}\x03\x02\x02\x02\u{e6}\u{e7}\x05\x08\x05\x02\u{e7}\u{e8}\x07\
	\x29\x02\x02\u{e8}\u{e9}\x05\x0a\x06\x02\u{e9}\u{eb}\x03\x02\x02\x02\u{ea}\
	\u{de}\x03\x02\x02\x02\u{ea}\u{e6}\x03\x02\x02\x02\u{eb}\x31\x03\x02\x02\
	\x02\u{ec}\u{ed}\x07\x14\x02\x02\u{ed}\u{ee}\x07\x2d\x02\x02\u{ee}\u{f2}\
	\x07\x23\x02\x02\u{ef}\u{f1}\x05\x34\x1b\x02\u{f0}\u{ef}\x03\x02\x02\x02\
	\u{f1}\u{f4}\x03\x02\x02\x02\u{f2}\u{f0}\x03\x02\x02\x02\u{f2}\u{f3}\x03\
	\x02\x02\x02\u{f3}\u{f5}\x03\x02\x02\x02\u{f4}\u{f2}\x03\x02\x02\x02\u{f5}\
	\u{f6}\x07\x24\x02\x02\u{f6}\x33\x03\x02\x02\x02\u{f7}\u{fb}\x07\x06\x02\
	\x02\u{f8}\u{fa}\x07\x06\x02\x02\u{f9}\u{f8}\x03\x02\x02\x02\u{fa}\u{fd}\
	\x03\x02\x02\x02\u{fb}\u{f9}\x03\x02\x02\x02\u{fb}\u{fc}\x03\x02\x02\x02\
	\u{fc}\u{102}\x03\x02\x02\x02\u{fd}\u{fb}\x03\x02\x02\x02\u{fe}\u{ff}\x05\
	\x36\x1c\x02\u{ff}\u{100}\x07\x07\x02\x02\u{100}\u{102}\x03\x02\x02\x02\
	\u{101}\u{f7}\x03\x02\x02\x02\u{101}\u{fe}\x03\x02\x02\x02\u{102}\x35\x03\
	\x02\x02\x02\u{103}\u{107}\x05\x38\x1d\x02\u{104}\u{106}\x05\x38\x1d\x02\
	\u{105}\u{104}\x03\x02\x02\x02\u{106}\u{109}\x03\x02\x02\x02\u{107}\u{105}\
	\x03\x02\x02\x02\u{107}\u{108}\x03\x02\x02\x02\u{108}\x37\x03\x02\x02\x02\
	\u{109}\u{107}\x03\x02\x02\x02\u{10a}\u{10b}\x07\x07\x02\x02\u{10b}\u{10c}\
	\x05\x3a\x1e\x02\u{10c}\x39\x03\x02\x02\x02\u{10d}\u{118}\x07\x30\x02\x02\
	\u{10e}\u{118}\x07\x15\x02\x02\u{10f}\u{114}\x05\x26\x14\x02\u{110}\u{111}\
	\x07\x21\x02\x02\u{111}\u{112}\x05\x3c\x1f\x02\u{112}\u{113}\x07\x22\x02\
	\x02\u{113}\u{115}\x03\x02\x02\x02\u{114}\u{110}\x03\x02\x02\x02\u{114}\
	\u{115}\x03\x02\x02\x02\u{115}\u{118}\x03\x02\x02\x02\u{116}\u{118}\x07\
	\x1a\x02\x02\u{117}\u{10d}\x03\x02\x02\x02\u{117}\u{10e}\x03\x02\x02\x02\
	\u{117}\u{10f}\x03\x02\x02\x02\u{117}\u{116}\x03\x02\x02\x02\u{118}\x3b\
	\x03\x02\x02\x02\u{119}\u{11a}\x09\x03\x02\x02\u{11a}\x3d\x03\x02\x02\x02\
	\u{11b}\u{11c}\x07\x18\x02\x02\u{11c}\u{11d}\x05\x40\x21\x02\u{11d}\u{11e}\
	\x07\x23\x02\x02\u{11e}\u{11f}\x05\x42\x22\x02\u{11f}\u{120}\x07\x24\x02\
	\x02\u{120}\x3f\x03\x02\x02\x02\u{121}\u{122}\x07\x2d\x02\x02\u{122}\x41\
	\x03\x02\x02\x02\u{123}\u{124}\x05\x06\x04\x02\u{124}\u{125}\x07\x08\x02\
	\x02\u{125}\u{127}\x03\x02\x02\x02\u{126}\u{123}\x03\x02\x02\x02\u{127}\
	\u{12a}\x03\x02\x02\x02\u{128}\u{126}\x03\x02\x02\x02\u{128}\u{129}\x03\
	\x02\x02\x02\u{129}\x43\x03\x02\x02\x02\u{12a}\u{128}\x03\x02\x02\x02\u{12b}\
	\u{12c}\x07\x19\x02\x02\u{12c}\u{12d}\x05\x52\x2a\x02\u{12d}\u{131}\x07\
	\x23\x02\x02\u{12e}\u{130}\x05\x46\x24\x02\u{12f}\u{12e}\x03\x02\x02\x02\
	\u{130}\u{133}\x03\x02\x02\x02\u{131}\u{12f}\x03\x02\x02\x02\u{131}\u{132}\
	\x03\x02\x02\x02\u{132}\u{134}\x03\x02\x02\x02\u{133}\u{131}\x03\x02\x02\
	\x02\u{134}\u{135}\x07\x24\x02\x02\u{135}\x45\x03\x02\x02\x02\u{136}\u{137}\
	\x05\x4a\x26\x02\u{137}\u{13a}\x07\x09\x02\x02\u{138}\u{13b}\x05\x4c\x27\
	\x02\u{139}\u{13b}\x05\x4e\x28\x02\u{13a}\u{138}\x03\x02\x02\x02\u{13a}\
	\u{139}\x03\x02\x02\x02\u{13b}\u{13d}\x03\x02\x02\x02\u{13c}\u{13e}\x07\
	\x08\x02\x02\u{13d}\u{13c}\x03\x02\x02\x02\u{13d}\u{13e}\x03\x02\x02\x02\
	\u{13e}\u{14a}\x03\x02\x02\x02\u{13f}\u{140}\x05\x4a\x26\x02\u{140}\u{144}\
	\x07\x23\x02\x02\u{141}\u{143}\x05\x48\x25\x02\u{142}\u{141}\x03\x02\x02\
	\x02\u{143}\u{146}\x03\x02\x02\x02\u{144}\u{142}\x03\x02\x02\x02\u{144}\
	\u{145}\x03\x02\x02\x02\u{145}\u{147}\x03\x02\x02\x02\u{146}\u{144}\x03\
	\x02\x02\x02\u{147}\u{148}\x07\x24\x02\x02\u{148}\u{14a}\x03\x02\x02\x02\
	\u{149}\u{136}\x03\x02\x02\x02\u{149}\u{13f}\x03\x02\x02\x02\u{14a}\x47\
	\x03\x02\x02\x02\u{14b}\u{14c}\x05\x4a\x26\x02\u{14c}\u{14d}\x07\x09\x02\
	\x02\u{14d}\u{14e}\x05\x4c\x27\x02\u{14e}\x49\x03\x02\x02\x02\u{14f}\u{150}\
	\x07\x2d\x02\x02\u{150}\x4b\x03\x02\x02\x02\u{151}\u{152}\x05\x0a\x06\x02\
	\u{152}\x4d\x03\x02\x02\x02\u{153}\u{154}\x07\x25\x02\x02\u{154}\u{159}\
	\x05\x50\x29\x02\u{155}\u{156}\x07\x2b\x02\x02\u{156}\u{158}\x05\x50\x29\
	\x02\u{157}\u{155}\x03\x02\x02\x02\u{158}\u{15b}\x03\x02\x02\x02\u{159}\
	\u{157}\x03\x02\x02\x02\u{159}\u{15a}\x03\x02\x02\x02\u{15a}\u{15c}\x03\
	\x02\x02\x02\u{15b}\u{159}\x03\x02\x02\x02\u{15c}\u{15d}\x07\x26\x02\x02\
	\u{15d}\x4f\x03\x02\x02\x02\u{15e}\u{15f}\x05\x52\x2a\x02\u{15f}\u{160}\
	\x07\x2a\x02\x02\u{160}\u{161}\x07\x2d\x02\x02\u{161}\x51\x03\x02\x02\x02\
	\u{162}\u{163}\x07\x2d\x02\x02\u{163}\x53\x03\x02\x02\x02\x21\x56\x58\x67\
	\x6b\x6f\x74\x77\u{82}\u{8a}\u{92}\u{99}\u{a5}\u{aa}\u{b4}\u{ce}\u{d9}\u{e3}\
	\u{ea}\u{f2}\u{fb}\u{101}\u{107}\u{114}\u{117}\u{128}\u{131}\u{13a}\u{13d}\
	\u{144}\u{149}\u{159}";

