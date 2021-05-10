#![allow(nonstandard_style)]
// Generated from docs/dsl/Design.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::design_parser::*;

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
 * Enter a parse tree produced by {@link DesignParser#configDeclaration}.
 * @param ctx the parse tree
 */
fn enter_configDeclaration(&mut self, _ctx: &ConfigDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#configDeclaration}.
 * @param ctx the parse tree
 */
fn exit_configDeclaration(&mut self, _ctx: &ConfigDeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#configKey}.
 * @param ctx the parse tree
 */
fn enter_configKey(&mut self, _ctx: &ConfigKeyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#configKey}.
 * @param ctx the parse tree
 */
fn exit_configKey(&mut self, _ctx: &ConfigKeyContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#configValue}.
 * @param ctx the parse tree
 */
fn enter_configValue(&mut self, _ctx: &ConfigValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#configValue}.
 * @param ctx the parse tree
 */
fn exit_configValue(&mut self, _ctx: &ConfigValueContext<'input>) { }

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
 * Enter a parse tree produced by {@link DesignParser#decalartions}.
 * @param ctx the parse tree
 */
fn enter_decalartions(&mut self, _ctx: &DecalartionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#decalartions}.
 * @param ctx the parse tree
 */
fn exit_decalartions(&mut self, _ctx: &DecalartionsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#flowDeclaration}.
 * @param ctx the parse tree
 */
fn enter_flowDeclaration(&mut self, _ctx: &FlowDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#flowDeclaration}.
 * @param ctx the parse tree
 */
fn exit_flowDeclaration(&mut self, _ctx: &FlowDeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#interactionDeclaration}.
 * @param ctx the parse tree
 */
fn enter_interactionDeclaration(&mut self, _ctx: &InteractionDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#interactionDeclaration}.
 * @param ctx the parse tree
 */
fn exit_interactionDeclaration(&mut self, _ctx: &InteractionDeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#seeDeclaration}.
 * @param ctx the parse tree
 */
fn enter_seeDeclaration(&mut self, _ctx: &SeeDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#seeDeclaration}.
 * @param ctx the parse tree
 */
fn exit_seeDeclaration(&mut self, _ctx: &SeeDeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#doDeclaration}.
 * @param ctx the parse tree
 */
fn enter_doDeclaration(&mut self, _ctx: &DoDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#doDeclaration}.
 * @param ctx the parse tree
 */
fn exit_doDeclaration(&mut self, _ctx: &DoDeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#reactDeclaration}.
 * @param ctx the parse tree
 */
fn enter_reactDeclaration(&mut self, _ctx: &ReactDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#reactDeclaration}.
 * @param ctx the parse tree
 */
fn exit_reactDeclaration(&mut self, _ctx: &ReactDeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#animateDeclaration}.
 * @param ctx the parse tree
 */
fn enter_animateDeclaration(&mut self, _ctx: &AnimateDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#animateDeclaration}.
 * @param ctx the parse tree
 */
fn exit_animateDeclaration(&mut self, _ctx: &AnimateDeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#reactAction}.
 * @param ctx the parse tree
 */
fn enter_reactAction(&mut self, _ctx: &ReactActionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#reactAction}.
 * @param ctx the parse tree
 */
fn exit_reactAction(&mut self, _ctx: &ReactActionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#gotoAction}.
 * @param ctx the parse tree
 */
fn enter_gotoAction(&mut self, _ctx: &GotoActionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#gotoAction}.
 * @param ctx the parse tree
 */
fn exit_gotoAction(&mut self, _ctx: &GotoActionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#showAction}.
 * @param ctx the parse tree
 */
fn enter_showAction(&mut self, _ctx: &ShowActionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#showAction}.
 * @param ctx the parse tree
 */
fn exit_showAction(&mut self, _ctx: &ShowActionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#actionName}.
 * @param ctx the parse tree
 */
fn enter_actionName(&mut self, _ctx: &ActionNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#actionName}.
 * @param ctx the parse tree
 */
fn exit_actionName(&mut self, _ctx: &ActionNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#componentValue}.
 * @param ctx the parse tree
 */
fn enter_componentValue(&mut self, _ctx: &ComponentValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#componentValue}.
 * @param ctx the parse tree
 */
fn exit_componentValue(&mut self, _ctx: &ComponentValueContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#componentName}.
 * @param ctx the parse tree
 */
fn enter_componentName(&mut self, _ctx: &ComponentNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#componentName}.
 * @param ctx the parse tree
 */
fn exit_componentName(&mut self, _ctx: &ComponentNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#sceneName}.
 * @param ctx the parse tree
 */
fn enter_sceneName(&mut self, _ctx: &SceneNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#sceneName}.
 * @param ctx the parse tree
 */
fn exit_sceneName(&mut self, _ctx: &SceneNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#animateName}.
 * @param ctx the parse tree
 */
fn enter_animateName(&mut self, _ctx: &AnimateNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#animateName}.
 * @param ctx the parse tree
 */
fn exit_animateName(&mut self, _ctx: &AnimateNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#pageDeclaration}.
 * @param ctx the parse tree
 */
fn enter_pageDeclaration(&mut self, _ctx: &PageDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#pageDeclaration}.
 * @param ctx the parse tree
 */
fn exit_pageDeclaration(&mut self, _ctx: &PageDeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#componentDeclaration}.
 * @param ctx the parse tree
 */
fn enter_componentDeclaration(&mut self, _ctx: &ComponentDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#componentDeclaration}.
 * @param ctx the parse tree
 */
fn exit_componentDeclaration(&mut self, _ctx: &ComponentDeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#componentBodyDeclaration}.
 * @param ctx the parse tree
 */
fn enter_componentBodyDeclaration(&mut self, _ctx: &ComponentBodyDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#componentBodyDeclaration}.
 * @param ctx the parse tree
 */
fn exit_componentBodyDeclaration(&mut self, _ctx: &ComponentBodyDeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#layoutDeclaration}.
 * @param ctx the parse tree
 */
fn enter_layoutDeclaration(&mut self, _ctx: &LayoutDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#layoutDeclaration}.
 * @param ctx the parse tree
 */
fn exit_layoutDeclaration(&mut self, _ctx: &LayoutDeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#layoutRow}.
 * @param ctx the parse tree
 */
fn enter_layoutRow(&mut self, _ctx: &LayoutRowContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#layoutRow}.
 * @param ctx the parse tree
 */
fn exit_layoutRow(&mut self, _ctx: &LayoutRowContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#layoutLines}.
 * @param ctx the parse tree
 */
fn enter_layoutLines(&mut self, _ctx: &LayoutLinesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#layoutLines}.
 * @param ctx the parse tree
 */
fn exit_layoutLines(&mut self, _ctx: &LayoutLinesContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#layoutLine}.
 * @param ctx the parse tree
 */
fn enter_layoutLine(&mut self, _ctx: &LayoutLineContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#layoutLine}.
 * @param ctx the parse tree
 */
fn exit_layoutLine(&mut self, _ctx: &LayoutLineContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#componentUseDeclaration}.
 * @param ctx the parse tree
 */
fn enter_componentUseDeclaration(&mut self, _ctx: &ComponentUseDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#componentUseDeclaration}.
 * @param ctx the parse tree
 */
fn exit_componentUseDeclaration(&mut self, _ctx: &ComponentUseDeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#componentLayoutValue}.
 * @param ctx the parse tree
 */
fn enter_componentLayoutValue(&mut self, _ctx: &ComponentLayoutValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#componentLayoutValue}.
 * @param ctx the parse tree
 */
fn exit_componentLayoutValue(&mut self, _ctx: &ComponentLayoutValueContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#styleDeclaration}.
 * @param ctx the parse tree
 */
fn enter_styleDeclaration(&mut self, _ctx: &StyleDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#styleDeclaration}.
 * @param ctx the parse tree
 */
fn exit_styleDeclaration(&mut self, _ctx: &StyleDeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#styleName}.
 * @param ctx the parse tree
 */
fn enter_styleName(&mut self, _ctx: &StyleNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#styleName}.
 * @param ctx the parse tree
 */
fn exit_styleName(&mut self, _ctx: &StyleNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#styleBody}.
 * @param ctx the parse tree
 */
fn enter_styleBody(&mut self, _ctx: &StyleBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#styleBody}.
 * @param ctx the parse tree
 */
fn exit_styleBody(&mut self, _ctx: &StyleBodyContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#libraryDeclaration}.
 * @param ctx the parse tree
 */
fn enter_libraryDeclaration(&mut self, _ctx: &LibraryDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#libraryDeclaration}.
 * @param ctx the parse tree
 */
fn exit_libraryDeclaration(&mut self, _ctx: &LibraryDeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#libraryExpress}.
 * @param ctx the parse tree
 */
fn enter_libraryExpress(&mut self, _ctx: &LibraryExpressContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#libraryExpress}.
 * @param ctx the parse tree
 */
fn exit_libraryExpress(&mut self, _ctx: &LibraryExpressContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#keyValue}.
 * @param ctx the parse tree
 */
fn enter_keyValue(&mut self, _ctx: &KeyValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#keyValue}.
 * @param ctx the parse tree
 */
fn exit_keyValue(&mut self, _ctx: &KeyValueContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#presetKey}.
 * @param ctx the parse tree
 */
fn enter_presetKey(&mut self, _ctx: &PresetKeyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#presetKey}.
 * @param ctx the parse tree
 */
fn exit_presetKey(&mut self, _ctx: &PresetKeyContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#presetValue}.
 * @param ctx the parse tree
 */
fn enter_presetValue(&mut self, _ctx: &PresetValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#presetValue}.
 * @param ctx the parse tree
 */
fn exit_presetValue(&mut self, _ctx: &PresetValueContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#presetArray}.
 * @param ctx the parse tree
 */
fn enter_presetArray(&mut self, _ctx: &PresetArrayContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#presetArray}.
 * @param ctx the parse tree
 */
fn exit_presetArray(&mut self, _ctx: &PresetArrayContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#presetCall}.
 * @param ctx the parse tree
 */
fn enter_presetCall(&mut self, _ctx: &PresetCallContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#presetCall}.
 * @param ctx the parse tree
 */
fn exit_presetCall(&mut self, _ctx: &PresetCallContext<'input>) { }

/**
 * Enter a parse tree produced by {@link DesignParser#libraryName}.
 * @param ctx the parse tree
 */
fn enter_libraryName(&mut self, _ctx: &LibraryNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link DesignParser#libraryName}.
 * @param ctx the parse tree
 */
fn exit_libraryName(&mut self, _ctx: &LibraryNameContext<'input>) { }

}
