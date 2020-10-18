use pw_derive::Settings;
use serde::{Deserialize, Serialize};

#[derive(Settings, Debug, Serialize, Deserialize, Clone)]
pub struct InnerSettings {
    pub x: f32,
    pub y: f32,
}

#[derive(Settings, Serialize, Deserialize, Debug)]
pub struct TheseSettings {
    #[settings(name = "Slidy", value = 0.4, inc = 0.01)]
    pub inner_diameter: f32,
    #[settings(name = "Count", value = 12.0, max = 128.0, inc = 1.0)]
    pub count: f32,
    // #[settings(value=[0.4, 0.1, 0.7])]
    pub vector: Vec<f32>,

    #[settings(value = "Hello world")]
    pub some_text: String,

    #[settings(data = true, value = "Private very private")]
    pub private_text: String,

    pub location: InnerSettings,
}

impl TheseSettings {
    pub fn new() -> Self {
        TheseSettings {
            inner_diameter: 0.4,
            count: 12.0,
            vector: vec![0.2, 0.5, 0.1],
            some_text: "Hellow world".into(),
            private_text: "Hellow world".into(),
            location: InnerSettings { x: 0.8, y: 0.4 },
        }
    }
}
