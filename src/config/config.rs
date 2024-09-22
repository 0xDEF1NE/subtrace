use std::fs;
use dirs::home_dir;
use log::debug;
use std::collections::HashMap;
use serde_yaml::Value;
use log::{error, warn};
use base64::{engine::general_purpose, Engine};
use std::io::Write;
use std::path::PathBuf;
use serde_yaml;

pub fn check_apikeys_file() -> Result<(), String> {
    // Define o conteúdo padrão a ser escrito no arquivo, se necessário
    let default_content = r#"
rapid-api-domain-records: ""
virustotal: ""
whoisxmlapi: ""
securitytrails: ""
binaryedge: ""
shodan: ""
fullhunt: ""
bevigil: ""
chaos: ""
c99: ""
censys: ""
leakix: ""
zoomeye: ""

# Dont need api keys
anubis: "NULL"
webarchive: "NULL"
hackertarget: "NULL"
crtsh: "NULL"
urlscan: "NULL"
alienvault: "NULL"
certspotter: "NULL"
"#;

    let path: PathBuf = home_dir()
        .map(|p| p.join(".config/subtrace/apikeys.yaml"))
        .ok_or_else(|| "Could not determine home directory.".to_string())?;
    
    if !path.exists() || !path.is_file() {
        let config_dir = path.parent().ok_or_else(|| "Could not get parent directory.".to_string())?;
        if !config_dir.exists() {
            fs::create_dir_all(config_dir).map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        let mut file = fs::File::create(&path).map_err(|e| format!("Failed to create API keys file: {}", e))?;
        file.write_all(default_content.as_bytes()).map_err(|e| format!("Failed to write to API keys file: {}", e))?;
        
        debug!("API keys file created at: {}", path.display());
    } else {
        debug!("API keys file path: {}", path.display());

        let content = fs::read_to_string(&path).map_err(|_| "Failed to read API keys file.".to_string())?;

        serde_yaml::from_str::<serde_yaml::Value>(&content)
            .map_err(|_| "Failed to parse API keys file.".to_string())?;
    }

    Ok(())
}

pub fn get_apikeys() -> Result<HashMap<String, Value>, String> {
    let path = home_dir()
        .map(|p| p.join(".config/subtrace/apikeys.yaml"))
        .ok_or("Could not determine home directory.")?;

    if !path.exists() || !path.is_file() {
        return Err("API keys file does not exist.".into());
    }

    let keys = fs::read_to_string(&path).map_err(|e| {
        let error_msg = format!("Failed to read API keys file: {}", e);
        error!("{}", error_msg);
        error_msg
    })?;

    let yaml_data: Value = serde_yaml::from_str(&keys).map_err(|e| {
        let error_msg = format!("Failed to parse YAML file: {}", e);
        error!("{}", error_msg);
        error_msg
    })?;

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