mod api;
mod categorizer;
mod models;
mod scraper;
mod storage;

use axum::{routing::get, Router};
use chrono::Utc;
use models::{AppState, SharedState};
use reqwest::Client;
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok(); // load .env if present (ignored in production)

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".into())
        .parse()
        .expect("PORT must be a number");

    let interval_mins: u64 = std::env::var("SCRAPE_INTERVAL_MINS")
        .unwrap_or_else(|_| "5".into())
        .parse()
        .expect("SCRAPE_INTERVAL_MINS must be a number");

    let bind_addr = format!("0.0.0.0:{port}");
    let state: SharedState = Arc::new(RwLock::new(AppState {
        by_country: BTreeMap::new(),
        last_updated: String::from("Never"),
    }));

    let client = Client::builder()
        .timeout(Duration::from_secs(20))
        .build()
        .expect("Failed to build HTTP client");

    // Initial scrape before accepting requests
    println!("Running initial scrape...");
    run_scrape(&client, &state).await;

    // Background scraping loop
    {
        let client = client.clone();
        let state = Arc::clone(&state);
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(interval_mins * 60)).await;
                println!("[{}] Running scheduled scrape...", timestamp());
                run_scrape(&client, &state).await;
            }
        });
    }

    let app = Router::new()
        .route("/", get(api::index))
        .route("/style.css", get(api::stylesheet))
        .route("/app.js", get(api::javascript))
        .route("/countries-110m.json", get(api::world_atlas))
        .route("/india-official.geojson", get(api::india_boundary))
        .route("/api/countries", get(api::get_countries))
        .route("/api/news", get(api::get_news))
        .with_state(Arc::clone(&state));

    println!("Server running → http://localhost:{port}");
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .expect("Failed to bind");
    axum::serve(listener, app).await.unwrap();
}

async fn run_scrape(client: &Client, state: &SharedState) {
    let sources = scraper::get_news_sources();
    println!("[{}] Fetching {} sources...", timestamp(), sources.len());

    let tasks: Vec<_> = sources
        .iter()
        .map(|source| {
            let client = client.clone();
            let source = source.clone();
            tokio::spawn(async move { scraper::fetch_articles(&client, &source).await })
        })
        .collect();

    let mut all_articles = Vec::new();
    for task in tasks {
        if let Ok(articles) = task.await {
            all_articles.extend(articles);
        }
    }

    println!("  Fetched {} articles. Categorizing...", all_articles.len());
    let categorized = categorizer::categorize(all_articles);

    // Drop articles published more than 1 hour ago
    let cutoff = Utc::now() - chrono::Duration::hours(1);
    let categorized: Vec<_> = categorized
        .into_iter()
        .filter(|a| {
            a.published
                .as_deref()
                .and_then(|s| chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M UTC").ok())
                .map(|dt| dt.and_utc() >= cutoff)
                .unwrap_or(false)
        })
        .collect();
    println!("  {} articles within last 1 hour.", categorized.len());

    // Write the text file
    storage::save_to_file(&categorized, Path::new("news_articles.txt")).ok();

    // Update shared state
    let mut lock = state.write().await;
    lock.last_updated = Utc::now().format("%Y-%m-%d %H:%M UTC").to_string();
    lock.by_country.clear();
    for article in categorized {
        lock.by_country
            .entry(article.country.clone())
            .or_default()
            .push(article);
    }
    let total: usize = lock.by_country.values().map(|v| v.len()).sum();
    println!(
        "  Done — {} articles across {} countries. Updated at {}",
        total,
        lock.by_country.len(),
        lock.last_updated
    );
}

fn timestamp() -> String {
    Utc::now().format("%H:%M:%S").to_string()
}
