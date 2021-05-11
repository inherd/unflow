#![allow(nonstandard_style)]
// Generated from src/grammar/Design.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::designparser::*;

pub trait DesignListener<'input> : ParseTreeListener<'input,DesignParserContextType>{

/**
 * Enter a parse tree produced by {@link DesignParser#start}.
 * @param ctx the parse tree
 */
fn enter_start(&mut self, _ctx: &StartContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#start}.
 * @param ctx the parse tree
 */
fn exit_start(&mut self, _ctx: &StartContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#comment}.
 * @param ctx the parse tree
 */
fn enter_comment(&mut self, _ctx: &CommentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#comment}.
 * @param ctx the parse tree
 */
fn exit_comment(&mut self, _ctx: &CommentContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#config_decl}.
 * @param ctx the parse tree
 */
fn enter_config_decl(&mut self, _ctx: &Config_declContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#config_decl}.
 * @param ctx the parse tree
 */
fn exit_config_decl(&mut self, _ctx: &Config_declContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#config_key}.
 * @param ctx the parse tree
 */
fn enter_config_key(&mut self, _ctx: &Config_keyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#config_key}.
 * @param ctx the parse tree
 */
fn exit_config_key(&mut self, _ctx: &Config_keyContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#config_value}.
 * @param ctx the parse tree
 */
fn enter_config_value(&mut self, _ctx: &Config_valueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#config_value}.
 * @param ctx the parse tree
 */
fn exit_config_value(&mut self, _ctx: &Config_valueContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#unit}.
 * @param ctx the parse tree
 */
fn enter_unit(&mut self, _ctx: &UnitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#unit}.
 * @param ctx the parse tree
 */
fn exit_unit(&mut self, _ctx: &UnitContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#declarations}.
 * @param ctx the parse tree
 */
fn enter_declarations(&mut self, _ctx: &DeclarationsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#declarations}.
 * @param ctx the parse tree
 */
fn exit_declarations(&mut self, _ctx: &DeclarationsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#flow_decl}.
 * @param ctx the parse tree
 */
fn enter_flow_decl(&mut self, _ctx: &Flow_declContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#flow_decl}.
 * @param ctx the parse tree
 */
fn exit_flow_decl(&mut self, _ctx: &Flow_declContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#interaction_decl}.
 * @param ctx the parse tree
 */
fn enter_interaction_decl(&mut self, _ctx: &Interaction_declContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#interaction_decl}.
 * @param ctx the parse tree
 */
fn exit_interaction_decl(&mut self, _ctx: &Interaction_declContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#see_decl}.
 * @param ctx the parse tree
 */
fn enter_see_decl(&mut self, _ctx: &See_declContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#see_decl}.
 * @param ctx the parse tree
 */
fn exit_see_decl(&mut self, _ctx: &See_declContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#do_decl}.
 * @param ctx the parse tree
 */
fn enter_do_decl(&mut self, _ctx: &Do_declContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#do_decl}.
 * @param ctx the parse tree
 */
fn exit_do_decl(&mut self, _ctx: &Do_declContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#react_decl}.
 * @param ctx the parse tree
 */
fn enter_react_decl(&mut self, _ctx: &React_declContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#react_decl}.
 * @param ctx the parse tree
 */
fn exit_react_decl(&mut self, _ctx: &React_declContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#animate_decl}.
 * @param ctx the parse tree
 */
fn enter_animate_decl(&mut self, _ctx: &Animate_declContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#animate_decl}.
 * @param ctx the parse tree
 */
fn exit_animate_decl(&mut self, _ctx: &Animate_declContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#react_action}.
 * @param ctx the parse tree
 */
fn enter_react_action(&mut self, _ctx: &React_actionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#react_action}.
 * @param ctx the parse tree
 */
fn exit_react_action(&mut self, _ctx: &React_actionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#goto_action}.
 * @param ctx the parse tree
 */
fn enter_goto_action(&mut self, _ctx: &Goto_actionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#goto_action}.
 * @param ctx the parse tree
 */
fn exit_goto_action(&mut self, _ctx: &Goto_actionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#show_action}.
 * @param ctx the parse tree
 */
fn enter_show_action(&mut self, _ctx: &Show_actionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#show_action}.
 * @param ctx the parse tree
 */
fn exit_show_action(&mut self, _ctx: &Show_actionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#action_name}.
 * @param ctx the parse tree
 */
fn enter_action_name(&mut self, _ctx: &Action_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#action_name}.
 * @param ctx the parse tree
 */
fn exit_action_name(&mut self, _ctx: &Action_nameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#component_value}.
 * @param ctx the parse tree
 */
fn enter_component_value(&mut self, _ctx: &Component_valueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#component_value}.
 * @param ctx the parse tree
 */
fn exit_component_value(&mut self, _ctx: &Component_valueContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#component_name}.
 * @param ctx the parse tree
 */
fn enter_component_name(&mut self, _ctx: &Component_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#component_name}.
 * @param ctx the parse tree
 */
fn exit_component_name(&mut self, _ctx: &Component_nameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#scene_name}.
 * @param ctx the parse tree
 */
fn enter_scene_name(&mut self, _ctx: &Scene_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#scene_name}.
 * @param ctx the parse tree
 */
fn exit_scene_name(&mut self, _ctx: &Scene_nameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#animate_name}.
 * @param ctx the parse tree
 */
fn enter_animate_name(&mut self, _ctx: &Animate_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#animate_name}.
 * @param ctx the parse tree
 */
fn exit_animate_name(&mut self, _ctx: &Animate_nameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#page_decl}.
 * @param ctx the parse tree
 */
fn enter_page_decl(&mut self, _ctx: &Page_declContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#page_decl}.
 * @param ctx the parse tree
 */
fn exit_page_decl(&mut self, _ctx: &Page_declContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#component_decl}.
 * @param ctx the parse tree
 */
fn enter_component_decl(&mut self, _ctx: &Component_declContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#component_decl}.
 * @param ctx the parse tree
 */
fn exit_component_decl(&mut self, _ctx: &Component_declContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code component_body_name}
 * labeled alternative in {@link DesignParser#component_body_decl}.
 * @param ctx the parse tree
 */
fn enter_component_body_name(&mut self, _ctx: &Component_body_nameContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code component_body_name}
 * labeled alternative in {@link DesignParser#component_body_decl}.
 * @param ctx the parse tree
 */
fn exit_component_body_name(&mut self, _ctx: &Component_body_nameContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code component_body_config}
 * labeled alternative in {@link DesignParser#component_body_decl}.
 * @param ctx the parse tree
 */
fn enter_component_body_config(&mut self, _ctx: &Component_body_configContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code component_body_config}
 * labeled alternative in {@link DesignParser#component_body_decl}.
 * @param ctx the parse tree
 */
fn exit_component_body_config(&mut self, _ctx: &Component_body_configContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#layout_decl}.
 * @param ctx the parse tree
 */
fn enter_layout_decl(&mut self, _ctx: &Layout_declContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#layout_decl}.
 * @param ctx the parse tree
 */
fn exit_layout_decl(&mut self, _ctx: &Layout_declContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code empty_line}
 * labeled alternative in {@link DesignParser#flex_child}.
 * @param ctx the parse tree
 */
fn enter_empty_line(&mut self, _ctx: &Empty_lineContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code empty_line}
 * labeled alternative in {@link DesignParser#flex_child}.
 * @param ctx the parse tree
 */
fn exit_empty_line(&mut self, _ctx: &Empty_lineContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code flex_layout_lines}
 * labeled alternative in {@link DesignParser#flex_child}.
 * @param ctx the parse tree
 */
fn enter_flex_layout_lines(&mut self, _ctx: &Flex_layout_linesContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code flex_layout_lines}
 * labeled alternative in {@link DesignParser#flex_child}.
 * @param ctx the parse tree
 */
fn exit_flex_layout_lines(&mut self, _ctx: &Flex_layout_linesContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#layout_lines}.
 * @param ctx the parse tree
 */
fn enter_layout_lines(&mut self, _ctx: &Layout_linesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#layout_lines}.
 * @param ctx the parse tree
 */
fn exit_layout_lines(&mut self, _ctx: &Layout_linesContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#layout_line}.
 * @param ctx the parse tree
 */
fn enter_layout_line(&mut self, _ctx: &Layout_lineContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#layout_line}.
 * @param ctx the parse tree
 */
fn exit_layout_line(&mut self, _ctx: &Layout_lineContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code component_use_decimal}
 * labeled alternative in {@link DesignParser#component_use_decl}.
 * @param ctx the parse tree
 */
fn enter_component_use_decimal(&mut self, _ctx: &Component_use_decimalContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code component_use_decimal}
 * labeled alternative in {@link DesignParser#component_use_decl}.
 * @param ctx the parse tree
 */
fn exit_component_use_decimal(&mut self, _ctx: &Component_use_decimalContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code component_use_position}
 * labeled alternative in {@link DesignParser#component_use_decl}.
 * @param ctx the parse tree
 */
fn enter_component_use_position(&mut self, _ctx: &Component_use_positionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code component_use_position}
 * labeled alternative in {@link DesignParser#component_use_decl}.
 * @param ctx the parse tree
 */
fn exit_component_use_position(&mut self, _ctx: &Component_use_positionContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code component_use_name_value}
 * labeled alternative in {@link DesignParser#component_use_decl}.
 * @param ctx the parse tree
 */
fn enter_component_use_name_value(&mut self, _ctx: &Component_use_name_valueContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code component_use_name_value}
 * labeled alternative in {@link DesignParser#component_use_decl}.
 * @param ctx the parse tree
 */
fn exit_component_use_name_value(&mut self, _ctx: &Component_use_name_valueContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code component_use_string}
 * labeled alternative in {@link DesignParser#component_use_decl}.
 * @param ctx the parse tree
 */
fn enter_component_use_string(&mut self, _ctx: &Component_use_stringContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code component_use_string}
 * labeled alternative in {@link DesignParser#component_use_decl}.
 * @param ctx the parse tree
 */
fn exit_component_use_string(&mut self, _ctx: &Component_use_stringContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#component_layout_value}.
 * @param ctx the parse tree
 */
fn enter_component_layout_value(&mut self, _ctx: &Component_layout_valueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#component_layout_value}.
 * @param ctx the parse tree
 */
fn exit_component_layout_value(&mut self, _ctx: &Component_layout_valueContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#style_decl}.
 * @param ctx the parse tree
 */
fn enter_style_decl(&mut self, _ctx: &Style_declContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#style_decl}.
 * @param ctx the parse tree
 */
fn exit_style_decl(&mut self, _ctx: &Style_declContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#style_name}.
 * @param ctx the parse tree
 */
fn enter_style_name(&mut self, _ctx: &Style_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#style_name}.
 * @param ctx the parse tree
 */
fn exit_style_name(&mut self, _ctx: &Style_nameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#style_body}.
 * @param ctx the parse tree
 */
fn enter_style_body(&mut self, _ctx: &Style_bodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#style_body}.
 * @param ctx the parse tree
 */
fn exit_style_body(&mut self, _ctx: &Style_bodyContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#library_decl}.
 * @param ctx the parse tree
 */
fn enter_library_decl(&mut self, _ctx: &Library_declContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#library_decl}.
 * @param ctx the parse tree
 */
fn exit_library_decl(&mut self, _ctx: &Library_declContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code library_config}
 * labeled alternative in {@link DesignParser#library_exp}.
 * @param ctx the parse tree
 */
fn enter_library_config(&mut self, _ctx: &Library_configContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code library_config}
 * labeled alternative in {@link DesignParser#library_exp}.
 * @param ctx the parse tree
 */
fn exit_library_config(&mut self, _ctx: &Library_configContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code library_object}
 * labeled alternative in {@link DesignParser#library_exp}.
 * @param ctx the parse tree
 */
fn enter_library_object(&mut self, _ctx: &Library_objectContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code library_object}
 * labeled alternative in {@link DesignParser#library_exp}.
 * @param ctx the parse tree
 */
fn exit_library_object(&mut self, _ctx: &Library_objectContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#key_value}.
 * @param ctx the parse tree
 */
fn enter_key_value(&mut self, _ctx: &Key_valueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#key_value}.
 * @param ctx the parse tree
 */
fn exit_key_value(&mut self, _ctx: &Key_valueContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#preset_key}.
 * @param ctx the parse tree
 */
fn enter_preset_key(&mut self, _ctx: &Preset_keyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#preset_key}.
 * @param ctx the parse tree
 */
fn exit_preset_key(&mut self, _ctx: &Preset_keyContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#preset_value}.
 * @param ctx the parse tree
 */
fn enter_preset_value(&mut self, _ctx: &Preset_valueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#preset_value}.
 * @param ctx the parse tree
 */
fn exit_preset_value(&mut self, _ctx: &Preset_valueContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#preset_array}.
 * @param ctx the parse tree
 */
fn enter_preset_array(&mut self, _ctx: &Preset_arrayContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#preset_array}.
 * @param ctx the parse tree
 */
fn exit_preset_array(&mut self, _ctx: &Preset_arrayContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#preset_call}.
 * @param ctx the parse tree
 */
fn enter_preset_call(&mut self, _ctx: &Preset_callContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#preset_call}.
 * @param ctx the parse tree
 */
fn exit_preset_call(&mut self, _ctx: &Preset_callContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#library_name}.
 * @param ctx the parse tree
 */
fn enter_library_name(&mut self, _ctx: &Library_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#library_name}.
 * @param ctx the parse tree
 */
fn exit_library_name(&mut self, _ctx: &Library_nameContext<'input>) { }

}
