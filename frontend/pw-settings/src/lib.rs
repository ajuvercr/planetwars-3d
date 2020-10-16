use serde::{Serialize};
use serde_json::Value;

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum FieldType {
    #[serde(rename = "vector3")]
    Vector3 {
        value: [f32; 3],
        min: f32,
        max: f32,
        inc: f32,
    },
    #[serde(rename = "text")]
    Text { value: String },
    #[serde(rename = "slider")]
    Slider {
        value: f32,
        min: f32,
        max: f32,
        inc: f32,
    },
    #[serde(rename="data")]
    Data { value: Value },

    #[serde(rename="settings")]
    Settings { inner: Settings },
}

#[derive(Serialize, Debug)]
pub struct Field {
    id: String,
    name: String,

    #[serde(flatten)]
    field_type: FieldType,
}

#[derive(Serialize, Debug)]
pub struct Settings {
    fields: Vec<Field>,
}

impl Settings {
    pub fn new() -> Self {
        Settings { fields: Vec::new() }
    }

    pub fn add_vec3<S: Into<String>, S2: Into<String>>(
        &mut self,
        id: S,
        name: S2,
        value: [f32; 3],
        min: f32,
        max: f32,
        inc: f32,
    ) {
        self.fields.push(Field {
            id: id.into(),
            name: name.into(),
            field_type: FieldType::Vector3 {
                value,
                min,
                max,
                inc,
            },
        });
    }

    pub fn add_text<S: Into<String>, S2: Into<String>, S3: Into<String>>(
        &mut self,
        id: S,
        name: S2,
        value: S3,
    ) {
        self.fields.push(Field {
            id: id.into(),
            name: name.into(),
            field_type: FieldType::Text {
                value: value.into(),
            },
        });
    }

    pub fn add_slider<S: Into<String>, S2: Into<String>>(
        &mut self,
        id: S,
        name: S2,
        value: f32,
        min: f32,
        max: f32,
        inc: f32,
    ) {
        self.fields.push(Field {
            id: id.into(),
            name: name.into(),
            field_type: FieldType::Slider {
                value,
                min,
                max,
                inc,
            },
        });
    }

    pub fn add_settings<S: Into<String>, S2: Into<String>, T: SettingsTrait>(
        &mut self,
        id: S,
        name: S2,
    ) {
        self.fields.push(Field {
            id: id.into(),
            name: name.into(),
            field_type: FieldType::Settings {
                inner: T::new_settings(),
            }
        })
    }

    pub fn add_settings_with<S: Into<String>, S2: Into<String>, T: SettingsTrait>(
        &mut self,
        id: S,
        name: S2,
        value: Settings,
    ) {
        self.fields.push(Field {
            id: id.into(),
            name: name.into(),
            field_type: FieldType::Settings {
                inner: value,
            }
        })
    }
}

pub trait SettingsTrait: Sized {
    fn default_settings() -> Self;
    fn to_settings(&self) -> Settings;

    fn new_settings() -> Settings {
        Self::default_settings().to_settings()
    }
}
