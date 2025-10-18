mod lib;

use std::io;
use std::io::Write;
use std::process::exit;
use std::sync::Arc;
use clap::Parser;
use serde_json::Value;
use tokio::sync::Semaphore;
use lib::requester::target_types;
use lib::config;
use crate::lib::config::RequestConfig;
use crate::lib::config::RequestMethod;
use crate::lib::requester::target_types::generic;

#[derive(Parser)]
#[command(version = "0.0.1", about = "Tired of skiddies using webhooks on cool platforms for their shitty stealer malware? same. you need to set an env variable of SPAMMY_PROXY in order to use the proxy.", long_about = None)]
struct Arguments {
    #[arg(short, long)]
    verbose: Option<bool>,
    #[arg(short, long)]
    discord_target: Option<bool>,
    #[arg(short, long)]
    telegram_target: Option<bool>,
    #[arg(short, long)]
    grabify_target: Option<bool>,
    #[arg(short, long)]
    undefined_target: Option<bool>,
    #[arg(short, long, value_name = "webhook", help = "target webhook url to spam blast. ~~[THIS IS REQUIRED]~~ \nIf you are using generic, it has to be the name of the file you are wanting to load.\nThis file should reside in the folder: $HOME/.local/share/spam_blaster/", required = true)]
    webhook_url: Option<String>,
    #[arg(short, long, help = "thread limit to not overload your computer with, default is server power, at 100 (currently unused as the default for all is 100)")]
    limit_threads: Option<usize>,
    #[arg(short, long, help = "use a proxy to send requests, this is useful for stealth")]
    proxy: Option<String>,
}

fn prompt(message: &str) -> Result<String, io::Error> {
    print!("{}\n", message);
    print!("-> ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();
    let mut r_cfg = RequestConfig {
        url: args.webhook_url.unwrap(),
        method: RequestMethod::GET,
        body: None,
        jitter: None,
        known_bad_response_codes: config::BAD_RESPONSE_CODES.to_vec(),
        known_bad_response_strings: config::BAD_RESPONSE_MESSAGES.to_vec(),
        proxy: None,
    };
    if let Some(p) = args.proxy {
        r_cfg.proxy = Some(p);
    }

    // Configure target-specific settings
    if let Some(_target_discord) = args.discord_target {
        let mut c = target_types::discord::WebHookBlaster::new();
        if let Ok(p) = prompt("Would you like to generate random junk data? [y/n]") {
            if p == "y" || p == "Y" || p == "yes" || p == "Yes" {
                c.set_random_junk(true);
            }
        }
        if let Ok(p) = prompt("Would you like to add jitter to your requests? [y/n]") {
            if p == "y" || p == "Y" || p == "yes" || p == "Yes" {
                c.set_mention_everyone(true);
            }
        }
        if let Ok(p) = prompt("Would you like to mention everyone? [y/n]") {
            if p == "y" || p == "Y" || p == "yes" || p == "Yes" {
                c.set_mention_everyone(true);
            }
        }
        if let Ok(p) = prompt("Would you like to add a delay between requests so we dont get blocked? [y/n] (no indicates that you want to spam as fast as possible)") {
            if p == "y" || p == "Y" || p == "yes" || p == "Yes" {
                r_cfg.jitter = Some(c.get_jitter_base()).unwrap_or_default();
            }
        }
        r_cfg.method = RequestMethod::POST;
        r_cfg.body = Some(serde_json::from_str(&c.build_body(None)).unwrap_or_default());
    }

    if let Some(_target_telegram) = args.telegram_target {
        if let Ok(p) = prompt("Would you like to add a delay between requests so we dont get blocked? [y/n] (no indicates that you want to spam as fast as possible)") {
            if p == "y" || p == "Y" || p == "yes" || p == "Yes" {
                r_cfg.jitter = Some(vec![1.0, 1.456]);
            }
        }
        r_cfg.body = Some(serde_json::from_str(&target_types::telegram::Telegram::new().build_body(&r_cfg.url.clone())).unwrap_or_default());
    }

    if let Some(_target_grabify) = args.grabify_target {
        println!("Grabify target, this is kind of odd. Grabify blocks tor outright, so you wont be able to hide while spamming this unless you have stealth proxies.");
        exit(0);
    }

    if let Some(_target_generic) = args.undefined_target {
        let home = std::env::var("HOME").unwrap();
        let target_json = r_cfg.url.clone();
        let d_directory = std::path::Path::new(&home)
            .join(config::DEFAULT_DATA_DIRECTORY)
            .join(target_json)
            .to_string_lossy()
            .to_string();
        println!("Please ensure that the program is pointed to a json file, this should include parameters for the request. Please refer to the repo for an example.");
        println!("Looking for json file in {d_directory}");
        if !std::path::Path::new(&d_directory).exists() {
            std::fs::create_dir_all(&d_directory).unwrap();
            println!("Created directory {d_directory}");
            println!("Please create a json file in {d_directory}");
            exit(0);
        }
        let j = generic::load_json_file(std::path::PathBuf::from(d_directory)).unwrap();
        r_cfg.url = j.target_url.as_str().unwrap_or("").to_string();
        r_cfg.set_body(Value::from(j.target_body));
        r_cfg.method = match j.target_method {
            generic::GenericMethod::GET => config::RequestMethod::GET,
            generic::GenericMethod::POST => config::RequestMethod::POST,
            generic::GenericMethod::PUT => config::RequestMethod::PUT,
            generic::GenericMethod::DELETE => config::RequestMethod::DELETE,
            generic::GenericMethod::PATCH => config::RequestMethod::PATCH,
            generic::GenericMethod::HEAD => config::RequestMethod::HEAD,
            generic::GenericMethod::OPTIONS => config::RequestMethod::OPTIONS,
            _ => config::RequestMethod::GET,
        };
        #[cfg(debug_assertions)]{
            println!("{:?}", r_cfg.clone());
        }
    }

    // Perform one-time connectivity check
    println!("Performing connectivity check...");
    match lib::requester::request_executor::check_connectivity(&r_cfg).await {
        Ok(true) => {
            println!("✓ Connectivity check passed. Starting request loop...");
        }
        Ok(false) => {
            eprintln!("✗ Connectivity check failed. Target is not reachable.");
            exit(1);
        }
        Err(e) => {
            eprintln!("✗ Connectivity check error: {}", e);
            exit(1);
        }
    }

    // Get thread limit from args or use default
    let thread_limit = args.limit_threads.unwrap_or(100);
    println!("Thread limit: {}", thread_limit);

    let r_cfg_clone = Arc::new(r_cfg.clone());
    let semaphore = Arc::new(Semaphore::new(thread_limit));
    let mut handles = Vec::new();

    // Continuous request loop
    loop {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let r_cfg_clone = r_cfg_clone.as_ref().clone();
        let handle = tokio::spawn(async move {
            let resp = lib::requester::request_executor::send_request(r_cfg_clone).await;
            match resp {
                Ok(_response) => {
                    println!("✓ Request successful");
                },
                Err(e) => {
                    eprintln!("✗ Request failed: {}", e);
                }
            }
            drop(permit);
        });
        handles.push(handle);
    }
}