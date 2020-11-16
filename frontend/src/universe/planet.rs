use pw_derive::Settings;
use serde::{Deserialize, Serialize};

use crate::engine::Entity;

#[derive(Debug, Serialize, Deserialize, Settings, Clone)]
pub struct Planet {
    name: String,
    location: Entity,
}

impl Planet {
    pub fn new<S: Into<String>>(name: S, location: Entity) -> Self {
        Self {
            name: name.into(),
            location,
        }
    }
}
