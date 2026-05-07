use serde::Serialize;
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize)]
pub struct Article {
    pub title: String,
    pub description: String,
    pub url: String,
    pub source: String,
    pub country: String,
    pub published: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NewsSource {
    pub name: &'static str,
    pub feed_url: &'static str,
    pub default_country: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct MarketIndex {
    pub name: String,
    pub symbol: String,
    pub ltp: f64,
    pub change: f64,
    pub percent_change: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub prev_close: f64,
    pub is_positive: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct MarketData {
    pub indices: Vec<MarketIndex>,
    pub updated: String,
}

pub struct AppState {
    pub by_country: BTreeMap<String, Vec<Article>>,
    pub last_updated: String,
    pub market: MarketData,
}

pub type SharedState = Arc<RwLock<AppState>>;
