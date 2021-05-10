// Generated from docs/dsl/Design.g4 by ANTLR 4.8
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
use super::design_listener::*;
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
	pub const RULE_configDeclaration:usize = 2; 
	pub const RULE_configKey:usize = 3; 
	pub const RULE_configValue:usize = 4; 
	pub const RULE_unit:usize = 5; 
	pub const RULE_decalartions:usize = 6; 
	pub const RULE_flowDeclaration:usize = 7; 
	pub const RULE_interactionDeclaration:usize = 8; 
	pub const RULE_seeDeclaration:usize = 9; 
	pub const RULE_doDeclaration:usize = 10; 
	pub const RULE_reactDeclaration:usize = 11; 
	pub const RULE_animateDeclaration:usize = 12; 
	pub const RULE_reactAction:usize = 13; 
	pub const RULE_gotoAction:usize = 14; 
	pub const RULE_showAction:usize = 15; 
	pub const RULE_actionName:usize = 16; 
	pub const RULE_componentValue:usize = 17; 
	pub const RULE_componentName:usize = 18; 
	pub const RULE_sceneName:usize = 19; 
	pub const RULE_animateName:usize = 20; 
	pub const RULE_pageDeclaration:usize = 21; 
	pub const RULE_componentDeclaration:usize = 22; 
	pub const RULE_componentBodyDeclaration:usize = 23; 
	pub const RULE_layoutDeclaration:usize = 24; 
	pub const RULE_layoutRow:usize = 25; 
	pub const RULE_layoutLines:usize = 26; 
	pub const RULE_layoutLine:usize = 27; 
	pub const RULE_componentUseDeclaration:usize = 28; 
	pub const RULE_componentLayoutValue:usize = 29; 
	pub const RULE_styleDeclaration:usize = 30; 
	pub const RULE_styleName:usize = 31; 
	pub const RULE_styleBody:usize = 32; 
	pub const RULE_libraryDeclaration:usize = 33; 
	pub const RULE_libraryExpress:usize = 34; 
	pub const RULE_keyValue:usize = 35; 
	pub const RULE_presetKey:usize = 36; 
	pub const RULE_presetValue:usize = 37; 
	pub const RULE_presetArray:usize = 38; 
	pub const RULE_presetCall:usize = 39; 
	pub const RULE_libraryName:usize = 40;
	pub const ruleNames: [&'static str; 41] =  [
		"start", "comment", "configDeclaration", "configKey", "configValue", "unit", 
		"decalartions", "flowDeclaration", "interactionDeclaration", "seeDeclaration", 
		"doDeclaration", "reactDeclaration", "animateDeclaration", "reactAction", 
		"gotoAction", "showAction", "actionName", "componentValue", "componentName", 
		"sceneName", "animateName", "pageDeclaration", "componentDeclaration", 
		"componentBodyDeclaration", "layoutDeclaration", "layoutRow", "layoutLines", 
		"layoutLine", "componentUseDeclaration", "componentLayoutValue", "styleDeclaration", 
		"styleName", "styleBody", "libraryDeclaration", "libraryExpress", "keyValue", 
		"presetKey", "presetValue", "presetArray", "presetCall", "libraryName"
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
pub type LocalTokenFactory<'input> = CommonTokenFactory;

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
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=DesignParserContextType>
{}

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
fn configDeclaration_all(&self) ->  Vec<Rc<ConfigDeclarationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn configDeclaration(&self, i: usize) -> Option<Rc<ConfigDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn decalartions_all(&self) ->  Vec<Rc<DecalartionsContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn decalartions(&self, i: usize) -> Option<Rc<DecalartionsContextAll<'input>>> where Self:Sized{
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
			recog.base.set_state(87);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << FLOW) | (1usize << LAYOUT) | (1usize << PAGE) | (1usize << COMPONENT) | (1usize << STYLE) | (1usize << LIBRARY) | (1usize << IDENTIFIER))) != 0) {
				{
				recog.base.set_state(85);
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
						/*InvokeRule configDeclaration*/
						recog.base.set_state(83);
						recog.configDeclaration()?;

						}
					}
				,
					3 =>{
						{
						/*InvokeRule decalartions*/
						recog.base.set_state(84);
						recog.decalartions()?;

						}
					}

					_ => {}
				}
				}
				recog.base.set_state(89);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(90);
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
			recog.base.set_state(92);
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
//------------------- configDeclaration ----------------
pub type ConfigDeclarationContextAll<'input> = ConfigDeclarationContext<'input>;


pub type ConfigDeclarationContext<'input> = BaseParserRuleContext<'input,ConfigDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct ConfigDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for ConfigDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for ConfigDeclarationContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_configDeclaration(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConfigDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_configDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_configDeclaration }
}
antlr_rust::type_id!{ConfigDeclarationContextExt<'a>}

impl<'input> ConfigDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ConfigDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConfigDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ConfigDeclarationContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<ConfigDeclarationContextExt<'input>>{

fn configKey(&self) -> Option<Rc<ConfigKeyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn configValue(&self) -> Option<Rc<ConfigValueContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ConfigDeclarationContextAttrs<'input> for ConfigDeclarationContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn configDeclaration(&mut self,)
	-> Result<Rc<ConfigDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConfigDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_configDeclaration);
        let mut _localctx: Rc<ConfigDeclarationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule configKey*/
			recog.base.set_state(94);
			recog.configKey()?;

			recog.base.set_state(95);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule configValue*/
			recog.base.set_state(96);
			recog.configValue()?;

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
//------------------- configKey ----------------
pub type ConfigKeyContextAll<'input> = ConfigKeyContext<'input>;


pub type ConfigKeyContext<'input> = BaseParserRuleContext<'input,ConfigKeyContextExt<'input>>;

#[derive(Clone)]
pub struct ConfigKeyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for ConfigKeyContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for ConfigKeyContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_configKey(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConfigKeyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_configKey }
	//fn type_rule_index() -> usize where Self: Sized { RULE_configKey }
}
antlr_rust::type_id!{ConfigKeyContextExt<'a>}

impl<'input> ConfigKeyContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ConfigKeyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConfigKeyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ConfigKeyContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<ConfigKeyContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> ConfigKeyContextAttrs<'input> for ConfigKeyContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn configKey(&mut self,)
	-> Result<Rc<ConfigKeyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConfigKeyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_configKey);
        let mut _localctx: Rc<ConfigKeyContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(98);
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
//------------------- configValue ----------------
pub type ConfigValueContextAll<'input> = ConfigValueContext<'input>;


pub type ConfigValueContext<'input> = BaseParserRuleContext<'input,ConfigValueContextExt<'input>>;

#[derive(Clone)]
pub struct ConfigValueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for ConfigValueContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for ConfigValueContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_configValue(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConfigValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_configValue }
	//fn type_rule_index() -> usize where Self: Sized { RULE_configValue }
}
antlr_rust::type_id!{ConfigValueContextExt<'a>}

impl<'input> ConfigValueContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ConfigValueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConfigValueContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ConfigValueContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<ConfigValueContextExt<'input>>{

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

impl<'input> ConfigValueContextAttrs<'input> for ConfigValueContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn configValue(&mut self,)
	-> Result<Rc<ConfigValueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConfigValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_configValue);
        let mut _localctx: Rc<ConfigValueContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(118);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 DIGITS_IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(100);
					recog.base.match_token(DIGITS_IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(102);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__0) | (1usize << T__1) | (1usize << T__2))) != 0) {
						{
						/*InvokeRule unit*/
						recog.base.set_state(101);
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
					recog.base.set_state(104);
					recog.base.match_token(DECIMAL_LITERAL,&mut recog.err_handler)?;

					recog.base.set_state(106);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__0) | (1usize << T__1) | (1usize << T__2))) != 0) {
						{
						/*InvokeRule unit*/
						recog.base.set_state(105);
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
					recog.base.set_state(108);
					recog.base.match_token(FLOAT_LITERAL,&mut recog.err_handler)?;

					recog.base.set_state(110);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__0) | (1usize << T__1) | (1usize << T__2))) != 0) {
						{
						/*InvokeRule unit*/
						recog.base.set_state(109);
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
					recog.base.set_state(112);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(115);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==COMMA {
						{
						recog.base.set_state(113);
						recog.base.match_token(COMMA,&mut recog.err_handler)?;

						recog.base.set_state(114);
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
					recog.base.set_state(117);
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
			recog.base.set_state(120);
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
//------------------- decalartions ----------------
pub type DecalartionsContextAll<'input> = DecalartionsContext<'input>;


pub type DecalartionsContext<'input> = BaseParserRuleContext<'input,DecalartionsContextExt<'input>>;

#[derive(Clone)]
pub struct DecalartionsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for DecalartionsContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for DecalartionsContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_decalartions(self);
	}
}

impl<'input> CustomRuleContext<'input> for DecalartionsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_decalartions }
	//fn type_rule_index() -> usize where Self: Sized { RULE_decalartions }
}
antlr_rust::type_id!{DecalartionsContextExt<'a>}

impl<'input> DecalartionsContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DecalartionsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DecalartionsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DecalartionsContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<DecalartionsContextExt<'input>>{

fn configDeclaration(&self) -> Option<Rc<ConfigDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn flowDeclaration(&self) -> Option<Rc<FlowDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn pageDeclaration(&self) -> Option<Rc<PageDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn styleDeclaration(&self) -> Option<Rc<StyleDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn componentDeclaration(&self) -> Option<Rc<ComponentDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn libraryDeclaration(&self) -> Option<Rc<LibraryDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn layoutDeclaration(&self) -> Option<Rc<LayoutDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DecalartionsContextAttrs<'input> for DecalartionsContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn decalartions(&mut self,)
	-> Result<Rc<DecalartionsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DecalartionsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_decalartions);
        let mut _localctx: Rc<DecalartionsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(129);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule configDeclaration*/
					recog.base.set_state(122);
					recog.configDeclaration()?;

					}
				}

			 FLOW 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule flowDeclaration*/
					recog.base.set_state(123);
					recog.flowDeclaration()?;

					}
				}

			 PAGE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule pageDeclaration*/
					recog.base.set_state(124);
					recog.pageDeclaration()?;

					}
				}

			 STYLE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule styleDeclaration*/
					recog.base.set_state(125);
					recog.styleDeclaration()?;

					}
				}

			 COMPONENT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule componentDeclaration*/
					recog.base.set_state(126);
					recog.componentDeclaration()?;

					}
				}

			 LIBRARY 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule libraryDeclaration*/
					recog.base.set_state(127);
					recog.libraryDeclaration()?;

					}
				}

			 LAYOUT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule layoutDeclaration*/
					recog.base.set_state(128);
					recog.layoutDeclaration()?;

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
//------------------- flowDeclaration ----------------
pub type FlowDeclarationContextAll<'input> = FlowDeclarationContext<'input>;


pub type FlowDeclarationContext<'input> = BaseParserRuleContext<'input,FlowDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct FlowDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for FlowDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for FlowDeclarationContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_flowDeclaration(self);
	}
}

impl<'input> CustomRuleContext<'input> for FlowDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_flowDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_flowDeclaration }
}
antlr_rust::type_id!{FlowDeclarationContextExt<'a>}

impl<'input> FlowDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FlowDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FlowDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FlowDeclarationContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<FlowDeclarationContextExt<'input>>{

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
fn interactionDeclaration_all(&self) ->  Vec<Rc<InteractionDeclarationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn interactionDeclaration(&self, i: usize) -> Option<Rc<InteractionDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> FlowDeclarationContextAttrs<'input> for FlowDeclarationContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn flowDeclaration(&mut self,)
	-> Result<Rc<FlowDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FlowDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_flowDeclaration);
        let mut _localctx: Rc<FlowDeclarationContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(131);
			recog.base.match_token(FLOW,&mut recog.err_handler)?;

			recog.base.set_state(132);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(133);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(137);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << SEE) | (1usize << DO) | (1usize << REACT))) != 0) {
				{
				{
				/*InvokeRule interactionDeclaration*/
				recog.base.set_state(134);
				recog.interactionDeclaration()?;

				}
				}
				recog.base.set_state(139);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(140);
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
//------------------- interactionDeclaration ----------------
pub type InteractionDeclarationContextAll<'input> = InteractionDeclarationContext<'input>;


pub type InteractionDeclarationContext<'input> = BaseParserRuleContext<'input,InteractionDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct InteractionDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for InteractionDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for InteractionDeclarationContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_interactionDeclaration(self);
	}
}

impl<'input> CustomRuleContext<'input> for InteractionDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_interactionDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_interactionDeclaration }
}
antlr_rust::type_id!{InteractionDeclarationContextExt<'a>}

impl<'input> InteractionDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InteractionDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InteractionDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InteractionDeclarationContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<InteractionDeclarationContextExt<'input>>{

fn seeDeclaration(&self) -> Option<Rc<SeeDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn doDeclaration(&self) -> Option<Rc<DoDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn reactDeclaration(&self) -> Option<Rc<ReactDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> InteractionDeclarationContextAttrs<'input> for InteractionDeclarationContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn interactionDeclaration(&mut self,)
	-> Result<Rc<InteractionDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InteractionDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_interactionDeclaration);
        let mut _localctx: Rc<InteractionDeclarationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(145);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 SEE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule seeDeclaration*/
					recog.base.set_state(142);
					recog.seeDeclaration()?;

					}
				}

			 DO 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule doDeclaration*/
					recog.base.set_state(143);
					recog.doDeclaration()?;

					}
				}

			 REACT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule reactDeclaration*/
					recog.base.set_state(144);
					recog.reactDeclaration()?;

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
//------------------- seeDeclaration ----------------
pub type SeeDeclarationContextAll<'input> = SeeDeclarationContext<'input>;


pub type SeeDeclarationContext<'input> = BaseParserRuleContext<'input,SeeDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct SeeDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for SeeDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for SeeDeclarationContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_seeDeclaration(self);
	}
}

impl<'input> CustomRuleContext<'input> for SeeDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_seeDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_seeDeclaration }
}
antlr_rust::type_id!{SeeDeclarationContextExt<'a>}

impl<'input> SeeDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SeeDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SeeDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SeeDeclarationContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<SeeDeclarationContextExt<'input>>{

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
fn componentName(&self) -> Option<Rc<ComponentNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> SeeDeclarationContextAttrs<'input> for SeeDeclarationContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn seeDeclaration(&mut self,)
	-> Result<Rc<SeeDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SeeDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_seeDeclaration);
        let mut _localctx: Rc<SeeDeclarationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(147);
			recog.base.match_token(SEE,&mut recog.err_handler)?;

			recog.base.set_state(152);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IDENTIFIER 
				=> {
					{
					recog.base.set_state(148);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}

			 STRING_LITERAL 
				=> {
					{
					recog.base.set_state(149);
					recog.base.match_token(STRING_LITERAL,&mut recog.err_handler)?;

					recog.base.set_state(150);
					recog.base.match_token(DOT,&mut recog.err_handler)?;

					/*InvokeRule componentName*/
					recog.base.set_state(151);
					recog.componentName()?;

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
//------------------- doDeclaration ----------------
pub type DoDeclarationContextAll<'input> = DoDeclarationContext<'input>;


pub type DoDeclarationContext<'input> = BaseParserRuleContext<'input,DoDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct DoDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for DoDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for DoDeclarationContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_doDeclaration(self);
	}
}

impl<'input> CustomRuleContext<'input> for DoDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_doDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_doDeclaration }
}
antlr_rust::type_id!{DoDeclarationContextExt<'a>}

impl<'input> DoDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DoDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DoDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DoDeclarationContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<DoDeclarationContextExt<'input>>{

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
fn actionName(&self) -> Option<Rc<ActionNameContextAll<'input>>> where Self:Sized{
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
fn componentName(&self) -> Option<Rc<ComponentNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DoDeclarationContextAttrs<'input> for DoDeclarationContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn doDeclaration(&mut self,)
	-> Result<Rc<DoDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DoDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_doDeclaration);
        let mut _localctx: Rc<DoDeclarationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(154);
			recog.base.match_token(DO,&mut recog.err_handler)?;

			recog.base.set_state(155);
			recog.base.match_token(LBRACK,&mut recog.err_handler)?;

			/*InvokeRule actionName*/
			recog.base.set_state(156);
			recog.actionName()?;

			recog.base.set_state(157);
			recog.base.match_token(RBRACK,&mut recog.err_handler)?;

			recog.base.set_state(158);
			recog.base.match_token(STRING_LITERAL,&mut recog.err_handler)?;

			recog.base.set_state(159);
			recog.base.match_token(DOT,&mut recog.err_handler)?;

			/*InvokeRule componentName*/
			recog.base.set_state(160);
			recog.componentName()?;

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
//------------------- reactDeclaration ----------------
pub type ReactDeclarationContextAll<'input> = ReactDeclarationContext<'input>;


pub type ReactDeclarationContext<'input> = BaseParserRuleContext<'input,ReactDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct ReactDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for ReactDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for ReactDeclarationContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_reactDeclaration(self);
	}
}

impl<'input> CustomRuleContext<'input> for ReactDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_reactDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_reactDeclaration }
}
antlr_rust::type_id!{ReactDeclarationContextExt<'a>}

impl<'input> ReactDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ReactDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ReactDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ReactDeclarationContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<ReactDeclarationContextExt<'input>>{

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
fn reactAction(&self) -> Option<Rc<ReactActionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn sceneName(&self) -> Option<Rc<SceneNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn animateDeclaration(&self) -> Option<Rc<AnimateDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ReactDeclarationContextAttrs<'input> for ReactDeclarationContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn reactDeclaration(&mut self,)
	-> Result<Rc<ReactDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ReactDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_reactDeclaration);
        let mut _localctx: Rc<ReactDeclarationContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(162);
			recog.base.match_token(REACT,&mut recog.err_handler)?;

			recog.base.set_state(164);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==IDENTIFIER {
				{
				/*InvokeRule sceneName*/
				recog.base.set_state(163);
				recog.sceneName()?;

				}
			}

			recog.base.set_state(166);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule reactAction*/
			recog.base.set_state(167);
			recog.reactAction()?;

			recog.base.set_state(169);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==WITHTEXT {
				{
				/*InvokeRule animateDeclaration*/
				recog.base.set_state(168);
				recog.animateDeclaration()?;

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
//------------------- animateDeclaration ----------------
pub type AnimateDeclarationContextAll<'input> = AnimateDeclarationContext<'input>;


pub type AnimateDeclarationContext<'input> = BaseParserRuleContext<'input,AnimateDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct AnimateDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for AnimateDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for AnimateDeclarationContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_animateDeclaration(self);
	}
}

impl<'input> CustomRuleContext<'input> for AnimateDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_animateDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_animateDeclaration }
}
antlr_rust::type_id!{AnimateDeclarationContextExt<'a>}

impl<'input> AnimateDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AnimateDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AnimateDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AnimateDeclarationContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<AnimateDeclarationContextExt<'input>>{

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
fn animateName(&self) -> Option<Rc<AnimateNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}

}

impl<'input> AnimateDeclarationContextAttrs<'input> for AnimateDeclarationContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn animateDeclaration(&mut self,)
	-> Result<Rc<AnimateDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AnimateDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_animateDeclaration);
        let mut _localctx: Rc<AnimateDeclarationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(171);
			recog.base.match_token(WITHTEXT,&mut recog.err_handler)?;

			recog.base.set_state(172);
			recog.base.match_token(ANIMATE,&mut recog.err_handler)?;

			recog.base.set_state(173);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			/*InvokeRule animateName*/
			recog.base.set_state(174);
			recog.animateName()?;

			recog.base.set_state(175);
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
//------------------- reactAction ----------------
pub type ReactActionContextAll<'input> = ReactActionContext<'input>;


pub type ReactActionContext<'input> = BaseParserRuleContext<'input,ReactActionContextExt<'input>>;

#[derive(Clone)]
pub struct ReactActionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for ReactActionContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for ReactActionContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_reactAction(self);
	}
}

impl<'input> CustomRuleContext<'input> for ReactActionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_reactAction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_reactAction }
}
antlr_rust::type_id!{ReactActionContextExt<'a>}

impl<'input> ReactActionContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ReactActionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ReactActionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ReactActionContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<ReactActionContextExt<'input>>{

fn gotoAction(&self) -> Option<Rc<GotoActionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn showAction(&self) -> Option<Rc<ShowActionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ReactActionContextAttrs<'input> for ReactActionContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn reactAction(&mut self,)
	-> Result<Rc<ReactActionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ReactActionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_reactAction);
        let mut _localctx: Rc<ReactActionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(179);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 GOTO_KEY 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule gotoAction*/
					recog.base.set_state(177);
					recog.gotoAction()?;

					}
				}

			 SHOW_KEY 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule showAction*/
					recog.base.set_state(178);
					recog.showAction()?;

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
//------------------- gotoAction ----------------
pub type GotoActionContextAll<'input> = GotoActionContext<'input>;


pub type GotoActionContext<'input> = BaseParserRuleContext<'input,GotoActionContextExt<'input>>;

#[derive(Clone)]
pub struct GotoActionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for GotoActionContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for GotoActionContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_gotoAction(self);
	}
}

impl<'input> CustomRuleContext<'input> for GotoActionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_gotoAction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_gotoAction }
}
antlr_rust::type_id!{GotoActionContextExt<'a>}

impl<'input> GotoActionContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<GotoActionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,GotoActionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait GotoActionContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<GotoActionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token GOTO_KEY
/// Returns `None` if there is no child corresponding to token GOTO_KEY
fn GOTO_KEY(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(GOTO_KEY, 0)
}
fn componentName(&self) -> Option<Rc<ComponentNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> GotoActionContextAttrs<'input> for GotoActionContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn gotoAction(&mut self,)
	-> Result<Rc<GotoActionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = GotoActionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_gotoAction);
        let mut _localctx: Rc<GotoActionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(181);
			recog.base.match_token(GOTO_KEY,&mut recog.err_handler)?;

			/*InvokeRule componentName*/
			recog.base.set_state(182);
			recog.componentName()?;

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
//------------------- showAction ----------------
pub type ShowActionContextAll<'input> = ShowActionContext<'input>;


pub type ShowActionContext<'input> = BaseParserRuleContext<'input,ShowActionContextExt<'input>>;

#[derive(Clone)]
pub struct ShowActionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for ShowActionContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for ShowActionContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_showAction(self);
	}
}

impl<'input> CustomRuleContext<'input> for ShowActionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_showAction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_showAction }
}
antlr_rust::type_id!{ShowActionContextExt<'a>}

impl<'input> ShowActionContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ShowActionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ShowActionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ShowActionContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<ShowActionContextExt<'input>>{

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
fn componentName(&self) -> Option<Rc<ComponentNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ShowActionContextAttrs<'input> for ShowActionContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn showAction(&mut self,)
	-> Result<Rc<ShowActionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ShowActionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_showAction);
        let mut _localctx: Rc<ShowActionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(184);
			recog.base.match_token(SHOW_KEY,&mut recog.err_handler)?;

			recog.base.set_state(185);
			recog.base.match_token(STRING_LITERAL,&mut recog.err_handler)?;

			recog.base.set_state(186);
			recog.base.match_token(DOT,&mut recog.err_handler)?;

			/*InvokeRule componentName*/
			recog.base.set_state(187);
			recog.componentName()?;

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
//------------------- actionName ----------------
pub type ActionNameContextAll<'input> = ActionNameContext<'input>;


pub type ActionNameContext<'input> = BaseParserRuleContext<'input,ActionNameContextExt<'input>>;

#[derive(Clone)]
pub struct ActionNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for ActionNameContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for ActionNameContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_actionName(self);
	}
}

impl<'input> CustomRuleContext<'input> for ActionNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_actionName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_actionName }
}
antlr_rust::type_id!{ActionNameContextExt<'a>}

impl<'input> ActionNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ActionNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ActionNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ActionNameContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<ActionNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> ActionNameContextAttrs<'input> for ActionNameContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn actionName(&mut self,)
	-> Result<Rc<ActionNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ActionNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_actionName);
        let mut _localctx: Rc<ActionNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(189);
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
//------------------- componentValue ----------------
pub type ComponentValueContextAll<'input> = ComponentValueContext<'input>;


pub type ComponentValueContext<'input> = BaseParserRuleContext<'input,ComponentValueContextExt<'input>>;

#[derive(Clone)]
pub struct ComponentValueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for ComponentValueContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for ComponentValueContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_componentValue(self);
	}
}

impl<'input> CustomRuleContext<'input> for ComponentValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_componentValue }
	//fn type_rule_index() -> usize where Self: Sized { RULE_componentValue }
}
antlr_rust::type_id!{ComponentValueContextExt<'a>}

impl<'input> ComponentValueContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ComponentValueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ComponentValueContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ComponentValueContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<ComponentValueContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> ComponentValueContextAttrs<'input> for ComponentValueContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn componentValue(&mut self,)
	-> Result<Rc<ComponentValueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ComponentValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_componentValue);
        let mut _localctx: Rc<ComponentValueContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(191);
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
//------------------- componentName ----------------
pub type ComponentNameContextAll<'input> = ComponentNameContext<'input>;


pub type ComponentNameContext<'input> = BaseParserRuleContext<'input,ComponentNameContextExt<'input>>;

#[derive(Clone)]
pub struct ComponentNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for ComponentNameContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for ComponentNameContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_componentName(self);
	}
}

impl<'input> CustomRuleContext<'input> for ComponentNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_componentName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_componentName }
}
antlr_rust::type_id!{ComponentNameContextExt<'a>}

impl<'input> ComponentNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ComponentNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ComponentNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ComponentNameContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<ComponentNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> ComponentNameContextAttrs<'input> for ComponentNameContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn componentName(&mut self,)
	-> Result<Rc<ComponentNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ComponentNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_componentName);
        let mut _localctx: Rc<ComponentNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(193);
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
//------------------- sceneName ----------------
pub type SceneNameContextAll<'input> = SceneNameContext<'input>;


pub type SceneNameContext<'input> = BaseParserRuleContext<'input,SceneNameContextExt<'input>>;

#[derive(Clone)]
pub struct SceneNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for SceneNameContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for SceneNameContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_sceneName(self);
	}
}

impl<'input> CustomRuleContext<'input> for SceneNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_sceneName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_sceneName }
}
antlr_rust::type_id!{SceneNameContextExt<'a>}

impl<'input> SceneNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SceneNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SceneNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SceneNameContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<SceneNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> SceneNameContextAttrs<'input> for SceneNameContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn sceneName(&mut self,)
	-> Result<Rc<SceneNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SceneNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_sceneName);
        let mut _localctx: Rc<SceneNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(195);
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
//------------------- animateName ----------------
pub type AnimateNameContextAll<'input> = AnimateNameContext<'input>;


pub type AnimateNameContext<'input> = BaseParserRuleContext<'input,AnimateNameContextExt<'input>>;

#[derive(Clone)]
pub struct AnimateNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for AnimateNameContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for AnimateNameContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_animateName(self);
	}
}

impl<'input> CustomRuleContext<'input> for AnimateNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_animateName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_animateName }
}
antlr_rust::type_id!{AnimateNameContextExt<'a>}

impl<'input> AnimateNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AnimateNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AnimateNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AnimateNameContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<AnimateNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> AnimateNameContextAttrs<'input> for AnimateNameContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn animateName(&mut self,)
	-> Result<Rc<AnimateNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AnimateNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_animateName);
        let mut _localctx: Rc<AnimateNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(197);
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
//------------------- pageDeclaration ----------------
pub type PageDeclarationContextAll<'input> = PageDeclarationContext<'input>;


pub type PageDeclarationContext<'input> = BaseParserRuleContext<'input,PageDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct PageDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for PageDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for PageDeclarationContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_pageDeclaration(self);
	}
}

impl<'input> CustomRuleContext<'input> for PageDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pageDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pageDeclaration }
}
antlr_rust::type_id!{PageDeclarationContextExt<'a>}

impl<'input> PageDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PageDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PageDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PageDeclarationContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<PageDeclarationContextExt<'input>>{

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
fn componentBodyDeclaration_all(&self) ->  Vec<Rc<ComponentBodyDeclarationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn componentBodyDeclaration(&self, i: usize) -> Option<Rc<ComponentBodyDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> PageDeclarationContextAttrs<'input> for PageDeclarationContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn pageDeclaration(&mut self,)
	-> Result<Rc<PageDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PageDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_pageDeclaration);
        let mut _localctx: Rc<PageDeclarationContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(199);
			recog.base.match_token(PAGE,&mut recog.err_handler)?;

			recog.base.set_state(200);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(201);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(205);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==IDENTIFIER {
				{
				{
				/*InvokeRule componentBodyDeclaration*/
				recog.base.set_state(202);
				recog.componentBodyDeclaration()?;

				}
				}
				recog.base.set_state(207);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(208);
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
//------------------- componentDeclaration ----------------
pub type ComponentDeclarationContextAll<'input> = ComponentDeclarationContext<'input>;


pub type ComponentDeclarationContext<'input> = BaseParserRuleContext<'input,ComponentDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct ComponentDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for ComponentDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for ComponentDeclarationContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_componentDeclaration(self);
	}
}

impl<'input> CustomRuleContext<'input> for ComponentDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_componentDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_componentDeclaration }
}
antlr_rust::type_id!{ComponentDeclarationContextExt<'a>}

impl<'input> ComponentDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ComponentDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ComponentDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ComponentDeclarationContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<ComponentDeclarationContextExt<'input>>{

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
fn componentBodyDeclaration_all(&self) ->  Vec<Rc<ComponentBodyDeclarationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn componentBodyDeclaration(&self, i: usize) -> Option<Rc<ComponentBodyDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ComponentDeclarationContextAttrs<'input> for ComponentDeclarationContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn componentDeclaration(&mut self,)
	-> Result<Rc<ComponentDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ComponentDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_componentDeclaration);
        let mut _localctx: Rc<ComponentDeclarationContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(210);
			recog.base.match_token(COMPONENT,&mut recog.err_handler)?;

			recog.base.set_state(211);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(212);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(216);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==IDENTIFIER {
				{
				{
				/*InvokeRule componentBodyDeclaration*/
				recog.base.set_state(213);
				recog.componentBodyDeclaration()?;

				}
				}
				recog.base.set_state(218);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(219);
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
//------------------- componentBodyDeclaration ----------------
pub type ComponentBodyDeclarationContextAll<'input> = ComponentBodyDeclarationContext<'input>;


pub type ComponentBodyDeclarationContext<'input> = BaseParserRuleContext<'input,ComponentBodyDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct ComponentBodyDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for ComponentBodyDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for ComponentBodyDeclarationContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_componentBodyDeclaration(self);
	}
}

impl<'input> CustomRuleContext<'input> for ComponentBodyDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_componentBodyDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_componentBodyDeclaration }
}
antlr_rust::type_id!{ComponentBodyDeclarationContextExt<'a>}

impl<'input> ComponentBodyDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ComponentBodyDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ComponentBodyDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ComponentBodyDeclarationContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<ComponentBodyDeclarationContextExt<'input>>{

fn componentName_all(&self) ->  Vec<Rc<ComponentNameContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn componentName(&self, i: usize) -> Option<Rc<ComponentNameContextAll<'input>>> where Self:Sized{
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
fn configKey(&self) -> Option<Rc<ConfigKeyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn configValue(&self) -> Option<Rc<ConfigValueContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ComponentBodyDeclarationContextAttrs<'input> for ComponentBodyDeclarationContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn componentBodyDeclaration(&mut self,)
	-> Result<Rc<ComponentBodyDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ComponentBodyDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_componentBodyDeclaration);
        let mut _localctx: Rc<ComponentBodyDeclarationContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(233);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(17,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule componentName*/
					recog.base.set_state(221);
					recog.componentName()?;

					recog.base.set_state(226);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==COMMA {
						{
						{
						recog.base.set_state(222);
						recog.base.match_token(COMMA,&mut recog.err_handler)?;

						/*InvokeRule componentName*/
						recog.base.set_state(223);
						recog.componentName()?;

						}
						}
						recog.base.set_state(228);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule configKey*/
					recog.base.set_state(229);
					recog.configKey()?;

					recog.base.set_state(230);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					/*InvokeRule configValue*/
					recog.base.set_state(231);
					recog.configValue()?;

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
//------------------- layoutDeclaration ----------------
pub type LayoutDeclarationContextAll<'input> = LayoutDeclarationContext<'input>;


pub type LayoutDeclarationContext<'input> = BaseParserRuleContext<'input,LayoutDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct LayoutDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for LayoutDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for LayoutDeclarationContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_layoutDeclaration(self);
	}
}

impl<'input> CustomRuleContext<'input> for LayoutDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_layoutDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_layoutDeclaration }
}
antlr_rust::type_id!{LayoutDeclarationContextExt<'a>}

impl<'input> LayoutDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LayoutDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LayoutDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LayoutDeclarationContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<LayoutDeclarationContextExt<'input>>{

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
fn layoutRow_all(&self) ->  Vec<Rc<LayoutRowContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn layoutRow(&self, i: usize) -> Option<Rc<LayoutRowContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> LayoutDeclarationContextAttrs<'input> for LayoutDeclarationContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn layoutDeclaration(&mut self,)
	-> Result<Rc<LayoutDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LayoutDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_layoutDeclaration);
        let mut _localctx: Rc<LayoutDeclarationContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(235);
			recog.base.match_token(LAYOUT,&mut recog.err_handler)?;

			recog.base.set_state(236);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(237);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(241);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__3 || _la==T__4 {
				{
				{
				/*InvokeRule layoutRow*/
				recog.base.set_state(238);
				recog.layoutRow()?;

				}
				}
				recog.base.set_state(243);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(244);
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
//------------------- layoutRow ----------------
pub type LayoutRowContextAll<'input> = LayoutRowContext<'input>;


pub type LayoutRowContext<'input> = BaseParserRuleContext<'input,LayoutRowContextExt<'input>>;

#[derive(Clone)]
pub struct LayoutRowContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for LayoutRowContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for LayoutRowContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_layoutRow(self);
	}
}

impl<'input> CustomRuleContext<'input> for LayoutRowContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_layoutRow }
	//fn type_rule_index() -> usize where Self: Sized { RULE_layoutRow }
}
antlr_rust::type_id!{LayoutRowContextExt<'a>}

impl<'input> LayoutRowContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LayoutRowContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LayoutRowContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LayoutRowContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<LayoutRowContextExt<'input>>{

fn layoutLines(&self) -> Option<Rc<LayoutLinesContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> LayoutRowContextAttrs<'input> for LayoutRowContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn layoutRow(&mut self,)
	-> Result<Rc<LayoutRowContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LayoutRowContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_layoutRow);
        let mut _localctx: Rc<LayoutRowContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			recog.base.set_state(256);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__3 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(246);
					recog.base.match_token(T__3,&mut recog.err_handler)?;

					recog.base.set_state(250);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(19,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							recog.base.set_state(247);
							recog.base.match_token(T__3,&mut recog.err_handler)?;

							}
							} 
						}
						recog.base.set_state(252);
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
					/*InvokeRule layoutLines*/
					recog.base.set_state(253);
					recog.layoutLines()?;

					recog.base.set_state(254);
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
//------------------- layoutLines ----------------
pub type LayoutLinesContextAll<'input> = LayoutLinesContext<'input>;


pub type LayoutLinesContext<'input> = BaseParserRuleContext<'input,LayoutLinesContextExt<'input>>;

#[derive(Clone)]
pub struct LayoutLinesContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for LayoutLinesContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for LayoutLinesContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_layoutLines(self);
	}
}

impl<'input> CustomRuleContext<'input> for LayoutLinesContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_layoutLines }
	//fn type_rule_index() -> usize where Self: Sized { RULE_layoutLines }
}
antlr_rust::type_id!{LayoutLinesContextExt<'a>}

impl<'input> LayoutLinesContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LayoutLinesContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LayoutLinesContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LayoutLinesContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<LayoutLinesContextExt<'input>>{

fn layoutLine_all(&self) ->  Vec<Rc<LayoutLineContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn layoutLine(&self, i: usize) -> Option<Rc<LayoutLineContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> LayoutLinesContextAttrs<'input> for LayoutLinesContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn layoutLines(&mut self,)
	-> Result<Rc<LayoutLinesContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LayoutLinesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_layoutLines);
        let mut _localctx: Rc<LayoutLinesContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule layoutLine*/
			recog.base.set_state(258);
			recog.layoutLine()?;

			recog.base.set_state(262);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(21,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule layoutLine*/
					recog.base.set_state(259);
					recog.layoutLine()?;

					}
					} 
				}
				recog.base.set_state(264);
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
//------------------- layoutLine ----------------
pub type LayoutLineContextAll<'input> = LayoutLineContext<'input>;


pub type LayoutLineContext<'input> = BaseParserRuleContext<'input,LayoutLineContextExt<'input>>;

#[derive(Clone)]
pub struct LayoutLineContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for LayoutLineContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for LayoutLineContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_layoutLine(self);
	}
}

impl<'input> CustomRuleContext<'input> for LayoutLineContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_layoutLine }
	//fn type_rule_index() -> usize where Self: Sized { RULE_layoutLine }
}
antlr_rust::type_id!{LayoutLineContextExt<'a>}

impl<'input> LayoutLineContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LayoutLineContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LayoutLineContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LayoutLineContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<LayoutLineContextExt<'input>>{

fn componentUseDeclaration(&self) -> Option<Rc<ComponentUseDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> LayoutLineContextAttrs<'input> for LayoutLineContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn layoutLine(&mut self,)
	-> Result<Rc<LayoutLineContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LayoutLineContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_layoutLine);
        let mut _localctx: Rc<LayoutLineContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(265);
			recog.base.match_token(T__4,&mut recog.err_handler)?;

			/*InvokeRule componentUseDeclaration*/
			recog.base.set_state(266);
			recog.componentUseDeclaration()?;

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
//------------------- componentUseDeclaration ----------------
pub type ComponentUseDeclarationContextAll<'input> = ComponentUseDeclarationContext<'input>;


pub type ComponentUseDeclarationContext<'input> = BaseParserRuleContext<'input,ComponentUseDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct ComponentUseDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for ComponentUseDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for ComponentUseDeclarationContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_componentUseDeclaration(self);
	}
}

impl<'input> CustomRuleContext<'input> for ComponentUseDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_componentUseDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_componentUseDeclaration }
}
antlr_rust::type_id!{ComponentUseDeclarationContextExt<'a>}

impl<'input> ComponentUseDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ComponentUseDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ComponentUseDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ComponentUseDeclarationContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<ComponentUseDeclarationContextExt<'input>>{

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
fn componentName(&self) -> Option<Rc<ComponentNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn componentLayoutValue(&self) -> Option<Rc<ComponentLayoutValueContextAll<'input>>> where Self:Sized{
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

impl<'input> ComponentUseDeclarationContextAttrs<'input> for ComponentUseDeclarationContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn componentUseDeclaration(&mut self,)
	-> Result<Rc<ComponentUseDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ComponentUseDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_componentUseDeclaration);
        let mut _localctx: Rc<ComponentUseDeclarationContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(278);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 DECIMAL_LITERAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(268);
					recog.base.match_token(DECIMAL_LITERAL,&mut recog.err_handler)?;

					}
				}

			 POSITION 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(269);
					recog.base.match_token(POSITION,&mut recog.err_handler)?;

					}
				}

			 IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule componentName*/
					recog.base.set_state(270);
					recog.componentName()?;

					recog.base.set_state(275);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LPAREN {
						{
						recog.base.set_state(271);
						recog.base.match_token(LPAREN,&mut recog.err_handler)?;

						/*InvokeRule componentLayoutValue*/
						recog.base.set_state(272);
						recog.componentLayoutValue()?;

						recog.base.set_state(273);
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
					recog.base.set_state(277);
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
//------------------- componentLayoutValue ----------------
pub type ComponentLayoutValueContextAll<'input> = ComponentLayoutValueContext<'input>;


pub type ComponentLayoutValueContext<'input> = BaseParserRuleContext<'input,ComponentLayoutValueContextExt<'input>>;

#[derive(Clone)]
pub struct ComponentLayoutValueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for ComponentLayoutValueContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for ComponentLayoutValueContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_componentLayoutValue(self);
	}
}

impl<'input> CustomRuleContext<'input> for ComponentLayoutValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_componentLayoutValue }
	//fn type_rule_index() -> usize where Self: Sized { RULE_componentLayoutValue }
}
antlr_rust::type_id!{ComponentLayoutValueContextExt<'a>}

impl<'input> ComponentLayoutValueContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ComponentLayoutValueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ComponentLayoutValueContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ComponentLayoutValueContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<ComponentLayoutValueContextExt<'input>>{

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

impl<'input> ComponentLayoutValueContextAttrs<'input> for ComponentLayoutValueContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn componentLayoutValue(&mut self,)
	-> Result<Rc<ComponentLayoutValueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ComponentLayoutValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_componentLayoutValue);
        let mut _localctx: Rc<ComponentLayoutValueContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(280);
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
//------------------- styleDeclaration ----------------
pub type StyleDeclarationContextAll<'input> = StyleDeclarationContext<'input>;


pub type StyleDeclarationContext<'input> = BaseParserRuleContext<'input,StyleDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct StyleDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for StyleDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for StyleDeclarationContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_styleDeclaration(self);
	}
}

impl<'input> CustomRuleContext<'input> for StyleDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_styleDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_styleDeclaration }
}
antlr_rust::type_id!{StyleDeclarationContextExt<'a>}

impl<'input> StyleDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StyleDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StyleDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StyleDeclarationContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<StyleDeclarationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STYLE
/// Returns `None` if there is no child corresponding to token STYLE
fn STYLE(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(STYLE, 0)
}
fn styleName(&self) -> Option<Rc<StyleNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
fn styleBody(&self) -> Option<Rc<StyleBodyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}

}

impl<'input> StyleDeclarationContextAttrs<'input> for StyleDeclarationContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn styleDeclaration(&mut self,)
	-> Result<Rc<StyleDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StyleDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_styleDeclaration);
        let mut _localctx: Rc<StyleDeclarationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(282);
			recog.base.match_token(STYLE,&mut recog.err_handler)?;

			/*InvokeRule styleName*/
			recog.base.set_state(283);
			recog.styleName()?;

			recog.base.set_state(284);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			/*InvokeRule styleBody*/
			recog.base.set_state(285);
			recog.styleBody()?;

			recog.base.set_state(286);
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
//------------------- styleName ----------------
pub type StyleNameContextAll<'input> = StyleNameContext<'input>;


pub type StyleNameContext<'input> = BaseParserRuleContext<'input,StyleNameContextExt<'input>>;

#[derive(Clone)]
pub struct StyleNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for StyleNameContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for StyleNameContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_styleName(self);
	}
}

impl<'input> CustomRuleContext<'input> for StyleNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_styleName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_styleName }
}
antlr_rust::type_id!{StyleNameContextExt<'a>}

impl<'input> StyleNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StyleNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StyleNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StyleNameContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<StyleNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> StyleNameContextAttrs<'input> for StyleNameContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn styleName(&mut self,)
	-> Result<Rc<StyleNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StyleNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_styleName);
        let mut _localctx: Rc<StyleNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(288);
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
//------------------- styleBody ----------------
pub type StyleBodyContextAll<'input> = StyleBodyContext<'input>;


pub type StyleBodyContext<'input> = BaseParserRuleContext<'input,StyleBodyContextExt<'input>>;

#[derive(Clone)]
pub struct StyleBodyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for StyleBodyContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for StyleBodyContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_styleBody(self);
	}
}

impl<'input> CustomRuleContext<'input> for StyleBodyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_styleBody }
	//fn type_rule_index() -> usize where Self: Sized { RULE_styleBody }
}
antlr_rust::type_id!{StyleBodyContextExt<'a>}

impl<'input> StyleBodyContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StyleBodyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StyleBodyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StyleBodyContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<StyleBodyContextExt<'input>>{

fn configDeclaration_all(&self) ->  Vec<Rc<ConfigDeclarationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn configDeclaration(&self, i: usize) -> Option<Rc<ConfigDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> StyleBodyContextAttrs<'input> for StyleBodyContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn styleBody(&mut self,)
	-> Result<Rc<StyleBodyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StyleBodyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_styleBody);
        let mut _localctx: Rc<StyleBodyContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(295);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==IDENTIFIER {
				{
				{
				/*InvokeRule configDeclaration*/
				recog.base.set_state(290);
				recog.configDeclaration()?;

				recog.base.set_state(291);
				recog.base.match_token(T__5,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(297);
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
//------------------- libraryDeclaration ----------------
pub type LibraryDeclarationContextAll<'input> = LibraryDeclarationContext<'input>;


pub type LibraryDeclarationContext<'input> = BaseParserRuleContext<'input,LibraryDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct LibraryDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for LibraryDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for LibraryDeclarationContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_libraryDeclaration(self);
	}
}

impl<'input> CustomRuleContext<'input> for LibraryDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_libraryDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_libraryDeclaration }
}
antlr_rust::type_id!{LibraryDeclarationContextExt<'a>}

impl<'input> LibraryDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LibraryDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LibraryDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LibraryDeclarationContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<LibraryDeclarationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LIBRARY
/// Returns `None` if there is no child corresponding to token LIBRARY
fn LIBRARY(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(LIBRARY, 0)
}
fn libraryName(&self) -> Option<Rc<LibraryNameContextAll<'input>>> where Self:Sized{
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
fn libraryExpress_all(&self) ->  Vec<Rc<LibraryExpressContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn libraryExpress(&self, i: usize) -> Option<Rc<LibraryExpressContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> LibraryDeclarationContextAttrs<'input> for LibraryDeclarationContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn libraryDeclaration(&mut self,)
	-> Result<Rc<LibraryDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LibraryDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_libraryDeclaration);
        let mut _localctx: Rc<LibraryDeclarationContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(298);
			recog.base.match_token(LIBRARY,&mut recog.err_handler)?;

			/*InvokeRule libraryName*/
			recog.base.set_state(299);
			recog.libraryName()?;

			recog.base.set_state(300);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(304);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==IDENTIFIER {
				{
				{
				/*InvokeRule libraryExpress*/
				recog.base.set_state(301);
				recog.libraryExpress()?;

				}
				}
				recog.base.set_state(306);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(307);
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
//------------------- libraryExpress ----------------
pub type LibraryExpressContextAll<'input> = LibraryExpressContext<'input>;


pub type LibraryExpressContext<'input> = BaseParserRuleContext<'input,LibraryExpressContextExt<'input>>;

#[derive(Clone)]
pub struct LibraryExpressContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for LibraryExpressContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for LibraryExpressContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_libraryExpress(self);
	}
}

impl<'input> CustomRuleContext<'input> for LibraryExpressContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_libraryExpress }
	//fn type_rule_index() -> usize where Self: Sized { RULE_libraryExpress }
}
antlr_rust::type_id!{LibraryExpressContextExt<'a>}

impl<'input> LibraryExpressContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LibraryExpressContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LibraryExpressContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LibraryExpressContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<LibraryExpressContextExt<'input>>{

fn presetKey(&self) -> Option<Rc<PresetKeyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn presetValue(&self) -> Option<Rc<PresetValueContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn presetArray(&self) -> Option<Rc<PresetArrayContextAll<'input>>> where Self:Sized{
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

impl<'input> LibraryExpressContextAttrs<'input> for LibraryExpressContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn libraryExpress(&mut self,)
	-> Result<Rc<LibraryExpressContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LibraryExpressContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_libraryExpress);
        let mut _localctx: Rc<LibraryExpressContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(328);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(29,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule presetKey*/
					recog.base.set_state(309);
					recog.presetKey()?;

					recog.base.set_state(310);
					recog.base.match_token(T__6,&mut recog.err_handler)?;

					recog.base.set_state(313);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 STRING_LITERAL | IDENTIFIER | DIGITS_IDENTIFIER | DECIMAL_LITERAL |
					 FLOAT_LITERAL 
						=> {
							{
							/*InvokeRule presetValue*/
							recog.base.set_state(311);
							recog.presetValue()?;

							}
						}

					 LBRACK 
						=> {
							{
							/*InvokeRule presetArray*/
							recog.base.set_state(312);
							recog.presetArray()?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					recog.base.set_state(316);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__5 {
						{
						recog.base.set_state(315);
						recog.base.match_token(T__5,&mut recog.err_handler)?;

						}
					}

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule presetKey*/
					recog.base.set_state(318);
					recog.presetKey()?;

					recog.base.set_state(319);
					recog.base.match_token(LBRACE,&mut recog.err_handler)?;

					recog.base.set_state(323);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==IDENTIFIER {
						{
						{
						/*InvokeRule keyValue*/
						recog.base.set_state(320);
						recog.keyValue()?;

						}
						}
						recog.base.set_state(325);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(326);
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

fn presetKey(&self) -> Option<Rc<PresetKeyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn presetValue(&self) -> Option<Rc<PresetValueContextAll<'input>>> where Self:Sized{
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
			/*InvokeRule presetKey*/
			recog.base.set_state(330);
			recog.presetKey()?;

			recog.base.set_state(331);
			recog.base.match_token(T__6,&mut recog.err_handler)?;

			/*InvokeRule presetValue*/
			recog.base.set_state(332);
			recog.presetValue()?;

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
//------------------- presetKey ----------------
pub type PresetKeyContextAll<'input> = PresetKeyContext<'input>;


pub type PresetKeyContext<'input> = BaseParserRuleContext<'input,PresetKeyContextExt<'input>>;

#[derive(Clone)]
pub struct PresetKeyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for PresetKeyContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for PresetKeyContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_presetKey(self);
	}
}

impl<'input> CustomRuleContext<'input> for PresetKeyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_presetKey }
	//fn type_rule_index() -> usize where Self: Sized { RULE_presetKey }
}
antlr_rust::type_id!{PresetKeyContextExt<'a>}

impl<'input> PresetKeyContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PresetKeyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PresetKeyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PresetKeyContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<PresetKeyContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> PresetKeyContextAttrs<'input> for PresetKeyContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn presetKey(&mut self,)
	-> Result<Rc<PresetKeyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PresetKeyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_presetKey);
        let mut _localctx: Rc<PresetKeyContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(334);
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
//------------------- presetValue ----------------
pub type PresetValueContextAll<'input> = PresetValueContext<'input>;


pub type PresetValueContext<'input> = BaseParserRuleContext<'input,PresetValueContextExt<'input>>;

#[derive(Clone)]
pub struct PresetValueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for PresetValueContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for PresetValueContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_presetValue(self);
	}
}

impl<'input> CustomRuleContext<'input> for PresetValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_presetValue }
	//fn type_rule_index() -> usize where Self: Sized { RULE_presetValue }
}
antlr_rust::type_id!{PresetValueContextExt<'a>}

impl<'input> PresetValueContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PresetValueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PresetValueContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PresetValueContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<PresetValueContextExt<'input>>{

fn configValue(&self) -> Option<Rc<ConfigValueContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PresetValueContextAttrs<'input> for PresetValueContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn presetValue(&mut self,)
	-> Result<Rc<PresetValueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PresetValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 74, RULE_presetValue);
        let mut _localctx: Rc<PresetValueContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule configValue*/
			recog.base.set_state(336);
			recog.configValue()?;

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
//------------------- presetArray ----------------
pub type PresetArrayContextAll<'input> = PresetArrayContext<'input>;


pub type PresetArrayContext<'input> = BaseParserRuleContext<'input,PresetArrayContextExt<'input>>;

#[derive(Clone)]
pub struct PresetArrayContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for PresetArrayContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for PresetArrayContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_presetArray(self);
	}
}

impl<'input> CustomRuleContext<'input> for PresetArrayContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_presetArray }
	//fn type_rule_index() -> usize where Self: Sized { RULE_presetArray }
}
antlr_rust::type_id!{PresetArrayContextExt<'a>}

impl<'input> PresetArrayContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PresetArrayContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PresetArrayContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PresetArrayContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<PresetArrayContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACK
/// Returns `None` if there is no child corresponding to token LBRACK
fn LBRACK(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(LBRACK, 0)
}
fn presetCall_all(&self) ->  Vec<Rc<PresetCallContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn presetCall(&self, i: usize) -> Option<Rc<PresetCallContextAll<'input>>> where Self:Sized{
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

impl<'input> PresetArrayContextAttrs<'input> for PresetArrayContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn presetArray(&mut self,)
	-> Result<Rc<PresetArrayContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PresetArrayContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 76, RULE_presetArray);
        let mut _localctx: Rc<PresetArrayContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(338);
			recog.base.match_token(LBRACK,&mut recog.err_handler)?;

			/*InvokeRule presetCall*/
			recog.base.set_state(339);
			recog.presetCall()?;

			recog.base.set_state(344);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(340);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule presetCall*/
				recog.base.set_state(341);
				recog.presetCall()?;

				}
				}
				recog.base.set_state(346);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(347);
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
//------------------- presetCall ----------------
pub type PresetCallContextAll<'input> = PresetCallContext<'input>;


pub type PresetCallContext<'input> = BaseParserRuleContext<'input,PresetCallContextExt<'input>>;

#[derive(Clone)]
pub struct PresetCallContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for PresetCallContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for PresetCallContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_presetCall(self);
	}
}

impl<'input> CustomRuleContext<'input> for PresetCallContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_presetCall }
	//fn type_rule_index() -> usize where Self: Sized { RULE_presetCall }
}
antlr_rust::type_id!{PresetCallContextExt<'a>}

impl<'input> PresetCallContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PresetCallContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PresetCallContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PresetCallContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<PresetCallContextExt<'input>>{

fn libraryName(&self) -> Option<Rc<LibraryNameContextAll<'input>>> where Self:Sized{
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

impl<'input> PresetCallContextAttrs<'input> for PresetCallContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn presetCall(&mut self,)
	-> Result<Rc<PresetCallContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PresetCallContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 78, RULE_presetCall);
        let mut _localctx: Rc<PresetCallContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule libraryName*/
			recog.base.set_state(349);
			recog.libraryName()?;

			recog.base.set_state(350);
			recog.base.match_token(DOT,&mut recog.err_handler)?;

			recog.base.set_state(351);
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
//------------------- libraryName ----------------
pub type LibraryNameContextAll<'input> = LibraryNameContext<'input>;


pub type LibraryNameContext<'input> = BaseParserRuleContext<'input,LibraryNameContextExt<'input>>;

#[derive(Clone)]
pub struct LibraryNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> DesignParserContext<'input> for LibraryNameContext<'input>{}

impl<'input,'a> Listenable<dyn DesignListener<'input> + 'a> for LibraryNameContext<'input>{
	fn enter(&self,listener: &mut (dyn DesignListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_libraryName(self);
	}
}

impl<'input> CustomRuleContext<'input> for LibraryNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = DesignParserContextType;
	fn get_rule_index(&self) -> usize { RULE_libraryName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_libraryName }
}
antlr_rust::type_id!{LibraryNameContextExt<'a>}

impl<'input> LibraryNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn DesignParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LibraryNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LibraryNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LibraryNameContextAttrs<'input>: DesignParserContext<'input> + BorrowMut<LibraryNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,DesignParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> LibraryNameContextAttrs<'input> for LibraryNameContext<'input>{}

impl<'input, I, H> DesignParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn libraryName(&mut self,)
	-> Result<Rc<LibraryNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LibraryNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 80, RULE_libraryName);
        let mut _localctx: Rc<LibraryNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(353);
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
	\x31\u{166}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x04\
	\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\x20\x09\
	\x20\x04\x21\x09\x21\x04\x22\x09\x22\x04\x23\x09\x23\x04\x24\x09\x24\x04\
	\x25\x09\x25\x04\x26\x09\x26\x04\x27\x09\x27\x04\x28\x09\x28\x04\x29\x09\
	\x29\x04\x2a\x09\x2a\x03\x02\x03\x02\x03\x02\x07\x02\x58\x0a\x02\x0c\x02\
	\x0e\x02\x5b\x0b\x02\x03\x02\x03\x02\x03\x03\x03\x03\x03\x04\x03\x04\x03\
	\x04\x03\x04\x03\x05\x03\x05\x03\x06\x03\x06\x05\x06\x69\x0a\x06\x03\x06\
	\x03\x06\x05\x06\x6d\x0a\x06\x03\x06\x03\x06\x05\x06\x71\x0a\x06\x03\x06\
	\x03\x06\x03\x06\x05\x06\x76\x0a\x06\x03\x06\x05\x06\x79\x0a\x06\x03\x07\
	\x03\x07\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x05\x08\
	\u{84}\x0a\x08\x03\x09\x03\x09\x03\x09\x03\x09\x07\x09\u{8a}\x0a\x09\x0c\
	\x09\x0e\x09\u{8d}\x0b\x09\x03\x09\x03\x09\x03\x0a\x03\x0a\x03\x0a\x05\x0a\
	\u{94}\x0a\x0a\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x05\x0b\u{9b}\x0a\
	\x0b\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\
	\x0d\x03\x0d\x05\x0d\u{a7}\x0a\x0d\x03\x0d\x03\x0d\x03\x0d\x05\x0d\u{ac}\
	\x0a\x0d\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0f\x03\x0f\
	\x05\x0f\u{b6}\x0a\x0f\x03\x10\x03\x10\x03\x10\x03\x11\x03\x11\x03\x11\x03\
	\x11\x03\x11\x03\x12\x03\x12\x03\x13\x03\x13\x03\x14\x03\x14\x03\x15\x03\
	\x15\x03\x16\x03\x16\x03\x17\x03\x17\x03\x17\x03\x17\x07\x17\u{ce}\x0a\x17\
	\x0c\x17\x0e\x17\u{d1}\x0b\x17\x03\x17\x03\x17\x03\x18\x03\x18\x03\x18\x03\
	\x18\x07\x18\u{d9}\x0a\x18\x0c\x18\x0e\x18\u{dc}\x0b\x18\x03\x18\x03\x18\
	\x03\x19\x03\x19\x03\x19\x07\x19\u{e3}\x0a\x19\x0c\x19\x0e\x19\u{e6}\x0b\
	\x19\x03\x19\x03\x19\x03\x19\x03\x19\x05\x19\u{ec}\x0a\x19\x03\x1a\x03\x1a\
	\x03\x1a\x03\x1a\x07\x1a\u{f2}\x0a\x1a\x0c\x1a\x0e\x1a\u{f5}\x0b\x1a\x03\
	\x1a\x03\x1a\x03\x1b\x03\x1b\x07\x1b\u{fb}\x0a\x1b\x0c\x1b\x0e\x1b\u{fe}\
	\x0b\x1b\x03\x1b\x03\x1b\x03\x1b\x05\x1b\u{103}\x0a\x1b\x03\x1c\x03\x1c\
	\x07\x1c\u{107}\x0a\x1c\x0c\x1c\x0e\x1c\u{10a}\x0b\x1c\x03\x1d\x03\x1d\x03\
	\x1d\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x05\x1e\u{116}\
	\x0a\x1e\x03\x1e\x05\x1e\u{119}\x0a\x1e\x03\x1f\x03\x1f\x03\x20\x03\x20\
	\x03\x20\x03\x20\x03\x20\x03\x20\x03\x21\x03\x21\x03\x22\x03\x22\x03\x22\
	\x07\x22\u{128}\x0a\x22\x0c\x22\x0e\x22\u{12b}\x0b\x22\x03\x23\x03\x23\x03\
	\x23\x03\x23\x07\x23\u{131}\x0a\x23\x0c\x23\x0e\x23\u{134}\x0b\x23\x03\x23\
	\x03\x23\x03\x24\x03\x24\x03\x24\x03\x24\x05\x24\u{13c}\x0a\x24\x03\x24\
	\x05\x24\u{13f}\x0a\x24\x03\x24\x03\x24\x03\x24\x07\x24\u{144}\x0a\x24\x0c\
	\x24\x0e\x24\u{147}\x0b\x24\x03\x24\x03\x24\x05\x24\u{14b}\x0a\x24\x03\x25\
	\x03\x25\x03\x25\x03\x25\x03\x26\x03\x26\x03\x27\x03\x27\x03\x28\x03\x28\
	\x03\x28\x03\x28\x07\x28\u{159}\x0a\x28\x0c\x28\x0e\x28\u{15c}\x0b\x28\x03\
	\x28\x03\x28\x03\x29\x03\x29\x03\x29\x03\x29\x03\x2a\x03\x2a\x03\x2a\x02\
	\x02\x2b\x02\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\
	\x22\x24\x26\x28\x2a\x2c\x2e\x30\x32\x34\x36\x38\x3a\x3c\x3e\x40\x42\x44\
	\x46\x48\x4a\x4c\x4e\x50\x52\x02\x04\x03\x02\x03\x05\x05\x02\x15\x15\x1a\
	\x1a\x2f\x2f\x02\u{167}\x02\x59\x03\x02\x02\x02\x04\x5e\x03\x02\x02\x02\
	\x06\x60\x03\x02\x02\x02\x08\x64\x03\x02\x02\x02\x0a\x78\x03\x02\x02\x02\
	\x0c\x7a\x03\x02\x02\x02\x0e\u{83}\x03\x02\x02\x02\x10\u{85}\x03\x02\x02\
	\x02\x12\u{93}\x03\x02\x02\x02\x14\u{95}\x03\x02\x02\x02\x16\u{9c}\x03\x02\
	\x02\x02\x18\u{a4}\x03\x02\x02\x02\x1a\u{ad}\x03\x02\x02\x02\x1c\u{b5}\x03\
	\x02\x02\x02\x1e\u{b7}\x03\x02\x02\x02\x20\u{ba}\x03\x02\x02\x02\x22\u{bf}\
	\x03\x02\x02\x02\x24\u{c1}\x03\x02\x02\x02\x26\u{c3}\x03\x02\x02\x02\x28\
	\u{c5}\x03\x02\x02\x02\x2a\u{c7}\x03\x02\x02\x02\x2c\u{c9}\x03\x02\x02\x02\
	\x2e\u{d4}\x03\x02\x02\x02\x30\u{eb}\x03\x02\x02\x02\x32\u{ed}\x03\x02\x02\
	\x02\x34\u{102}\x03\x02\x02\x02\x36\u{104}\x03\x02\x02\x02\x38\u{10b}\x03\
	\x02\x02\x02\x3a\u{118}\x03\x02\x02\x02\x3c\u{11a}\x03\x02\x02\x02\x3e\u{11c}\
	\x03\x02\x02\x02\x40\u{122}\x03\x02\x02\x02\x42\u{129}\x03\x02\x02\x02\x44\
	\u{12c}\x03\x02\x02\x02\x46\u{14a}\x03\x02\x02\x02\x48\u{14c}\x03\x02\x02\
	\x02\x4a\u{150}\x03\x02\x02\x02\x4c\u{152}\x03\x02\x02\x02\x4e\u{154}\x03\
	\x02\x02\x02\x50\u{15f}\x03\x02\x02\x02\x52\u{163}\x03\x02\x02\x02\x54\x58\
	\x05\x04\x03\x02\x55\x58\x05\x06\x04\x02\x56\x58\x05\x0e\x08\x02\x57\x54\
	\x03\x02\x02\x02\x57\x55\x03\x02\x02\x02\x57\x56\x03\x02\x02\x02\x58\x5b\
	\x03\x02\x02\x02\x59\x57\x03\x02\x02\x02\x59\x5a\x03\x02\x02\x02\x5a\x5c\
	\x03\x02\x02\x02\x5b\x59\x03\x02\x02\x02\x5c\x5d\x07\x02\x02\x03\x5d\x03\
	\x03\x02\x02\x02\x5e\x5f\x07\x2d\x02\x02\x5f\x05\x03\x02\x02\x02\x60\x61\
	\x05\x08\x05\x02\x61\x62\x07\x29\x02\x02\x62\x63\x05\x0a\x06\x02\x63\x07\
	\x03\x02\x02\x02\x64\x65\x07\x2d\x02\x02\x65\x09\x03\x02\x02\x02\x66\x68\
	\x07\x2f\x02\x02\x67\x69\x05\x0c\x07\x02\x68\x67\x03\x02\x02\x02\x68\x69\
	\x03\x02\x02\x02\x69\x79\x03\x02\x02\x02\x6a\x6c\x07\x30\x02\x02\x6b\x6d\
	\x05\x0c\x07\x02\x6c\x6b\x03\x02\x02\x02\x6c\x6d\x03\x02\x02\x02\x6d\x79\
	\x03\x02\x02\x02\x6e\x70\x07\x31\x02\x02\x6f\x71\x05\x0c\x07\x02\x70\x6f\
	\x03\x02\x02\x02\x70\x71\x03\x02\x02\x02\x71\x79\x03\x02\x02\x02\x72\x75\
	\x07\x2d\x02\x02\x73\x74\x07\x2b\x02\x02\x74\x76\x07\x2d\x02\x02\x75\x73\
	\x03\x02\x02\x02\x75\x76\x03\x02\x02\x02\x76\x79\x03\x02\x02\x02\x77\x79\
	\x07\x1a\x02\x02\x78\x66\x03\x02\x02\x02\x78\x6a\x03\x02\x02\x02\x78\x6e\
	\x03\x02\x02\x02\x78\x72\x03\x02\x02\x02\x78\x77\x03\x02\x02\x02\x79\x0b\
	\x03\x02\x02\x02\x7a\x7b\x09\x02\x02\x02\x7b\x0d\x03\x02\x02\x02\x7c\u{84}\
	\x05\x06\x04\x02\x7d\u{84}\x05\x10\x09\x02\x7e\u{84}\x05\x2c\x17\x02\x7f\
	\u{84}\x05\x3e\x20\x02\u{80}\u{84}\x05\x2e\x18\x02\u{81}\u{84}\x05\x44\x23\
	\x02\u{82}\u{84}\x05\x32\x1a\x02\u{83}\x7c\x03\x02\x02\x02\u{83}\x7d\x03\
	\x02\x02\x02\u{83}\x7e\x03\x02\x02\x02\u{83}\x7f\x03\x02\x02\x02\u{83}\u{80}\
	\x03\x02\x02\x02\u{83}\u{81}\x03\x02\x02\x02\u{83}\u{82}\x03\x02\x02\x02\
	\u{84}\x0f\x03\x02\x02\x02\u{85}\u{86}\x07\x0c\x02\x02\u{86}\u{87}\x07\x2d\
	\x02\x02\u{87}\u{8b}\x07\x23\x02\x02\u{88}\u{8a}\x05\x12\x0a\x02\u{89}\u{88}\
	\x03\x02\x02\x02\u{8a}\u{8d}\x03\x02\x02\x02\u{8b}\u{89}\x03\x02\x02\x02\
	\u{8b}\u{8c}\x03\x02\x02\x02\u{8c}\u{8e}\x03\x02\x02\x02\u{8d}\u{8b}\x03\
	\x02\x02\x02\u{8e}\u{8f}\x07\x24\x02\x02\u{8f}\x11\x03\x02\x02\x02\u{90}\
	\u{94}\x05\x14\x0b\x02\u{91}\u{94}\x05\x16\x0c\x02\u{92}\u{94}\x05\x18\x0d\
	\x02\u{93}\u{90}\x03\x02\x02\x02\u{93}\u{91}\x03\x02\x02\x02\u{93}\u{92}\
	\x03\x02\x02\x02\u{94}\x13\x03\x02\x02\x02\u{95}\u{9a}\x07\x0d\x02\x02\u{96}\
	\u{9b}\x07\x2d\x02\x02\u{97}\u{98}\x07\x1a\x02\x02\u{98}\u{99}\x07\x2a\x02\
	\x02\u{99}\u{9b}\x05\x26\x14\x02\u{9a}\u{96}\x03\x02\x02\x02\u{9a}\u{97}\
	\x03\x02\x02\x02\u{9b}\x15\x03\x02\x02\x02\u{9c}\u{9d}\x07\x0e\x02\x02\u{9d}\
	\u{9e}\x07\x25\x02\x02\u{9e}\u{9f}\x05\x22\x12\x02\u{9f}\u{a0}\x07\x26\x02\
	\x02\u{a0}\u{a1}\x07\x1a\x02\x02\u{a1}\u{a2}\x07\x2a\x02\x02\u{a2}\u{a3}\
	\x05\x26\x14\x02\u{a3}\x17\x03\x02\x02\x02\u{a4}\u{a6}\x07\x0f\x02\x02\u{a5}\
	\u{a7}\x05\x28\x15\x02\u{a6}\u{a5}\x03\x02\x02\x02\u{a6}\u{a7}\x03\x02\x02\
	\x02\u{a7}\u{a8}\x03\x02\x02\x02\u{a8}\u{a9}\x07\x29\x02\x02\u{a9}\u{ab}\
	\x05\x1c\x0f\x02\u{aa}\u{ac}\x05\x1a\x0e\x02\u{ab}\u{aa}\x03\x02\x02\x02\
	\u{ab}\u{ac}\x03\x02\x02\x02\u{ac}\x19\x03\x02\x02\x02\u{ad}\u{ae}\x07\x10\
	\x02\x02\u{ae}\u{af}\x07\x11\x02\x02\u{af}\u{b0}\x07\x21\x02\x02\u{b0}\u{b1}\
	\x05\x2a\x16\x02\u{b1}\u{b2}\x07\x22\x02\x02\u{b2}\x1b\x03\x02\x02\x02\u{b3}\
	\u{b6}\x05\x1e\x10\x02\u{b4}\u{b6}\x05\x20\x11\x02\u{b5}\u{b3}\x03\x02\x02\
	\x02\u{b5}\u{b4}\x03\x02\x02\x02\u{b6}\x1d\x03\x02\x02\x02\u{b7}\u{b8}\x07\
	\x0a\x02\x02\u{b8}\u{b9}\x05\x26\x14\x02\u{b9}\x1f\x03\x02\x02\x02\u{ba}\
	\u{bb}\x07\x0b\x02\x02\u{bb}\u{bc}\x07\x1a\x02\x02\u{bc}\u{bd}\x07\x2a\x02\
	\x02\u{bd}\u{be}\x05\x26\x14\x02\u{be}\x21\x03\x02\x02\x02\u{bf}\u{c0}\x07\
	\x2d\x02\x02\u{c0}\x23\x03\x02\x02\x02\u{c1}\u{c2}\x07\x2d\x02\x02\u{c2}\
	\x25\x03\x02\x02\x02\u{c3}\u{c4}\x07\x2d\x02\x02\u{c4}\x27\x03\x02\x02\x02\
	\u{c5}\u{c6}\x07\x2d\x02\x02\u{c6}\x29\x03\x02\x02\x02\u{c7}\u{c8}\x07\x2d\
	\x02\x02\u{c8}\x2b\x03\x02\x02\x02\u{c9}\u{ca}\x07\x16\x02\x02\u{ca}\u{cb}\
	\x07\x2d\x02\x02\u{cb}\u{cf}\x07\x23\x02\x02\u{cc}\u{ce}\x05\x30\x19\x02\
	\u{cd}\u{cc}\x03\x02\x02\x02\u{ce}\u{d1}\x03\x02\x02\x02\u{cf}\u{cd}\x03\
	\x02\x02\x02\u{cf}\u{d0}\x03\x02\x02\x02\u{d0}\u{d2}\x03\x02\x02\x02\u{d1}\
	\u{cf}\x03\x02\x02\x02\u{d2}\u{d3}\x07\x24\x02\x02\u{d3}\x2d\x03\x02\x02\
	\x02\u{d4}\u{d5}\x07\x17\x02\x02\u{d5}\u{d6}\x07\x2d\x02\x02\u{d6}\u{da}\
	\x07\x23\x02\x02\u{d7}\u{d9}\x05\x30\x19\x02\u{d8}\u{d7}\x03\x02\x02\x02\
	\u{d9}\u{dc}\x03\x02\x02\x02\u{da}\u{d8}\x03\x02\x02\x02\u{da}\u{db}\x03\
	\x02\x02\x02\u{db}\u{dd}\x03\x02\x02\x02\u{dc}\u{da}\x03\x02\x02\x02\u{dd}\
	\u{de}\x07\x24\x02\x02\u{de}\x2f\x03\x02\x02\x02\u{df}\u{e4}\x05\x26\x14\
	\x02\u{e0}\u{e1}\x07\x2b\x02\x02\u{e1}\u{e3}\x05\x26\x14\x02\u{e2}\u{e0}\
	\x03\x02\x02\x02\u{e3}\u{e6}\x03\x02\x02\x02\u{e4}\u{e2}\x03\x02\x02\x02\
	\u{e4}\u{e5}\x03\x02\x02\x02\u{e5}\u{ec}\x03\x02\x02\x02\u{e6}\u{e4}\x03\
	\x02\x02\x02\u{e7}\u{e8}\x05\x08\x05\x02\u{e8}\u{e9}\x07\x29\x02\x02\u{e9}\
	\u{ea}\x05\x0a\x06\x02\u{ea}\u{ec}\x03\x02\x02\x02\u{eb}\u{df}\x03\x02\x02\
	\x02\u{eb}\u{e7}\x03\x02\x02\x02\u{ec}\x31\x03\x02\x02\x02\u{ed}\u{ee}\x07\
	\x14\x02\x02\u{ee}\u{ef}\x07\x2d\x02\x02\u{ef}\u{f3}\x07\x23\x02\x02\u{f0}\
	\u{f2}\x05\x34\x1b\x02\u{f1}\u{f0}\x03\x02\x02\x02\u{f2}\u{f5}\x03\x02\x02\
	\x02\u{f3}\u{f1}\x03\x02\x02\x02\u{f3}\u{f4}\x03\x02\x02\x02\u{f4}\u{f6}\
	\x03\x02\x02\x02\u{f5}\u{f3}\x03\x02\x02\x02\u{f6}\u{f7}\x07\x24\x02\x02\
	\u{f7}\x33\x03\x02\x02\x02\u{f8}\u{fc}\x07\x06\x02\x02\u{f9}\u{fb}\x07\x06\
	\x02\x02\u{fa}\u{f9}\x03\x02\x02\x02\u{fb}\u{fe}\x03\x02\x02\x02\u{fc}\u{fa}\
	\x03\x02\x02\x02\u{fc}\u{fd}\x03\x02\x02\x02\u{fd}\u{103}\x03\x02\x02\x02\
	\u{fe}\u{fc}\x03\x02\x02\x02\u{ff}\u{100}\x05\x36\x1c\x02\u{100}\u{101}\
	\x07\x07\x02\x02\u{101}\u{103}\x03\x02\x02\x02\u{102}\u{f8}\x03\x02\x02\
	\x02\u{102}\u{ff}\x03\x02\x02\x02\u{103}\x35\x03\x02\x02\x02\u{104}\u{108}\
	\x05\x38\x1d\x02\u{105}\u{107}\x05\x38\x1d\x02\u{106}\u{105}\x03\x02\x02\
	\x02\u{107}\u{10a}\x03\x02\x02\x02\u{108}\u{106}\x03\x02\x02\x02\u{108}\
	\u{109}\x03\x02\x02\x02\u{109}\x37\x03\x02\x02\x02\u{10a}\u{108}\x03\x02\
	\x02\x02\u{10b}\u{10c}\x07\x07\x02\x02\u{10c}\u{10d}\x05\x3a\x1e\x02\u{10d}\
	\x39\x03\x02\x02\x02\u{10e}\u{119}\x07\x30\x02\x02\u{10f}\u{119}\x07\x15\
	\x02\x02\u{110}\u{115}\x05\x26\x14\x02\u{111}\u{112}\x07\x21\x02\x02\u{112}\
	\u{113}\x05\x3c\x1f\x02\u{113}\u{114}\x07\x22\x02\x02\u{114}\u{116}\x03\
	\x02\x02\x02\u{115}\u{111}\x03\x02\x02\x02\u{115}\u{116}\x03\x02\x02\x02\
	\u{116}\u{119}\x03\x02\x02\x02\u{117}\u{119}\x07\x1a\x02\x02\u{118}\u{10e}\
	\x03\x02\x02\x02\u{118}\u{10f}\x03\x02\x02\x02\u{118}\u{110}\x03\x02\x02\
	\x02\u{118}\u{117}\x03\x02\x02\x02\u{119}\x3b\x03\x02\x02\x02\u{11a}\u{11b}\
	\x09\x03\x02\x02\u{11b}\x3d\x03\x02\x02\x02\u{11c}\u{11d}\x07\x18\x02\x02\
	\u{11d}\u{11e}\x05\x40\x21\x02\u{11e}\u{11f}\x07\x23\x02\x02\u{11f}\u{120}\
	\x05\x42\x22\x02\u{120}\u{121}\x07\x24\x02\x02\u{121}\x3f\x03\x02\x02\x02\
	\u{122}\u{123}\x07\x2d\x02\x02\u{123}\x41\x03\x02\x02\x02\u{124}\u{125}\
	\x05\x06\x04\x02\u{125}\u{126}\x07\x08\x02\x02\u{126}\u{128}\x03\x02\x02\
	\x02\u{127}\u{124}\x03\x02\x02\x02\u{128}\u{12b}\x03\x02\x02\x02\u{129}\
	\u{127}\x03\x02\x02\x02\u{129}\u{12a}\x03\x02\x02\x02\u{12a}\x43\x03\x02\
	\x02\x02\u{12b}\u{129}\x03\x02\x02\x02\u{12c}\u{12d}\x07\x19\x02\x02\u{12d}\
	\u{12e}\x05\x52\x2a\x02\u{12e}\u{132}\x07\x23\x02\x02\u{12f}\u{131}\x05\
	\x46\x24\x02\u{130}\u{12f}\x03\x02\x02\x02\u{131}\u{134}\x03\x02\x02\x02\
	\u{132}\u{130}\x03\x02\x02\x02\u{132}\u{133}\x03\x02\x02\x02\u{133}\u{135}\
	\x03\x02\x02\x02\u{134}\u{132}\x03\x02\x02\x02\u{135}\u{136}\x07\x24\x02\
	\x02\u{136}\x45\x03\x02\x02\x02\u{137}\u{138}\x05\x4a\x26\x02\u{138}\u{13b}\
	\x07\x09\x02\x02\u{139}\u{13c}\x05\x4c\x27\x02\u{13a}\u{13c}\x05\x4e\x28\
	\x02\u{13b}\u{139}\x03\x02\x02\x02\u{13b}\u{13a}\x03\x02\x02\x02\u{13c}\
	\u{13e}\x03\x02\x02\x02\u{13d}\u{13f}\x07\x08\x02\x02\u{13e}\u{13d}\x03\
	\x02\x02\x02\u{13e}\u{13f}\x03\x02\x02\x02\u{13f}\u{14b}\x03\x02\x02\x02\
	\u{140}\u{141}\x05\x4a\x26\x02\u{141}\u{145}\x07\x23\x02\x02\u{142}\u{144}\
	\x05\x48\x25\x02\u{143}\u{142}\x03\x02\x02\x02\u{144}\u{147}\x03\x02\x02\
	\x02\u{145}\u{143}\x03\x02\x02\x02\u{145}\u{146}\x03\x02\x02\x02\u{146}\
	\u{148}\x03\x02\x02\x02\u{147}\u{145}\x03\x02\x02\x02\u{148}\u{149}\x07\
	\x24\x02\x02\u{149}\u{14b}\x03\x02\x02\x02\u{14a}\u{137}\x03\x02\x02\x02\
	\u{14a}\u{140}\x03\x02\x02\x02\u{14b}\x47\x03\x02\x02\x02\u{14c}\u{14d}\
	\x05\x4a\x26\x02\u{14d}\u{14e}\x07\x09\x02\x02\u{14e}\u{14f}\x05\x4c\x27\
	\x02\u{14f}\x49\x03\x02\x02\x02\u{150}\u{151}\x07\x2d\x02\x02\u{151}\x4b\
	\x03\x02\x02\x02\u{152}\u{153}\x05\x0a\x06\x02\u{153}\x4d\x03\x02\x02\x02\
	\u{154}\u{155}\x07\x25\x02\x02\u{155}\u{15a}\x05\x50\x29\x02\u{156}\u{157}\
	\x07\x2b\x02\x02\u{157}\u{159}\x05\x50\x29\x02\u{158}\u{156}\x03\x02\x02\
	\x02\u{159}\u{15c}\x03\x02\x02\x02\u{15a}\u{158}\x03\x02\x02\x02\u{15a}\
	\u{15b}\x03\x02\x02\x02\u{15b}\u{15d}\x03\x02\x02\x02\u{15c}\u{15a}\x03\
	\x02\x02\x02\u{15d}\u{15e}\x07\x26\x02\x02\u{15e}\x4f\x03\x02\x02\x02\u{15f}\
	\u{160}\x05\x52\x2a\x02\u{160}\u{161}\x07\x2a\x02\x02\u{161}\u{162}\x07\
	\x2d\x02\x02\u{162}\x51\x03\x02\x02\x02\u{163}\u{164}\x07\x2d\x02\x02\u{164}\
	\x53\x03\x02\x02\x02\x21\x57\x59\x68\x6c\x70\x75\x78\u{83}\u{8b}\u{93}\u{9a}\
	\u{a6}\u{ab}\u{b5}\u{cf}\u{da}\u{e4}\u{eb}\u{f3}\u{fc}\u{102}\u{108}\u{115}\
	\u{118}\u{129}\u{132}\u{13b}\u{13e}\u{145}\u{14a}\u{15a}";

