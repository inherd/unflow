#![allow(nonstandard_style)]
// Generated from src/grammar/Design.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor};
use super::designparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link DesignParser}.
 */
pub trait DesignVisitor<'input>: ParseTreeVisitor<'input,DesignParserContextType>{
	/**
	 * Visit a parse tree produced by {@link DesignParser#start}.
	 * @param ctx the parse tree
	 */
	fn visit_start(&mut self, ctx: &StartContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#comment}.
	 * @param ctx the parse tree
	 */
	fn visit_comment(&mut self, ctx: &CommentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#config_decl}.
	 * @param ctx the parse tree
	 */
	fn visit_config_decl(&mut self, ctx: &Config_declContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#config_key}.
	 * @param ctx the parse tree
	 */
	fn visit_config_key(&mut self, ctx: &Config_keyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#config_value}.
	 * @param ctx the parse tree
	 */
	fn visit_config_value(&mut self, ctx: &Config_valueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#unit}.
	 * @param ctx the parse tree
	 */
	fn visit_unit(&mut self, ctx: &UnitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#declarations}.
	 * @param ctx the parse tree
	 */
	fn visit_declarations(&mut self, ctx: &DeclarationsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#flow_decl}.
	 * @param ctx the parse tree
	 */
	fn visit_flow_decl(&mut self, ctx: &Flow_declContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#interaction_decl}.
	 * @param ctx the parse tree
	 */
	fn visit_interaction_decl(&mut self, ctx: &Interaction_declContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#see_decl}.
	 * @param ctx the parse tree
	 */
	fn visit_see_decl(&mut self, ctx: &See_declContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#do_decl}.
	 * @param ctx the parse tree
	 */
	fn visit_do_decl(&mut self, ctx: &Do_declContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#react_decl}.
	 * @param ctx the parse tree
	 */
	fn visit_react_decl(&mut self, ctx: &React_declContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#animate_decl}.
	 * @param ctx the parse tree
	 */
	fn visit_animate_decl(&mut self, ctx: &Animate_declContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#react_action}.
	 * @param ctx the parse tree
	 */
	fn visit_react_action(&mut self, ctx: &React_actionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#goto_action}.
	 * @param ctx the parse tree
	 */
	fn visit_goto_action(&mut self, ctx: &Goto_actionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#show_action}.
	 * @param ctx the parse tree
	 */
	fn visit_show_action(&mut self, ctx: &Show_actionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#action_name}.
	 * @param ctx the parse tree
	 */
	fn visit_action_name(&mut self, ctx: &Action_nameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#component_value}.
	 * @param ctx the parse tree
	 */
	fn visit_component_value(&mut self, ctx: &Component_valueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#component_name}.
	 * @param ctx the parse tree
	 */
	fn visit_component_name(&mut self, ctx: &Component_nameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#scene_name}.
	 * @param ctx the parse tree
	 */
	fn visit_scene_name(&mut self, ctx: &Scene_nameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#animate_name}.
	 * @param ctx the parse tree
	 */
	fn visit_animate_name(&mut self, ctx: &Animate_nameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#page_decl}.
	 * @param ctx the parse tree
	 */
	fn visit_page_decl(&mut self, ctx: &Page_declContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#component_decl}.
	 * @param ctx the parse tree
	 */
	fn visit_component_decl(&mut self, ctx: &Component_declContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code component_body_name}
	 * labeled alternative in {@link DesignParser#component_body_decl}.
	 * @param ctx the parse tree
	 */
	fn visit_component_body_name(&mut self, ctx: &Component_body_nameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code component_body_config}
	 * labeled alternative in {@link DesignParser#component_body_decl}.
	 * @param ctx the parse tree
	 */
	fn visit_component_body_config(&mut self, ctx: &Component_body_configContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#layout_decl}.
	 * @param ctx the parse tree
	 */
	fn visit_layout_decl(&mut self, ctx: &Layout_declContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code empty_line}
	 * labeled alternative in {@link DesignParser#flex_child}.
	 * @param ctx the parse tree
	 */
	fn visit_empty_line(&mut self, ctx: &Empty_lineContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code flex_layout_lines}
	 * labeled alternative in {@link DesignParser#flex_child}.
	 * @param ctx the parse tree
	 */
	fn visit_flex_layout_lines(&mut self, ctx: &Flex_layout_linesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#layout_lines}.
	 * @param ctx the parse tree
	 */
	fn visit_layout_lines(&mut self, ctx: &Layout_linesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#layout_line}.
	 * @param ctx the parse tree
	 */
	fn visit_layout_line(&mut self, ctx: &Layout_lineContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code component_use_decimal}
	 * labeled alternative in {@link DesignParser#component_use_decl}.
	 * @param ctx the parse tree
	 */
	fn visit_component_use_decimal(&mut self, ctx: &Component_use_decimalContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code component_use_position}
	 * labeled alternative in {@link DesignParser#component_use_decl}.
	 * @param ctx the parse tree
	 */
	fn visit_component_use_position(&mut self, ctx: &Component_use_positionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code component_use_name_value}
	 * labeled alternative in {@link DesignParser#component_use_decl}.
	 * @param ctx the parse tree
	 */
	fn visit_component_use_name_value(&mut self, ctx: &Component_use_name_valueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code component_use_string}
	 * labeled alternative in {@link DesignParser#component_use_decl}.
	 * @param ctx the parse tree
	 */
	fn visit_component_use_string(&mut self, ctx: &Component_use_stringContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#component_layout_value}.
	 * @param ctx the parse tree
	 */
	fn visit_component_layout_value(&mut self, ctx: &Component_layout_valueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#style_decl}.
	 * @param ctx the parse tree
	 */
	fn visit_style_decl(&mut self, ctx: &Style_declContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#style_name}.
	 * @param ctx the parse tree
	 */
	fn visit_style_name(&mut self, ctx: &Style_nameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#style_body}.
	 * @param ctx the parse tree
	 */
	fn visit_style_body(&mut self, ctx: &Style_bodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#library_decl}.
	 * @param ctx the parse tree
	 */
	fn visit_library_decl(&mut self, ctx: &Library_declContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code library_config}
	 * labeled alternative in {@link DesignParser#library_exp}.
	 * @param ctx the parse tree
	 */
	fn visit_library_config(&mut self, ctx: &Library_configContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code library_object}
	 * labeled alternative in {@link DesignParser#library_exp}.
	 * @param ctx the parse tree
	 */
	fn visit_library_object(&mut self, ctx: &Library_objectContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#key_value}.
	 * @param ctx the parse tree
	 */
	fn visit_key_value(&mut self, ctx: &Key_valueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#preset_key}.
	 * @param ctx the parse tree
	 */
	fn visit_preset_key(&mut self, ctx: &Preset_keyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#preset_value}.
	 * @param ctx the parse tree
	 */
	fn visit_preset_value(&mut self, ctx: &Preset_valueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#preset_array}.
	 * @param ctx the parse tree
	 */
	fn visit_preset_array(&mut self, ctx: &Preset_arrayContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#preset_call}.
	 * @param ctx the parse tree
	 */
	fn visit_preset_call(&mut self, ctx: &Preset_callContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link DesignParser#library_name}.
	 * @param ctx the parse tree
	 */
	fn visit_library_name(&mut self, ctx: &Library_nameContext<'input>) { self.visit_children(ctx) }


}