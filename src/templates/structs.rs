use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct ITemplate {
    pub templates: Vec<Template>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Template {
    pub id: String,
    pub info: Info,
    pub requests: Vec<Request>,  // Alterado para Vec<Request>
    pub settings: Option<Settings>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Info {
    pub name: String,
    pub reference: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Settings {
    pub concatenate: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Request {
    pub domain: String,
    pub method: String,
    pub path: String,  // Alterado de url para path para corresponder ao YAML fornecido
    pub data: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub matchers: Option<Vec<Matcher>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Matcher {
    #[serde(rename = "type")]
    pub matcher_type: String,
    pub filter: Option<String>,
    pub words: Option<Vec<String>>,
    pub status: Option<Vec<u16>>,
    pub indice: Option<usize>,
    pub value: Option<String>,
}
