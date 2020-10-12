use pw_derive::Settings;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Settings)]
pub struct TheseSettings {
    #[settings(name = "Slidy", value = 0.4)]
    pub inner_diameter: f32,
    #[settings(name = "Count", value = 12.0, max = 128.0, inc = 1.0)]
    pub count: f32,
    #[settings(value=[0.4, 0.1, 0.7])]
    pub vector: [f32; 3],
}
