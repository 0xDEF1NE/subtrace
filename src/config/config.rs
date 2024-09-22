use std::fs;
use dirs::home_dir;
use log::debug;
use std::collections::HashMap;
use serde_yaml::Value;
use log::{error, warn};
use base64::{engine::general_purpose, Engine};

pub fn check_apikeys_file() -> Result<(), String> {
    let path = home_dir()
        .map(|p| p.join(".config/subtrace/apikeys.yaml"))
        .ok_or("Could not determine home directory.")?;
    
    if !path.exists() || !path.is_file() {
        return Err("API keys file does not exist.".into());
    }

    debug!("API keys file path: {}", path.display());
    
    let content = fs::read_to_string(&path).map_err(|_| "Failed to read API keys file.")?;
    serde_yaml::from_str::<serde_yaml::Value>(&content).map_err(|_| "Failed to parse API keys file.")?;
    
    Ok(())
}

pub fn get_apikeys() -> Result<HashMap<String, Value>, String> {
    // Determina o caminho do arquivo de chaves de API
    let path = home_dir()
        .map(|p| p.join(".config/subtrace/apikeys.yaml"))
        .ok_or("Could not determine home directory.")?;

    // Verifica se o arquivo existe e é um arquivo regular
    if !path.exists() || !path.is_file() {
        return Err("API keys file does not exist.".into());
    }

    // Lê o conteúdo do arquivo
    let keys = fs::read_to_string(&path).map_err(|e| {
        let error_msg = format!("Failed to read API keys file: {}", e);
        error!("{}", error_msg);
        error_msg
    })?;

    // Parseia o conteúdo YAML
    let yaml_data: Value = serde_yaml::from_str(&keys).map_err(|e| {
        let error_msg = format!("Failed to parse YAML file: {}", e);
        error!("{}", error_msg);
        error_msg
    })?;

    // Converte o YAML para um HashMap
    let hashmap = yaml_data.as_mapping().ok_or("Failed to convert YAML to HashMap")?;
    
    let result = hashmap.iter()
        .filter_map(|(k, v)| k.as_str().map(|key_str| (key_str.to_string(), v.clone())))
        .collect::<HashMap<_, _>>();

    Ok(result)
}

pub fn return_api(api_keys: &HashMap<String, Value>, key: &str) -> String {
    let api_key = api_keys.get(key).and_then(|api| {
        api.as_str().map(|api_str| {
            if api_str.is_empty() {
                warn!("The API value for key '{}' is empty", key);
                return "".to_string();
            }
            return api_str.to_string();
        })
    });
    match api_key {
        Some(key) => key,
        None => {
            return "".to_string();
        }
    }
}    

pub fn encode_base64(input: &str) -> String {
    general_purpose::STANDARD.encode(input)
}