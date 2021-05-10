grammar Design;

@tokenfactory{
  pub type LocalTokenFactory<'input> = antlr_rust::token_factory::ArenaCommonFactory<'input>;
}

start: (comment | declarations)* EOF;

comment: IDENTIFIER;

config_decl: config_key COLON config_value;

config_key: IDENTIFIER;
config_value
    : DIGITS_IDENTIFIER unit?
    | DECIMAL_LITERAL unit?
    | FLOAT_LITERAL unit?
    | IDENTIFIER (',' IDENTIFIER)?
    | STRING_LITERAL
    ;

unit: 'rem'
    | 'px'
    | 'em'
    ;

declarations
    : config_decl
    | flow_decl
    | page_decl
    | style_decl
    | component_decl
    | library_decl
    | layout_decl
    ;

// Flow
flow_decl: FLOW IDENTIFIER LBRACE interaction_decl* RBRACE;

interaction_decl
    : see_decl
    | do_decl
    | react_decl
    ;

see_decl: SEE (IDENTIFIER | STRING_LITERAL DOT component_name);
do_decl: DO LBRACK action_name RBRACK STRING_LITERAL DOT component_name ;
react_decl: REACT scene_name? COLON react_action animate_decl?;

animate_decl: WITHTEXT ANIMATE LPAREN animate_name RPAREN;

react_action
    : goto_action
    | show_action
    ;

goto_action: GOTO_KEY component_name;
show_action: SHOW_KEY STRING_LITERAL DOT component_name;

action_name: IDENTIFIER;
component_value: IDENTIFIER;
component_name: IDENTIFIER;
scene_name: IDENTIFIER;
animate_name: IDENTIFIER;

GOTO_KEY: 'goto' | 'GOTO' | '跳转';
SHOW_KEY: 'show' | 'SHOW' | '展示';

FLOW: 'flow' | '流' ;
SEE: 'see' | 'SEE' | '看到';
DO: 'do' | 'DO' | '做';
REACT: 'react' | 'REACT' | '响应';

WITHTEXT: 'with' | 'WITH' | '使用';
ANIMATE: 'animate' | 'ANIMATE' | '动画';

//PAGE

page_decl: PAGE IDENTIFIER LBRACE component_body_decl* RBRACE;
component_decl: COMPONENT IDENTIFIER LBRACE component_body_decl* RBRACE;

component_body_decl
    : component_name (',' component_name)*
    | config_key COLON config_value
    ;

layout_decl: LAYOUT IDENTIFIER LBRACE layout_row* RBRACE;


REPEAT: 'repeat';
REPEAT_TIMES: INTEGER;

layout_row
    : '-' '-'*
    | layout_lines  '|'
    ;

layout_lines: layout_line layout_line*;
layout_line: '|' component_use_decl;

component_use_decl
    : DECIMAL_LITERAL
    | POSITION
    | component_name (LPAREN component_layout_value RPAREN)?
    | STRING_LITERAL
    ;

component_layout_value: DIGITS_IDENTIFIER | POSITION | STRING_LITERAL;

LAYOUT: 'layout' | 'Layout' | '布局';
POSITION: 'LEFT' | 'RIGHT' | 'TOP' | 'BOTTOM';

PAGE: 'page' | 'PAGE' | '页面';
COMPONENT: 'component' | 'COMPONENT' | '组件';

// STYLE

style_decl: STYLE style_name LBRACE style_body RBRACE;

style_name: IDENTIFIER;
style_body: (config_decl ';')*;

STYLE: 'style' | 'STYLE' | 'CSS' | 'css';

// LIBRARY


library_decl: LIBRARY library_name LBRACE library_exp* RBRACE;

library_exp
    : preset_key '=' (preset_value |preset_array) ';'?
    | preset_key '{' keyValue* '}'
    ;

keyValue: preset_key '=' preset_value;

preset_key: IDENTIFIER;
preset_value: config_value;
preset_array: LBRACK preset_call (',' preset_call)* RBRACK;

preset_call: library_name DOT IDENTIFIER;

library_name: IDENTIFIER;

LIBRARY: 'library' | 'LIBRARY' | '库';


// WORD

STRING_LITERAL:     '"' (~["\\\r\n] | EscapeSequence)* '"';
WS:                 [ \t\r\n\u000C]+ -> channel(HIDDEN);
COMMENT:            '/*' .*? '*/'    -> channel(HIDDEN);
LINE_COMMENT:       '//' ~[\r\n]*    -> channel(HIDDEN);

EmptyLine:          NewLine Space+ NewLine -> skip;
Space :             [ \t];
NewLine :           '\r\n' | '\n' | '\r';

LPAREN:             '(';
RPAREN:             ')';
LBRACE:             '{';
RBRACE:             '}';
LBRACK:             '[';
RBRACK:             ']';
Quote:              '"';
SingleQuote:        '\'';
COLON:              ':';
DOT:                '.';
COMMA:              ',';


LETTER:             Letter;
IDENTIFIER:         Letter LetterOrDigit*;
DIGITS:             Digits;
DIGITS_IDENTIFIER:  LetterOrDigit LetterOrDigit*;

DECIMAL_LITERAL:    ('0' | [1-9] (Digits? | '_'+ Digits)) [lL]?;

fragment DIGIT
    :'0'..'9'
    ;

fragment ExponentPart
    : [eE] [+-]? Digits
    ;


FLOAT_LITERAL
    : (Digits '.' Digits? | '.' Digits) ExponentPart? [fFdD]?
    | Digits (ExponentPart [fFdD]? | [fFdD])
    ;

fragment INTEGER
    : DIGIT+
    ;

fragment EscapeSequence
    : '\\' [btnfr"'\\]
    | '\\' ([0-3]? [0-7])? [0-7]
    | '\\' 'u'+ HexDigit HexDigit HexDigit HexDigit
    ;

fragment HexDigit
    : [0-9a-fA-F]
    ;

fragment Digits
    : [0-9] ([0-9_]* [0-9])?
    | ('0' .. '9') + ('.' ('0' .. '9') +)?
    ;

fragment LetterOrDigit
    : Letter
    | Digits
    ;

fragment Letter
    : [a-zA-Z$_] // these are the "java letters" below 0x7F
    | ~[\u0000-\u007F\uD800-\uDBFF] // covers all characters above 0x7F which are not a surrogate
    | [\uD800-\uDBFF] [\uDC00-\uDFFF] // covers UTF-16 surrogate pairs encodings for U+10000 to U+10FFFF
    ;
