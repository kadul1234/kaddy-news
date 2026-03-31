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

pub struct AppState {
    pub by_country: BTreeMap<String, Vec<Article>>,
    pub last_updated: String,
}

pub type SharedState = Arc<RwLock<AppState>>;
