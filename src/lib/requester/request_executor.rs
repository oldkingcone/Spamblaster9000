use std::error::Error;
use std::time::Duration;
use reqwest;
use reqwest::{Client, Proxy, Response};
use crate::config::RequestConfig;

pub fn build_client(proxy: String) -> Result<reqwest::Client, reqwest::Error> {
    if std::env::var("SPAMMY_PROXY").is_ok() {
        let proxy = std::env::var("SPAMMY_PROXY").unwrap();
        Client::builder()
            .proxy(Proxy::all(&proxy)?)
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.127 Safari/537.36")
            .connect_timeout(Duration::from_secs(5))
            .danger_accept_invalid_certs(true)
            .danger_accept_invalid_hostnames(true)
            .build()
    } else {
        let mut p = String::new();
        if proxy != "" {
            println!("Using proxy: {}", proxy);
            p = proxy.to_string();
        } else {
            println!("No proxy set, using default");
            p = "socks5h://127.0.0.1:1025".to_string();
        }
        Client::builder()
            .proxy(Proxy::all(p)?)
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.127 Safari/537.36")
            .connect_timeout(Duration::from_secs(5))
            .danger_accept_invalid_certs(true)
            .danger_accept_invalid_hostnames(true)
            .build()
    }
}

/// Performs a one-time connectivity check to the target
pub async fn check_connectivity(config: &RequestConfig) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    #[cfg(debug_assertions)]
    {
        println!("[DEBUG] Connectivity check:");
        println!("[DEBUG]   URL: {}", config.url);
        println!("[DEBUG]   Method: {:?}", config.method);
    }

    let client = build_client(config.proxy.clone().unwrap_or_default())?;
    let mut req_builder = client.request(config.method.clone().into(), &config.url);

    if let Some(body) = &config.body {
        req_builder = req_builder.json(body);
    }

    match req_builder.send().await {
        Ok(response) => {
            let status = response.status();
            #[cfg(debug_assertions)]
            {
                println!("[DEBUG] Connectivity check status: {}", status);
            }

            if status.to_string() != "" {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Err(e) => {
            #[cfg(debug_assertions)]
            {
                println!("[DEBUG] Connectivity check error: {}", e);
            }
            Ok(false)
        }
    }
}

/// Sends a single request without connectivity check
pub async fn send_request(config: RequestConfig) -> Result<Response, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let client = build_client(config.proxy.clone().unwrap_or_default())?;

    let mut req_builder = client.request(config.method.into(), &config.url);

    if let Some(body) = &config.body {
        req_builder = req_builder.json(body);
    }

    let response = req_builder.send().await?;
    let status = response.status();

    #[cfg(debug_assertions)]
    {
        println!("[DEBUG] Response status: {} ({})", status.as_u16(), status.canonical_reason().unwrap_or("Unknown"));
    }

    if status.is_success() {
        Ok(response)
    } else if status.is_client_error() || status.is_server_error() {
        let text = response.text().await.unwrap_or_default();
        if text.contains("Unknown Webhook") {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Webhook was deleted")))
        } else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Request failed with status {}", status))))
        }
    } else {
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Unexpected response")))
    }
}

// Keep old function for backward compatibility if needed
pub async fn make_request(req: RequestConfig) -> Result<Response, Box<dyn std::error::Error + Send + Sync + 'static>> {
    send_request(req).await
}