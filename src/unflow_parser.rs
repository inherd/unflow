use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::InputStream;
use antlr_rust::parser_rule_context::BaseParserRuleContext;
use antlr_rust::token_factory::ArenaCommonFactory;
use antlr_rust::tree::{ParseTree, ParseTreeVisitor, Tree, Visitable};
use serde::{Deserialize, Serialize};

use crate::{Component_body_declContextAll, Component_declContext, Component_nameContextAll, Config_declContext, DesignLexer, DesignParser, DesignParserContextType, DesignVisitor, Do_declContext, Do_declContextExt, Flow_declContext, Goto_actionContext, Interaction_declContextAll, Layout_declContext, Library_declContext, Library_expContextAll, React_declContext, React_declContextExt, See_declContext, See_declContextExt, Show_actionContext};
#[allow(unused_imports)]
use crate::{
    Animate_declContextAttrs,
    Component_body_configContextAttrs,
    Component_body_declContextAttrs,
    Component_body_nameContextAttrs,
    Component_declContextAttrs,
    Config_declContextAttrs,
    Config_keyContextAttrs,
    Do_declContextAttrs,
    Flow_declContextAttrs,
    Goto_actionContextAttrs,
    Interaction_declContextAttrs,
    Library_declContextAttrs,
    Library_objectContextAttrs,
    React_actionContextAttrs,
    React_declContextAttrs,
    See_declContextAttrs,
    Show_actionContextAttrs,
};
use crate::layout::{Flex, FlexCell, UiLayout};

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
            ui_event: "".to_string(),
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
pub struct UiLibrary {
    pub name: String,
    pub presets: Vec<UiLibraryPreset>
}

impl Default for UiLibrary {
    fn default() -> Self {
        UiLibrary {
            name: "".to_string(),
            presets: vec![]
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UiLibraryPreset {
    pub key: String,
    pub value: String,
    pub preset_calls: Vec<PresetCall>,
    pub sub_properties: Vec<UiProperty>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PresetCall {
    pub name: String,
    pub preset: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UiProperty {
    pub key: String,
    pub value: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UiFlow {
    pub name: String,
    pub interactions: Vec<Interaction>,
    pub layout: Vec<UiLayout>
}

impl UiFlow {
    fn new(name: String) -> Self {
        UiFlow {
            name,
            interactions: vec![],
            layout: vec![]
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Component {
    pub name: String,
    pub child_components: Vec<String>,
    pub configs: HashMap<String, String>,
}

impl Default for Component {
    fn default() -> Self {
        Component {
            name: "".to_string(),
            child_components: vec![],
            configs: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Unflow {
    pub config: HashMap<String, String>,
    pub flows: Vec<UiFlow>,
    pub components: Vec<Component>,
    pub libraries: Vec<UiLibrary>,
}

impl Default for Unflow {
    fn default() -> Self {
        Unflow {
            config: Default::default(),
            flows: vec![],
            components: vec![],
            libraries: vec![]
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
                    current_interaction.ui_see = <UnflowParser<'i>>::build_see_interaction(&see_decl);
                }
                "antlr_rust::parser_rule_context::BaseParserRuleContext<unflow::grammar::designparser::Do_declContextExt>" => {
                    let do_decl = decl.do_decl().unwrap() as Rc<Do_declContext<'i>>;
                    current_interaction.ui_do = <UnflowParser<'i>>::build_do_interaction(&do_decl);
                }
                "antlr_rust::parser_rule_context::BaseParserRuleContext<unflow::grammar::designparser::React_declContextExt>" => {
                    let react_decl = decl.react_decl().unwrap() as Rc<React_declContext<'i>>;
                    let interaction = <UnflowParser<'i>>::build_react_interaction(&react_decl);
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

    fn visit_component_decl(&mut self, ctx: &Component_declContext<'i>) {
        let mut component = Component::default();
        let component_name = ctx.IDENTIFIER().unwrap().get_text();
        component.name = component_name;

        let decls: Vec<Rc<Component_body_declContextAll<'i>>> = ctx.component_body_decl_all();
        for decl in &decls {
            match decl.deref() {
                Component_body_declContextAll::Component_body_configContext(sub_ctx) => {
                    let key = sub_ctx.config_key().unwrap().get_text();
                    let value = sub_ctx.config_value().unwrap().get_text();

                    component.configs.insert(key, value);
                }
                Component_body_declContextAll::Component_body_nameContext(sub_ctx) => {
                    let names: Vec<Rc<Component_nameContextAll<'i>>> = sub_ctx.component_name_all();
                    for name in names {
                        component.child_components.push(name.get_text());
                    }
                }
                Component_body_declContextAll::Error(_) => {}
            }
        }

        self.flow.components.push(component);
    }

    fn visit_layout_decl(&mut self, _ctx: &Layout_declContext<'i>) {

    }

    fn visit_library_decl(&mut self, ctx: &Library_declContext<'i>) {
        let mut library = UiLibrary::default();
        library.name = ctx.library_name().unwrap().get_text();

        let exps: Vec<Rc<Library_expContextAll<'i>>> = ctx.library_exp_all();
        for exp in exps {
            match exp.deref() {
                Library_expContextAll::Library_objectContext(_) => {}
                Library_expContextAll::Library_configContext(_) => {}
                Library_expContextAll::Error(_) => {}
            }
        }

        self.flow.libraries.push(library);
    }
}


impl<'i> UnflowParser<'i> {
    fn build_do_interaction<'c>(do_decl: &Rc<BaseParserRuleContext<'c, Do_declContextExt<'c>>>) -> DoInteraction {
        let text: String = do_decl.STRING_LITERAL().unwrap().get_text();
        let without_quote: &str = &text[1..text.len() - 1];

        let do_inter = DoInteraction {
            component_name: do_decl.component_name().unwrap().get_text(),
            data: without_quote.to_string(),
            ui_event: do_decl.action_name().unwrap().get_text(),
        };
        do_inter
    }

    fn build_see_interaction<'c>(see_decl: &Rc<BaseParserRuleContext<'c, See_declContextExt<'c>>>) -> SeeInteraction {
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
        see_inter
    }

    fn build_react_interaction<'c>(decl: &Rc<BaseParserRuleContext<'c, React_declContextExt<'c>>>) -> ReactInteraction {
        let mut scene_name = "".to_string();
        if let Some(_) = decl.scene_name() {
            scene_name = decl.scene_name().unwrap().get_text();
        }

        let mut animate_name = "".to_string();

        if let Some(animate) = decl.animate_decl() {
            let type_name = format!("{:?}", animate);
            if type_name.as_str().contains("Animate_declContextExt") {
                animate_name = animate.animate_name().unwrap().get_text();
            }
        }

        let mut react_component_data = "".to_string();
        let mut react_action = "".to_string();
        let mut react_component_name = "".to_string();
        if let Some(action) = &decl.react_action() {
            let sub_action = action.get_child(0).unwrap();
            let type_name = format!("{:?}", sub_action);

            if type_name.as_str().contains("Show_actionContextExt") {
                let show = action.show_action().unwrap() as Rc<Show_actionContext<'c>>;
                let text: String = show.STRING_LITERAL().unwrap().get_text();
                let without_quote: &str = &text[1..text.len() - 1];

                react_action = "SHOW".to_string();
                react_component_data = without_quote.to_string();

                react_component_name = show.component_name().unwrap().get_text();
            }

            if type_name.as_str().contains("Goto_actionContextExt") {
                let goto = action.goto_action().unwrap() as Rc<Goto_actionContext<'c>>;
                react_action = "GOTO".to_string();
                react_component_name = goto.component_name().unwrap().get_text();
            }
        }

        let interaction = ReactInteraction {
            scene_name,
            react_action,
            react_component_name,
            animate_name,
            react_component_data,
        };

        interaction
    }
}
