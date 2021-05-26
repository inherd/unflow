use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::domain::ui_interaction::UiInteraction;
use crate::domain::ui_layout::UiLayout;

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

impl Default for UiLibraryPreset {
    fn default() -> Self {
        UiLibraryPreset {
            key: "".to_string(),
            value: "".to_string(),
            preset_calls: vec![],
            sub_properties: vec![]
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PresetCall {
    pub name: String,
    pub preset: String
}

impl Default for PresetCall {
    fn default() -> Self {
        PresetCall {
            name: "".to_string(),
            preset: "".to_string()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UiProperty {
    pub key: String,
    pub value: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UiFlow {
    pub name: String,
    pub interactions: Vec<UiInteraction>,
    pub layout: Vec<UiLayout>
}

impl UiFlow {
    pub fn new(name: String) -> Self {
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
