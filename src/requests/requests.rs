use crate::templates::structs::{Template, Matcher, Settings};
use std::collections::HashSet;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::error::Error as StdError;
use log::{error, info};
use regex::Regex;
use serde_json::Value;
use tokio::time::{sleep, Duration};

pub struct MakeRequest {
    subdomains: Vec<String>,
}

impl MakeRequest {
    pub fn new() -> Self {
        Self {
            subdomains: Vec::new(),
        }
    }

    pub async fn execute_requests(&mut self, templates_to_process: Vec<Template>, domain: String, threads: i32) -> Result<Vec<String>, Box<dyn StdError>> {
        let client = Client::builder()
            .timeout(Duration::from_secs(15)) // Tempo limite padrão
            .build()
            .expect("Failed to build HTTP client");

        let mut unique_subdomains = HashSet::new();

        for template in templates_to_process {
            let template_name = template.info.name.clone();

            for request in &template.requests {
                let headers = self.build_headers(request.headers.as_ref())?;

                match self.send_request(template_name.clone(), &client, &request.method, &request.path, &headers, request.data.as_ref()).await {
                    Ok(resp) => {
                        let status_code = resp.status().as_u16();
                        let response_text = resp.text().await.unwrap_or_default(); // Retorna string vazia se falhar

                        if let Some(matchers) = &request.matchers {
                            let new_subdomains = self.process_matchers(&response_text, status_code, matchers, template.clone(), template_name.clone());
                            for subdomain in &new_subdomains {
                                if subdomain.contains(domain.as_str()) {
                                    unique_subdomains.insert(subdomain.clone()); // Adicionar subdomínios ao HashSet
                                }
                            }
                        }
                    }
                    Err(_) => {
                        continue; // Continue para a próxima requisição
                    }
                }
            }
        }

        self.subdomains = unique_subdomains.into_iter().collect(); // Converter HashSet de volta para Vec
        Ok(self.subdomains.clone())
    }

    async fn send_request(&self, template_name: String, client: &Client, method: &str, path: &str, headers: &HeaderMap, data: Option<&String>) -> Result<reqwest::Response, Box<dyn StdError>> {
        let mut retries = 3;
        let mut delay = Duration::from_secs(2);
    
        while retries > 0 {
            let request = match method {
                "GET" => client.get(path).headers(headers.clone()),
                "POST" => {
                    let req = client.post(path).headers(headers.clone());
                    if let Some(body_data) = data {
                        req.body(body_data.clone())
                    } else {
                        req
                    }
                }
                _ => return Err(Box::from("Unsupported HTTP method")),
            };
    
            match request.send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        return Ok(response);
                    } else {
                        error!("{} | Request failed with status: {}", template_name, response.status());
                        return Err(Box::from(format!("Request failed with status: {}", response.status())));
                    }
                }
                Err(e) => {
                    if e.is_timeout() {
                        info!("Request timed out for path: {}. Retries left: {}...", path, retries - 1);
                    } else {
                        error!("Request error for path: {}: {:?}", path, e);
                        return Err(Box::from(e));
                    }
                }
            }
    
            // Retry logic
            retries -= 1;
            if retries > 0 {
                sleep(delay).await;
                delay *= 2; // Exponential backoff
            }
        }
    
        Err(Box::from("Request failed after multiple retries"))
    }

    fn process_matchers(&mut self, response_text: &str, status_code: u16, matchers: &[Matcher], template: Template, template_name: String) -> Vec<String> {
        let mut subdomains = Vec::new();
        let mut error_occurred = false;

        if let Some(matcher) = matchers.iter().find(|m| m.matcher_type == "status") {
            if let Some(result) = self.check_status_matcher(response_text, status_code, matcher, &template) {
                error_occurred = true;
                error!("{}| {:?}", template_name, result);
                subdomains.extend(result);
            }
        }

        if !error_occurred {
            if let Some(matcher) = matchers.iter().find(|m| m.matcher_type == "subdomains") {
                if let Some(result) = self.check_subdomains_matcher(response_text, matcher, &template) {
                    subdomains.extend(result);
                }
            }
        }

        subdomains.iter().map(|s| s.replace("*.", "")).collect()
    }

    fn check_status_matcher(&mut self, response_text: &str, status_code: u16, matcher: &Matcher, template: &Template) -> Option<Vec<String>> {
        if let Some(status_codes) = &matcher.status {
            if status_codes.contains(&status_code) {
                return Some(self.filter_request(response_text, matcher, template.clone()));
            }
        }

        None
    }

    fn check_subdomains_matcher(&mut self, response_text: &str, matcher: &Matcher, template: &Template) -> Option<Vec<String>> {
        Some(self.filter_request(response_text, matcher, template.clone()))
    }

    fn build_headers(&self, req_headers: Option<&std::collections::HashMap<String, String>>) -> Result<HeaderMap, Box<dyn StdError>> {
        let mut headers = HeaderMap::new();
        if let Some(headers_map) = req_headers {
            for (key, value) in headers_map {
                let header_name = HeaderName::from_bytes(key.as_bytes())?;
                let header_value = value.parse::<HeaderValue>()?;
                headers.insert(header_name, header_value);
            }
        }
        Ok(headers)
    }

    fn filter_request(&mut self, response_text: &str, matcher: &Matcher, template: Template) -> Vec<String> {
        if let Some(filter_type) = matcher.filter.as_deref() {
            match filter_type {
                "regex" => self.handle_regex_filter(response_text, matcher),
                "json" => self.handle_json_filter(response_text, matcher, template),
                _ => vec![format!("{} | Unsupported filter type", template.info.name)],
            }
        } else {
            vec![format!("{} | No filter provided", template.info.name)]
        }
    }

    fn handle_regex_filter(&mut self, response: &str, matcher: &Matcher) -> Vec<String> {
        if let Some(regex_value) = &matcher.value {
            let subdomains = Self::parse_response_wregex(response, regex_value);
            for subdomain in &subdomains {
                if !self.subdomains.contains(subdomain) {
                    self.subdomains.push(subdomain.to_string());
                }
            }
            return subdomains;
        }
        vec![]
    }

    fn handle_json_filter(&mut self, response: &str, matcher: &Matcher, template: Template) -> Vec<String> {
        if !Self::is_json_valid(response) {
            return vec![format!("{} | Invalid JSON response", template.info.name)];
        }

        let json_ret: Value = serde_json::from_str(response).expect(&format!("JSON was not well-formatted for template: {}", template.info.name));
        let json_values = Self::parse_json_words(json_ret, &template.settings, matcher.words.clone());

        json_values.as_array()
            .map(|array| {
                array.iter().filter_map(|item| {
                    item.as_str().map(|domain| {
                        let domain_string = domain.to_string();
                        if !self.subdomains.contains(&domain_string) {
                            self.subdomains.push(domain_string.clone());
                        }
                        domain_string
                    })
                }).collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }

    fn parse_response_wregex(response: &str, regex_value: &str) -> Vec<String> {
        let re = match Regex::new(regex_value) {
            Ok(r) => r,
            Err(_) => return Vec::new(),
        };

        re.captures_iter(response)
            .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
            .collect()
    }

    fn is_json_valid(json_str: &str) -> bool {
        serde_json::from_str::<Value>(json_str).is_ok()
    }

    fn parse_json_words(json_ret: Value, settings: &Option<Settings>, words: Option<Vec<String>>) -> Value {
        let mut value = json_ret;
        if let Some(words) = words {
            for key in words {
                value = match value {
                    Value::Array(mut arr) => Value::Array(Self::process_array(&mut arr, settings, &key)),
                    Value::Object(mut obj) => obj.remove(&key).unwrap_or(Value::Object(Default::default())),
                    _ => Value::Object(Default::default()),
                };
            }
        }
        value
    }

    fn process_array(arr: &mut Vec<Value>, settings: &Option<Settings>, key: &str) -> Vec<Value> {
        arr.drain(..)
            .map(|item| Self::parse_json_words(item, settings, Some(vec![key.to_string()])))
            .flat_map(|item_value| match item_value {
                Value::Array(item_arr) => {
                    item_arr.into_iter().flat_map(|s| {
                        s.as_str()
                            .filter(|str_val| str_val.contains('\n'))
                            .map(|str_val| Value::String(Self::remove_newline(str_val)))
                            .into_iter()
                            .chain(std::iter::once(s))
                    }).collect::<Vec<_>>()
                }
                Value::String(str_val) => {
                    vec![Value::String(
                        if str_val.contains('\n') {
                            Self::remove_newline(&str_val)
                        } else {
                            str_val
                        }
                    )]
                }
                _ => vec![item_value],
            })
            .collect()
    }

    fn remove_newline(s: &str) -> String {
        s.lines().next().unwrap_or("").to_string()
    }
}
