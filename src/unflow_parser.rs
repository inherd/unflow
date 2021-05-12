use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::InputStream;
use antlr_rust::parser_rule_context::BaseParserRuleContext;
use antlr_rust::token_factory::ArenaCommonFactory;
use antlr_rust::tree::{ParseTree, ParseTreeVisitor, Tree, Visitable};
use serde::{Deserialize, Serialize};

use crate::{Component_body_declContextAll, Component_declContext, Component_nameContextAll, Component_use_declContextAll, Config_declContext, DesignLexer, DesignParser, DesignParserContextType, DesignVisitor, Do_declContext, Do_declContextExt, Flex_childContextAll, FlexCell, FlexChild, Flow_declContext, Goto_actionContext, Interaction_declContextAll, Key_valueContextAll, Layout_declContext, Library_declContext, Library_expContextAll, PresetCall, React_actionContextAll, React_declContext, React_declContextExt, See_declContext, See_declContextExt, Show_actionContext, UiLayout, UiLibraryPreset, Component_parameterContextAll};
use crate::{
    Animate_declContextAttrs,
    Component_body_configContextAttrs,
    Component_body_nameContextAttrs,
    Component_declContextAttrs,
    Component_use_decimalContextAttrs,
    Component_use_name_valueContextAttrs,
    Component_use_positionContextAttrs,
    Component_use_stringContextAttrs,
    Config_declContextAttrs,
    Do_declContextAttrs,
    Flex_component_useContextAttrs,
    Flow_declContextAttrs,
    Goto_actionContextAttrs,
    Interaction_declContextAttrs,
    Key_valueContextAttrs,
    Layout_declContextAttrs,
    Library_configContextAttrs,
    Library_declContextAttrs,
    Library_objectContextAttrs,
    React_actionContextAttrs,
    React_declContextAttrs,
    See_declContextAttrs,
    Show_actionContextAttrs,
};
use crate::ui_interaction::{DoInteraction, ReactInteraction, SeeInteraction, UiInteraction};
use crate::ui_library::{Component, UiFlow, UiLibrary};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Unflow {
    pub config: HashMap<String, String>,
    pub flows: Vec<UiFlow>,
    pub components: Vec<Component>,
    pub libraries: Vec<UiLibrary>,
    pub layouts: Vec<UiLayout>,
}

impl Default for Unflow {
    fn default() -> Self {
        Unflow {
            config: Default::default(),
            flows: vec![],
            components: vec![],
            libraries: vec![],
            layouts: vec![],
        }
    }
}

pub struct UnflowParser<'i> {
    #[allow(dead_code)]
    _inputs: Vec<&'i str>,
    pub(crate) flow: Unflow,
}

pub fn str_to_flow<'input>(data: &str) -> Unflow {
    let tf = ArenaCommonFactory::default();
    let lexer = DesignLexer::new_with_token_factory(InputStream::new(data.into()), &tf);
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = DesignParser::new(token_source);
    let result = parser.start().expect("parsed unsuccessfully");

    let mut unflow = UnflowParser {
        _inputs: vec![],
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
        let mut current_interaction = UiInteraction::default();

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
                        current_interaction = UiInteraction::default();
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

    fn visit_layout_decl(&mut self, ctx: &Layout_declContext<'i>) {
        let mut layout = UiLayout::default();
        layout.name = ctx.IDENTIFIER().unwrap().get_text();

        let decls: Vec<Rc<Flex_childContextAll<'i>>> = ctx.flex_child_all();
        for decl in &decls {
            if let Flex_childContextAll::Flex_component_useContext(uses) = decl.deref() {
                let mut flex_child = FlexChild::default();
                let components: Vec<Rc<Component_use_declContextAll<'i>>> = uses.component_use_decl_all();
                for component in components {
                    let cell = <UnflowParser<'i>>::build_layout_cell(component);
                    flex_child.cells.push(cell);
                }

                layout.flex_childs.push(flex_child);
            }
        }

        self.flow.layouts.push(layout);
    }

    fn visit_library_decl(&mut self, ctx: &Library_declContext<'i>) {
        let mut library = UiLibrary::default();
        library.name = ctx.library_name().unwrap().get_text();

        let exps: Vec<Rc<Library_expContextAll<'i>>> = ctx.library_exp_all();
        for exp in &exps {
            let mut preset = UiLibraryPreset::default();
            match exp.deref() {
                Library_expContextAll::Library_objectContext(lib_ctx) => {
                    preset.key = lib_ctx.preset_key().unwrap().get_text();
                    let key_values: Vec<Rc<Key_valueContextAll<'i>>> = lib_ctx.key_value_all();

                    for obj in key_values {
                        let mut preset_call = PresetCall::default();

                        preset_call.name = obj.preset_key().unwrap().get_text();
                        preset_call.preset = remove_quote(obj.preset_value().unwrap().get_text());

                        preset.preset_calls.push(preset_call);
                    }
                }
                Library_expContextAll::Library_configContext(con_ctx) => {
                    preset.key = con_ctx.preset_key().unwrap().get_text();
                    if let Some(value) = con_ctx.preset_value() {
                        preset.value = value.get_text();
                    }
                }
                Library_expContextAll::Error(_) => {}
            }

            library.presets.push(preset);
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
                see_inter.data = remove_quote(text);
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
        if let Some(act) = &decl.react_action() {
            let action: &Rc<React_actionContextAll<'c>> = act;
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

    fn build_layout_cell(component: Rc<Component_use_declContextAll>) -> FlexCell {
        let mut cell = FlexCell::default();
        match component.deref() {
            Component_use_declContextAll::Component_use_name_valueContext(sub_ctx) => {
                cell.component_name = sub_ctx.component_name().unwrap().get_text();
                let params: Vec<Rc<Component_parameterContextAll>> = sub_ctx.component_parameter_all();
                for param in params {
                    cell.parameters.push(param.get_text());
                }
            }
            Component_use_declContextAll::Component_use_decimalContext(sub_ctx) => {
                cell.normal_info = "value".to_string();
                cell.component_name = remove_quote(sub_ctx.DECIMAL_LITERAL().unwrap().get_text());
            }
            Component_use_declContextAll::Component_use_stringContext(sub_ctx) => {
                cell.normal_info = "value".to_string();
                cell.component_name = remove_quote(sub_ctx.STRING_LITERAL().unwrap().get_text());
            }
            Component_use_declContextAll::Component_use_positionContext(sub_ctx) => {
                cell.normal_info = "value".to_string();
                cell.component_name = remove_quote(sub_ctx.POSITION().unwrap().get_text());
            }
            Component_use_declContextAll::Error(_) => {}
        }

        cell
    }
}

fn remove_quote(str: String) -> String {
    str[1..str.len() - 1].to_string()
}
