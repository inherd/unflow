use std::collections::HashMap;

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::InputStream;
use antlr_rust::token_factory::ArenaCommonFactory;
use antlr_rust::tree::{ParseTree, ParseTreeVisitor, Visitable, Tree};
use serde::{Deserialize, Serialize};

use crate::{Config_declContext, DesignLexer, DesignParser, DesignParserContextType, DesignVisitor, Flow_declContext, Interaction_declContextAll, See_declContext, Do_declContext, React_declContext};
#[allow(unused_imports)]
use crate::{
    Config_declContextAttrs,
    Flow_declContextAttrs,
    Interaction_declContextAttrs,
    See_declContextAttrs,
    Do_declContextAttrs,
    React_declContextAttrs,
};
use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Interaction {
    pub ui_do: DoInteraction,
    pub ui_see: SeeInteraction,
    pub ui_react: Vec<ReactInteraction>,
}

impl Default for Interaction {
    fn default() -> Self {
        Interaction {
            ui_do: Default::default(),
            ui_see: Default::default(),
            ui_react: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DoInteraction {
    pub component_name: String,
    pub data: String,
    pub ui_event: String,
}

impl Default for DoInteraction {
    fn default() -> Self {
        DoInteraction {
            component_name: "".to_string(),
            data: "".to_string(),
            ui_event: "".to_string()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SeeInteraction {
    pub ui_event: String,
    pub component_name: String,
    pub data: String,
}

impl Default for SeeInteraction {
    fn default() -> Self {
        SeeInteraction {
            ui_event: "".to_string(),
            component_name: "".to_string(),
            data: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReactInteraction {
    pub scene_name: String,
    pub react_action: String,
    pub react_component_name: String,
    pub animate_name: String,
    pub react_component_data: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UiFlow {
    pub name: String,
    pub interactions: Vec<Interaction>,
}

impl UiFlow {
    fn new(name: String) -> Self {
        UiFlow {
            name,
            interactions: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Unflow {
    pub config: HashMap<String, String>,
    pub flows: Vec<UiFlow>,
}

impl Default for Unflow {
    fn default() -> Self {
        Unflow {
            config: Default::default(),
            flows: vec![],
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
        flow: Default::default(),
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

    fn visit_flow_decl(&mut self, ctx: &Flow_declContext<'i>) {
        let flow_name = ctx.IDENTIFIER().unwrap().get_text();
        let mut flow = UiFlow::new(flow_name);

        let decls: Vec<Rc<Interaction_declContextAll<'i>>> = ctx.interaction_decl_all();
        let mut index = 0;
        let mut current_interaction = Interaction::default();

        for decl in &decls {
            let child = decl.get_child(0).unwrap();
            let type_name = format!("{:?}", child);

            match type_name.as_str() {
                "antlr_rust::parser_rule_context::BaseParserRuleContext<unflow::grammar::designparser::See_declContextExt>" => {
                    let see_decl = decl.see_decl().unwrap() as Rc<See_declContext<'i>>;
                    let mut see_inter = SeeInteraction::default();
                    match see_decl.IDENTIFIER() {
                        Some(ident_ctx) => { see_inter.component_name = ident_ctx.get_text() }
                        None => {
                            see_inter.component_name = see_decl.component_name().unwrap().get_text();
                            let text: String = see_decl.STRING_LITERAL().unwrap().get_text();
                            let without_quote: &str = &text[1..text.len() - 1];
                            see_inter.data = without_quote.to_string();
                        }
                    }

                    current_interaction.ui_see = see_inter;
                }
                "antlr_rust::parser_rule_context::BaseParserRuleContext<unflow::grammar::designparser::Do_declContextExt>" => {
                    let do_decl = decl.do_decl().unwrap() as Rc<Do_declContext<'i>>;

                    let text: String = do_decl.STRING_LITERAL().unwrap().get_text();
                    let without_quote: &str = &text[1..text.len() - 1];

                    let do_inter = DoInteraction {
                        component_name: do_decl.component_name().unwrap().get_text(),
                        data: without_quote.to_string(),
                        ui_event: do_decl.action_name().unwrap().get_text()
                    };

                    current_interaction.ui_do = do_inter;
                }
                "antlr_rust::parser_rule_context::BaseParserRuleContext<unflow::grammar::designparser::React_declContextExt>" => {
                    let react_decl = decl.react_decl().unwrap() as Rc<React_declContext<'i>>;

                    let mut scene_name = "".to_string();
                    if let Some(_) = react_decl.scene_name() {
                        scene_name = react_decl.scene_name().unwrap().get_text();
                    }

                    let animate_name = "".to_string();
                    // if let Some(_) = react_decl.animate_name() {
                    //     animate_name = react_decl.animate_name().unwrap().get_text();
                    // }

                    let interaction = ReactInteraction {
                        scene_name,
                        react_action: "".to_string(),
                        react_component_name: "".to_string(),
                        animate_name,
                        react_component_data: "".to_string()
                    };

                    current_interaction.ui_react.push(interaction);

                    let mut has_next_see = false;
                    if let Some(decl) = &decls.get(index + 1) {
                        let next_name = format!("{:?}", decl.get_child(0).unwrap());
                        if next_name.as_str().contains("See_declContextExt") {
                            has_next_see = true;
                        }
                    }

                    if has_next_see {
                        flow.interactions.push(current_interaction);
                        current_interaction = Interaction::default();
                    }
                }
                _ => {}
            }
            index = index + 1;
        }

        flow.interactions.push(current_interaction);
        self.flow.flows.push(flow);
    }
}