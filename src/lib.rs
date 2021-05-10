#![feature(try_blocks)]

pub use grammar::*;
pub use unflow_parser::*;

pub mod grammar;

pub mod unflow_parser;

#[cfg(test)]
mod tests {
    use crate::unflow_parser;

    #[test]
    fn should_parse_config() {
        let data = r#"project: DesignDSL
feature: "design basic dsl"
type: web
width: 1080px

"#;

        let result = unflow_parser::parse(data);
        assert_eq!(4, result.config.len())
    }

    fn get_examples_flows() -> String {
        let data = r#"flow 登出 {
    SEE 个人信息页
    DO [Click] "登出".按钮
        REACT Success: SHOW "登出成功".Toast with ANIMATE(bounce)
        REACT Success: GOTO HomePage

    SEE "登录失败".窗口
    DO [Click] "登出".按钮
      REACT Success: SHOW "登出成功".Toast with ANIMATE(bounce)

}
"#;
        data.to_string()
    }

    #[test]
    fn should_parse_flow_see() {
        let result = unflow_parser::parse(get_examples_flows().as_str());

        assert_eq!(1, result.flows.len());
        assert_eq!("登出", &result.flows[0].name);
        assert_eq!(2, result.flows[0].interactions.len());

        let interaction = &result.flows[0].clone().interactions[0];
        assert_eq!("个人信息页", interaction.ui_see.component_name);

        let interaction = &result.flows[0].clone().interactions[1];
        assert_eq!("窗口", interaction.ui_see.component_name);
        assert_eq!("登录失败", interaction.ui_see.data);
    }

    #[test]
    fn should_parse_flow_do() {
        let result = unflow_parser::parse(get_examples_flows().as_str());

        let interaction = &result.flows[0].clone().interactions[0];
        assert_eq!("登出", interaction.ui_do.data);
        assert_eq!("按钮", interaction.ui_do.component_name);

        let interaction = &result.flows[0].clone().interactions[1];
        assert_eq!("登出", interaction.ui_do.data);
        assert_eq!("按钮", interaction.ui_do.component_name);
    }

    #[test]
    fn should_parse_flow_react() {
        let result = unflow_parser::parse(get_examples_flows().as_str());

        let react_inter = &result.flows[0].clone().interactions[0].ui_react[0];
        assert_eq!("Success", react_inter.scene_name);
        assert_eq!("bounce", react_inter.animate_name);
        assert_eq!("SHOW", react_inter.react_action);
        assert_eq!("Toast", react_inter.react_component_name);
        assert_eq!("登出成功", react_inter.react_component_data);

        let react_inter2 = &result.flows[0].clone().interactions[0].ui_react[1];
        assert_eq!("GOTO", react_inter2.react_action);
        assert_eq!("HomePage", react_inter2.react_component_name);
    }

    #[test]
    fn should_parse_component_data() {
        let data = r#"component Dialog {
    width: 320px
    height: 240px
}

component BlogList {
    BlogDetail, Space8
}
"#;
        let result = unflow_parser::parse(data);

        assert_eq!(2, result.components.len());
        assert_eq!("Dialog", result.components[0].name);
        assert_eq!(2, result.components[0].configs.len());
        assert_eq!(2, result.components[1].child_components.len());
        assert_eq!("BlogDetail", result.components[1].child_components[0]);
        assert_eq!("Space8", result.components[1].child_components[1]);
    }

    #[test]
    fn should_parse_libraries() {
        let data = r#"library FontSize {
    H1 = 32px
    H2 = 28px
    H3 = 24px
    H4 = 20px
    H5 = 18px
    H6 = 16px
}

"#;
        let result = unflow_parser::parse(data);

        assert_eq!(1, result.libraries.len());
        assert_eq!("FontSize", result.libraries[0].name);
    }
}