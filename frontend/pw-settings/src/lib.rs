use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
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
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Field {
    id: String,
    name: String,

    #[serde(flatten)]
    field_type: FieldType,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Settings {
    fields: Vec<Field>,
}

impl Settings {
    pub fn new() -> Self {
        Settings { fields: Vec::new() }
    }

    pub fn vec3<S: Into<String>, S2: Into<String>>(
        mut self,
        id: S,
        name: S2,
        value: [f32; 3],
        min: f32,
        max: f32,
        inc: f32,
    ) -> Self {
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
        self
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

    pub fn text<S: Into<String>, S2: Into<String>, S3: Into<String>>(
        mut self,
        id: S,
        name: S2,
        value: S3,
    ) -> Self {
        self.fields.push(Field {
            id: id.into(),
            name: name.into(),
            field_type: FieldType::Text {
                value: value.into(),
            },
        });
        self
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

    pub fn slider<S: Into<String>, S2: Into<String>>(
        mut self,
        id: S,
        name: S2,
        value: f32,
        min: f32,
        max: f32,
        inc: f32,
    ) -> Self {
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

        self
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
}

pub trait SettingsTrait: Sized {
    fn into_settings() -> Settings;
}
