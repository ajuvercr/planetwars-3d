use pw_derive::Settings;
use serde::{Deserialize, Serialize};

use crate::engine::{Entity, Vec3};

const MIN_SIZE: f32 = 10.0;
const INC_SIZE: f32 = 10.0;
const MAX_SIZE: f32 = 10.0;

#[derive(Debug, Serialize, Deserialize, Settings, Clone)]
pub struct Planet {
    pub name: String,
    #[settings(
        position = [x = [inc = INC_SIZE, min = MIN_SIZE, max = MAX_SIZE, ty=[f32]], y = [inc = INC_SIZE,min = MIN_SIZE, max = MAX_SIZE, ty=[f32]], z = [inc = INC_SIZE, min = MIN_SIZE, max = MAX_SIZE, ty=[f32]], ty = [Vec3]],
        speed = [x = [inc = INC_SIZE, min = MIN_SIZE, max = MAX_SIZE, ty=[f32]], y = [inc = INC_SIZE,min = MIN_SIZE, max = MAX_SIZE, ty=[f32]], z = [inc = INC_SIZE, min = MIN_SIZE, max = MAX_SIZE, ty=[f32]], ty = [Vec3]],
        rotation = [x = [inc = INC_SIZE, min = MIN_SIZE, max = MAX_SIZE, ty=[f32]], y = [inc = INC_SIZE,min = MIN_SIZE, max = MAX_SIZE, ty=[f32]], z = [inc = INC_SIZE, min = MIN_SIZE, max = MAX_SIZE, ty=[f32]], ty = [Vec3]],
        ang_speed = [x = [inc = INC_SIZE, min = MIN_SIZE, max = MAX_SIZE, ty=[f32]], y = [inc = INC_SIZE, min = MIN_SIZE, max = MAX_SIZE, ty=[f32]], z = [inc = INC_SIZE, min = MIN_SIZE, max = MAX_SIZE, ty=[f32]], ty = [Vec3]],
        scale = [x = [inc = INC_SIZE, min = MIN_SIZE, max = MAX_SIZE, ty=[f32]], y = [inc = INC_SIZE,min = MIN_SIZE, max = MAX_SIZE, ty=[f32]], z = [inc = INC_SIZE, min = MIN_SIZE, max = MAX_SIZE, ty=[f32]], ty = [Vec3]]
    )]
    pub location: Entity,
    pub disabled: bool,
}

impl Planet {
    pub fn new<S: Into<String>>(name: S, location: Entity) -> Self {
        Self {
            name: name.into(),
            location,
            disabled: false,
        }
    }
}
