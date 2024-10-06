use super::structs::Template; // Certifique-se de que Matcher está sendo importado
use crate::config::{get_apikeys, return_api};
use dirs::home_dir;
use std::path::PathBuf;
use std::error::Error;
use tokio::fs; // Usar a versão assíncrona do fs
use log::{info,error};
use crate::config::encode_base64;
use crate::requests::requests::MakeRequest;
use std::fs::OpenOptions;
use std::io::Write;
use std::collections::{HashMap, HashSet};
pub struct TemplateManager {
    templates: Vec<Template>,
    t_temp: Vec<PathBuf>,
    domain: String,
    threads: i32,
}

impl TemplateManager {
    pub fn new(domain: String) -> Self {
        Self {
            templates: Vec::new(),
            t_temp: Vec::new(),
            domain,
            threads: 0,
        }
    }

    pub async fn load_templates(&mut self, templates_path: Option<String>) -> Result<(), Box<dyn Error>> {
        let path: PathBuf = match templates_path {
            Some(path) => {
                info!("Loading templates from: {}", path);
                PathBuf::from(path)
            },
            None => {
                let path = home_dir()
                    .map(|p| p.join(".config/subtrace/templates/"))
                    .ok_or("Could not determine home directory.")?;
                
                info!("No templates path provided. Loading default templates from {}", path.display());
                path
            }
        };
        
        self.read_templates(path).await?;
        
        let templates_to_process: Vec<PathBuf> = self.t_temp.clone();
    
        for entry in &templates_to_process {
            self.process_template(entry).await?;
        }
    
        Ok(())
    }
    pub async fn list_loaded_templates(&self) -> Result<(), Box<dyn Error>> {
        let templates_to_process: Vec<Template> = self.templates.clone();
        for template in templates_to_process {
            println!("{}",template.id);
        }
        Ok(())
    }

    pub async fn execute_loaded_templates(&self, output_file: String) -> Result<(), Box<dyn Error>> {
        let templates_to_process: Vec<Template> = self.templates.clone();
        let mut seen_subdomains = HashSet::new();
        let mut init = MakeRequest::new();
        match init.execute_requests(templates_to_process, self.domain.clone(), self.threads).await {
            Ok(subdomains) => {
                let len_sub = subdomains.len();
                info!("Total Subdomains Found: {}", len_sub);
                
                let mut subdomain_count: HashMap<String, usize> = HashMap::new();
                
                let mut file = if !output_file.is_empty() {
                    Some(OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(output_file)?)
                } else {
                    None
                };
    
                for (finder, subdomain) in subdomains {
                    // Se o subdomínio já foi visto, continue para a próxima iteração
                    if seen_subdomains.contains(&subdomain) {
                        continue;
                    }
                
                    // Adiciona o subdomínio ao HashSet
                    seen_subdomains.insert(subdomain.clone());
                
                    *subdomain_count.entry(finder.clone()).or_insert(0) += 1;
                
                    println!("{}", subdomain);
                
                    if let Some(f) = file.as_mut() {
                        writeln!(f, "{}", subdomain)?;
                    }
                }
    
                for (finder, count) in subdomain_count {
                    info!("{} found {} subdomains", finder, count);
                }
            }
            Err(e) => {
                error!("Failed to execute requests: {:?}", e);
            }
        }
        Ok(())
    }
    
    async fn read_templates(&mut self, templates_path: PathBuf) -> Result<(), Box<dyn Error>> {
        let mut entries = vec![];

        let mut read_dir = tokio::fs::read_dir(templates_path).await?;
        while let Some(entry) = read_dir.next_entry().await? {
            if let Some(ext) = entry.path().extension() {
                if ext == "yaml" || ext == "yml" {
                    entries.push(entry.path());
                }
            }
        }

        self.t_temp = entries;
        Ok(())
    }

    async fn process_template(&mut self, path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string(path).await?;
        let mut data: serde_yaml::Value = serde_yaml::from_str(&content)?;
        let api_keys = get_apikeys()?;
        let id_value = data["id"].as_str().unwrap_or_default();
        let token = return_api(&api_keys, id_value);
        
        self.substitute_variables_recursive(&mut data, self.domain.as_str(), &token);

        if let serde_yaml::Value::Mapping(ref mut map) = data {
            if let Some(serde_yaml::Value::Sequence(requests)) = map.get_mut(&serde_yaml::Value::String("requests".to_string())) {
                for request in requests.iter_mut() {
                    if let serde_yaml::Value::Mapping(request_map) = request {
                        request_map.insert(
                            serde_yaml::Value::String("domain".to_string()),
                            serde_yaml::Value::String(self.domain.clone()),
                        );
                    }
                }
            }
        }

        let template: Template = serde_yaml::from_value(data)?;
        self.templates.push(template);

        Ok(())
    }

    pub fn set_threads(&mut self, concurrency: i32){
        self.threads = concurrency;
    }
    
    fn substitute_variables_recursive(&self, data: &mut serde_yaml::Value, domain: &str, token: &str) {
        match data {
            serde_yaml::Value::String(s) => {
                if s.contains("subtrace_B64({{domain}})") {
                    *s = s.replace("subtrace_B64({{domain}})", &encode_base64(domain));
                }
                *s = s.replace("{{domain}}", domain).replace("{{token}}", token);
            }
            serde_yaml::Value::Mapping(map) => {
                for (_, value) in map.iter_mut() {
                    self.substitute_variables_recursive(value, domain, token);
                }
            }
            serde_yaml::Value::Sequence(vec) => {
                for item in vec.iter_mut() {
                    self.substitute_variables_recursive(item, domain, token);
                }
            }
            _ => {}
        }
    }
}
