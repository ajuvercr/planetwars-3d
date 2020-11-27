use pw_derive::Settings;
use serde::{Deserialize, Serialize};

use crate::engine::Entity;

// #[derive(Debug, Serialize, Deserialize, Settings, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Planet {
    pub name: String,
    pub location: Entity,
    pub disabled: bool,
}

// impl Planet {
//     pub fn new<S: Into<String>>(name: S, location: Entity) -> Self {
//         Self {
//             name: name.into(),
//             location,
//             disabled: false,
//         }
//     }
// }
