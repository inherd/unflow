#![feature(try_blocks)]

pub use grammar::*;
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

    #[test]
    fn should_parse_flow() {
        let data = r#"flow 登出 {
    SEE 个人信息页
    DO [Click] "登出".按钮
    REACT Success: SHOW "登出成功".Toast with ANIMATE(bounce)

    SEE "登录失败".窗口
    DO [Click] "登出".按钮
      REACT Success: SHOW "登出成功".Toast with ANIMATE(bounce)

}

"#;

        let result = unflow_parser::parse(data);
        println!("{:?}", result);

        assert_eq!(1, result.flows.len());
        assert_eq!("登出", &result.flows[0].name);
        assert_eq!(2, result.flows[0].interactions.len());

        let interaction = &result.flows[0].clone().interactions[0];
        assert_eq!("个人信息页", interaction.ui_see.component_name);

        let interaction = &result.flows[0].clone().interactions[1];
        assert_eq!("窗口", interaction.ui_see.component_name);
        assert_eq!("登录失败", interaction.ui_see.data);
    }
}