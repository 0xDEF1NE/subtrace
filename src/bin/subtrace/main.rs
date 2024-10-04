mod clap_app;

use log::{error, info};
use env_logger::{Env, Builder};
use std::io::Write;
use chrono::Local;
use colored::*;
use subtrace::config::check_apikeys_file;
use subtrace::templates::TemplateManager;
use subtrace::modules::zone_transfer::call_zone_transfer;

#[tokio::main]
async fn main() {
    let interactive_output = atty::is(atty::Stream::Stdout);
    let app = clap_app::build_app(interactive_output);
    let matches = app.clone().get_matches();

    init_logger(&matches);

    match check_apikeys_file() {
        Ok(_) => {
            info!("API keys file is valid.");
        }
        Err(e) => {
            error!("Error loading API keys file: {}. Retrying...", e);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    if matches.get_flag("listtemplates") {
        list_templates(&matches).await;
        return;
    }

    if let Some(domain) = matches.get_one::<String>("domain").cloned() {
        zone_transfer(&domain).await;

        passive_start(domain, &matches).await;
    } else {
        error!("Domain not provided.");
    }
}

fn init_logger(matches: &clap::ArgMatches) {
    let silent_mode = matches.get_flag("silent");

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
}

async fn list_templates(matches: &clap::ArgMatches) {
    let mut manager = TemplateManager::new("NULL".to_string());
    println!("Listing all available templates...");

    if let Some(templates_path) = matches.get_one::<String>("templates").cloned() {
        if let Err(e) = manager.load_templates(Some(templates_path)).await {
            error!("Error loading templates: {}", e);
            return;
        }
        if let Err(e) = manager.list_loaded_templates().await {
            error!("Error listing templates: {}", e);
        }
    } else {
        info!("No templates path provided.");
    }
}

async fn zone_transfer(domain: &str) {
    let domains = call_zone_transfer(domain).await;
    if !domains.is_empty() {
        info!("Zone Transfer: ");
        for domain in &domains {
            println!("{}", domain);
        }
    } else {
        info!("Zone Transfer: No domains found in zone transfer.");
    }
}

async fn passive_start(domain: String, matches: &clap::ArgMatches) {
    let mut manager = TemplateManager::new(domain);

    if let Some(templates_path) = matches.get_one::<String>("templates").cloned() {
        if let Err(e) = manager.load_templates(Some(templates_path)).await {
            error!("Error loading templates: {}", e);
            return;
        }

        if let Some(threads) = matches.get_one::<i32>("concurrency").cloned() {
            manager.set_threads(threads);
        }

        let output_file = matches.get_one::<String>("output").cloned().unwrap_or_default();

        if let Err(e) = manager.execute_loaded_templates(output_file).await {
            error!("Error executing requests: {}", e);
        }
    } else {
        info!("No templates path provided.");
    }
}
