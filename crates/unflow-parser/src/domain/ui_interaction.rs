use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UiInteraction {
    pub ui_do: DoInteraction,
    pub ui_see: SeeInteraction,
    pub ui_react: Vec<ReactInteraction>,
}

impl Default for UiInteraction {
    fn default() -> Self {
        UiInteraction {
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
    pub action_name: String,
}

impl Default for DoInteraction {
    fn default() -> Self {
        DoInteraction {
            component_name: "".to_string(),
            data: "".to_string(),
            action_name: "".to_string(),
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

impl Default for ReactInteraction {
    fn default() -> Self {
        Self {
            scene_name: "".to_string(),
            react_action: "".to_string(),
            react_component_name: "".to_string(),
            animate_name: "".to_string(),
            react_component_data: "".to_string()
        }
    }
}
