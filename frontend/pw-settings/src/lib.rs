use serde::{Deserialize, Serialize};
use serde_json::Value;

static FLOAT_DEFAULT: f32 = 0.0;
static FLOAT_MIN: f32 = 0.0;
static FLOAT_MAX: f32 = 1.0;
static FLOAT_INC: f32 = 0.1;

#[derive(Serialize, Debug)]
#[serde(tag = "type", content = "content")]
pub enum FieldType {
    #[serde(rename = "text")]
    Text(String),

    #[serde(rename = "slider")]
    Slider {
        value: f32,
        min: f32,
        max: f32,
        inc: f32,
    },

    #[serde(rename = "data")]
    Data(Value),

    #[serde(rename = "settings")]
    Settings(Settings),

    #[serde(rename = "array")]
    Array(Vec<FieldType>),
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

    pub fn add_field<S1: Into<String>, S2: Into<String>>(
        &mut self,
        id: S1,
        name: S2,
        field: FieldType,
    ) {
        self.fields.push(Field {
            id: id.into(),
            name: name.into(),
            field_type: field,
        });
    }

    pub fn add_data<S1: Into<String>, V: Serialize>(&mut self, id: S1, value: V) {
        let id = id.into();
        self.fields.push(Field {
            id: id.clone(),
            name: id,
            field_type: FieldType::Data(
                serde_json::to_value(value).expect("Expected serializable json"),
            ),
        });
    }
}

pub struct FieldConfig<T> {
    pub value: Option<T>,
    pub min: Option<T>,
    pub max: Option<T>,
    pub inc: Option<T>,
}

impl<T> Default for FieldConfig<T> {
    fn default() -> Self {
        Self {
            value: None,
            min: None,
            max: None,
            inc: None,
        }
    }
}

pub trait SettingsTrait: Sized {
    fn default_settings() -> Self;
    fn to_settings(&self) -> Settings;

    fn new_settings() -> Settings {
        Self::default_settings().to_settings()
    }
}

impl<T: SettingsTrait + Clone> FieldTrait for T {
    fn default_self(settings: &FieldConfig<Self>) -> Self {
        settings.value.clone().unwrap_or(T::default_settings())
    }
    fn to_field(&self, _config: &FieldConfig<Self>) -> FieldType {
        FieldType::Settings(self.to_settings())
    }
}

pub trait FieldTrait: Sized {
    fn default_self(config: &FieldConfig<Self>) -> Self;
    fn to_field(&self, config: &FieldConfig<Self>) -> FieldType;

    fn d_self() -> Self {
        Self::default_self(&FieldConfig::default())
    }

    fn d_field(&self) -> FieldType {
        self.to_field(&FieldConfig::default())
    }
}

impl FieldTrait for f32 {
    fn default_self(settings: &FieldConfig<Self>) -> Self {
        settings.value.clone().unwrap_or(FLOAT_DEFAULT)
    }
    fn to_field(&self, config: &FieldConfig<Self>) -> FieldType {
        FieldType::Slider {
            value: *self,
            min: config.min.unwrap_or(FLOAT_MIN),
            max: config.max.unwrap_or(FLOAT_MAX),
            inc: config.inc.unwrap_or(FLOAT_INC),
        }
    }
}

impl FieldTrait for String {
    fn default_self(settings: &FieldConfig<Self>) -> Self {
        settings.value.clone().unwrap_or_default()
    }
    fn to_field(&self, _: &FieldConfig<Self>) -> FieldType {
        FieldType::Text(self.clone())
    }
}

impl<T: FieldTrait + Clone> FieldTrait for Vec<T> {
    fn default_self(settings: &FieldConfig<Self>) -> Self {
        settings.value.clone().unwrap_or_default()
    }
    fn to_field(
        &self,
        FieldConfig {
            ref value,
            min,
            max,
            inc,
        }: &FieldConfig<Self>,
    ) -> FieldType {
        let mut configs = Vec::new();
        for i in 0..self.len() {
            let config = FieldConfig {
                value: value.as_ref().and_then(|vs| vs.get(i).cloned()),
                min: min.as_ref().and_then(|vs| vs.get(i).cloned()),
                max: max.as_ref().and_then(|vs| vs.get(i).cloned()),
                inc: inc.as_ref().and_then(|vs| vs.get(i).cloned()),
            };
            configs.push(config);
        }

        FieldType::Array(
            self.iter()
                .zip(configs.iter())
                .map(|(x, config)| x.to_field(config))
                .collect(),
        )
    }
}
