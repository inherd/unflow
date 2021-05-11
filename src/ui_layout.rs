use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UiLayout {
    pub name: String,
    pub flex_child: Vec<FlexChild>
}

impl Default for UiLayout {
    fn default() -> Self {
        UiLayout {
            name: "".to_string(),
            flex_child: vec![]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Flex {
    ROW,
    COLUMN
}
