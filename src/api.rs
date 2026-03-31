use axum::{
    extract::{Query, State},
    response::{Html, Response},
    body::Body,
    http::header,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::models::{Article, SharedState};

pub async fn index() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

pub async fn world_atlas() -> Response<Body> {
    Response::builder()
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::CACHE_CONTROL, "public, max-age=86400")
        .body(Body::from(include_str!("../static/countries-110m.json")))
        .unwrap()
}

pub async fn india_boundary() -> Response<Body> {
    Response::builder()
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::CACHE_CONTROL, "public, max-age=86400")
        .body(Body::from(include_str!("../static/india-official.geojson")))
        .unwrap()
}

pub async fn stylesheet() -> Response<Body> {
    Response::builder()
        .header(header::CONTENT_TYPE, "text/css")
        .header(header::CACHE_CONTROL, "public, max-age=3600")
        .body(Body::from(include_str!("../static/style.css")))
        .unwrap()
}

pub async fn javascript() -> Response<Body> {
    Response::builder()
        .header(header::CONTENT_TYPE, "application/javascript")
        .header(header::CACHE_CONTROL, "public, max-age=3600")
        .body(Body::from(include_str!("../static/app.js")))
        .unwrap()
}

#[derive(Serialize)]
pub struct CountryInfo {
    name: String,
    count: usize,
}

#[derive(Serialize)]
pub struct CountriesResponse {
    updated: String,
    countries: Vec<CountryInfo>,
}

pub async fn get_countries(State(state): State<SharedState>) -> Json<CountriesResponse> {
    let lock = state.read().await;
    let mut countries: Vec<CountryInfo> = lock
        .by_country
        .iter()
        .map(|(name, articles)| CountryInfo {
            name: name.clone(),
            count: articles.len(),
        })
        .collect();
    countries.sort_by(|a, b| b.count.cmp(&a.count));
    Json(CountriesResponse {
        updated: lock.last_updated.clone(),
        countries,
    })
}

#[derive(Deserialize)]
pub struct NewsQuery {
    pub country: String,
}

pub async fn get_news(
    State(state): State<SharedState>,
    Query(q): Query<NewsQuery>,
) -> Json<Vec<Article>> {
    let lock = state.read().await;
    let articles = lock.by_country.get(&q.country).cloned().unwrap_or_default();
    Json(articles)
}
