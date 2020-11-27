use serde::{Serialize};
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

    #[serde(rename = "check")]
    Bool(bool),
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

#[derive(Clone)]
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
    type Config: Clone + Default;

    fn default_settings<T: Into<Option<Self::Config>>>(config: T) -> Self {
        Self::default_settings_with(&config.into().unwrap_or_default())
    }

    fn default_settings_with(config: &Self::Config) -> Self;

    fn to_settings<T: Into<Option<Self::Config>>>(&self, config: T) -> Settings {
        self.to_settings_with(
            &config.into().unwrap_or_default()
        )
    }
    fn to_settings_with(&self, config: &Self::Config) -> Settings;

    fn new_settings() -> Settings {
        Self::default_settings(None).to_settings(None)
    }
}

impl<T: SettingsTrait + Clone> FieldTrait for T {
    type Config = <T as SettingsTrait>::Config;

    fn default_self(config: &Self::Config) -> Self {
        T::default_settings_with(&config)
    }

    fn to_field(&self, config: &Self::Config) -> FieldType {
        FieldType::Settings(self.to_settings_with(config))
    }
}

pub trait FieldTrait: Sized {
    type Config: Default + Clone;

    fn default_self(config: &Self::Config) -> Self;
    fn to_field(&self, config: &Self::Config) -> FieldType;

    fn d_self() -> Self {
        Self::default_self(&Self::Config::default())
    }

    fn d_field(&self) -> FieldType {
        self.to_field(&Self::Config::default())
    }
}

impl FieldTrait for f32 {
    type Config = FieldConfig<f32>;

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

#[derive(Default, Clone)]
pub struct DefaultConfig<T> {
    pub value: Option<T>,
}

impl FieldTrait for bool {
    type Config = DefaultConfig<bool>;

    fn default_self(settings: &DefaultConfig<Self>) -> Self {
        settings.value.clone().unwrap_or_default()
    }
    fn to_field(&self, _: &DefaultConfig<Self>) -> FieldType {
        FieldType::Bool(*self)
    }
}

impl FieldTrait for String {
    type Config = DefaultConfig<String>;

    fn default_self(settings: &DefaultConfig<Self>) -> Self {
        settings.value.clone().unwrap_or_default()
    }
    fn to_field(&self, _: &DefaultConfig<Self>) -> FieldType {
        FieldType::Text(self.clone())
    }
}

impl<T: FieldTrait + Clone> FieldTrait for Vec<T> {
    type Config = <T as FieldTrait>::Config;

    // Maybe instantiate some entries?
    fn default_self(_config: &Self::Config) -> Self {
        Vec::new()
    }

    fn to_field(
        &self,
        config: &Self::Config,
    ) -> FieldType {
        FieldType::Array(
            self.iter()
                .map(|x| x.to_field(
                &config)
            ).collect(),
        )
    }
}
