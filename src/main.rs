use clap::Parser;
use reqwest::header::CONTENT_TYPE;
use std::collections::HashMap;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(long)]
    target: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let client = reqwest::Client::builder()
        .user_agent("DiscordWebHook429/0.1.0")
        .build()
        .unwrap();

    loop {
        let resp = client
            .post(&cli.target)
            .header(CONTENT_TYPE, "application/json")
            .json(&{
                let mut req = HashMap::new();
                req.insert("content".to_string(), "Hello".to_string());
                req
            })
            .send()
            .await
            .unwrap();

        for (key, value) in resp.headers() {
            if key
                .as_str()
                .to_ascii_lowercase()
                .starts_with("x-ratelimit")
            {
                println!("{key:width$}: {}", value.to_str().unwrap(), width = 32);
            }
        }

        if resp.status().is_client_error() {
            println!("{}", resp.text().await.unwrap());
            break;
        }

        println!("-----");
    }
}
