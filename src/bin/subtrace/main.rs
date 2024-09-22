mod clap_app;

use log::{error, info};
use env_logger::{Env, Builder};
use std::io::Write;
use chrono::Local;
use colored::*;
use subtrace::config::check_apikeys_file;
use subtrace::templates::TemplateManager;

#[tokio::main] 
async fn main() {
    let interactive_output = atty::is(atty::Stream::Stdout);
    let app = clap_app::build_app(interactive_output);
    let matches = app.clone().get_matches();

    let silent_mode = matches.contains_id("silent");

    let log_level = if silent_mode {
        "off"
    } else {
        match matches.get_one::<String>("debug").map(|s| s.as_str()).unwrap_or("3") {
            "0" => "error",
            "1" => "warn",
            "2" => "info",
            "3" => "debug",
            _ => "error",
        }
    };
    

    Builder::from_env(Env::default().default_filter_or(log_level))
        .format(|buf, record| {
            let timestamp = Local::now().format("%Y-%m-%dT%H:%M:%S%.3fZ");
            let level = match record.level() {
                log::Level::Error => "ERROR".red(),
                log::Level::Warn => "WARN ".yellow(),
                log::Level::Info => "INFO ".green(),
                log::Level::Debug => "DEBUG".blue(),
                log::Level::Trace => "TRACE".purple(),
            };
            let target = record.target().split("::").next().unwrap_or("unknown");
            writeln!(
                buf,
                "[{} {} {}] {}",
                timestamp,
                level,
                target,
                record.args()
            )
        })
        .init();

    match check_apikeys_file() {
        Ok(_) => {
            info!("API keys file is valid.");
        }
        Err(e) => {
            error!("Error loading API keys file: {}", e);
            return;
        }
    }

    if matches.contains_id("listtemplates") {
        let mut manager = TemplateManager::new("NULL".to_string());
        println!("Listing all available templates...");
        if let Some(templates_path) = matches.get_one::<String>("templates").cloned() {
            if let Err(e) = manager.load_templates(Some(templates_path)).await {
                error!("Error loading templates: {}", e);
                return;
            }
            if let Err(e) = manager.list_loaded_templates().await {
                error!("Error executing requests: {}", e);
            }
        } else {
            info!("No templates path provided.");
        }
        return;
    }

    if let Some(domain) = matches.get_one::<String>("domain").cloned() {
        let mut manager = TemplateManager::new(domain);
        
        if let Some(templates_path) = matches.get_one::<String>("templates").cloned() {
            if let Err(e) = manager.load_templates(Some(templates_path)).await {
                error!("Error loading templates: {}", e);
                return;
            }
            if let Some(threads) = matches.get_one::<i32>("concurrency").cloned() {
                manager.set_threads(threads);
            }
            if let Err(e) = manager.execute_loaded_templates().await {
                error!("Error executing requests: {}", e);
            }

        } else {
            info!("No templates path provided.");
        }
    } else {
        error!("Domain not provided.");
    }
}
