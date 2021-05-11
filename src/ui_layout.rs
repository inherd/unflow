use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UiLayout {
    pub name: String,
    pub flex_childs: Vec<FlexChild>
}

impl Default for UiLayout {
    fn default() -> Self {
        UiLayout {
            name: "".to_string(),
            flex_childs: vec![]
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlexChild {
    pub flex: Flex,
    pub cells: Vec<FlexCell>
}

impl Default for FlexChild {
    fn default() -> Self {
        FlexChild {
            flex: Flex::ROW,
            cells: vec![]
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FlexCell {
    pub component_name: String,
    pub layout_info: String,
    pub normal_info: String
}

impl Default for FlexCell {
    fn default() -> Self {
        FlexCell {
            component_name: "".to_string(),
            layout_info: "".to_string(),
            normal_info: "".to_string()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Flex {
    ROW,
    COLUMN
}
