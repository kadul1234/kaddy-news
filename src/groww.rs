use crate::models::MarketIndex;
use reqwest::Client;
use serde::Deserialize;

const GROWW_API_BASE: &str = "https://groww.in/v1/api";

// (display name, Groww search_id, exchange)
const INDICES: &[(&str, &str, &str)] = &[
    ("Nifty 50", "GIDXNIFTY50", "NSE"),
    ("Sensex", "GIDXBSESN", "BSE"),
    ("Bank Nifty", "GIDXBANKNIFTY", "NSE"),
    ("Nifty IT", "GIDXNIFTYIT", "NSE"),
    ("Nifty Midcap 100", "GIDXNIFTYMIDCAP100", "NSE"),
];

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LiveDataResponse {
    ltp: Option<f64>,
    day_change: Option<f64>,
    day_change_perc: Option<f64>,
    open: Option<f64>,
    high: Option<f64>,
    low: Option<f64>,
    previous_close: Option<f64>,
}

pub struct GrowwRepository {
    client: Client,
}

impl GrowwRepository {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn fetch_market_indices(&self) -> Vec<MarketIndex> {
        let mut results = Vec::new();
        for (name, search_id, exchange) in INDICES {
            if let Some(index) = self.fetch_single_index(name, search_id, exchange).await {
                results.push(index);
            }
        }
        results
    }

    async fn fetch_single_index(
        &self,
        name: &str,
        search_id: &str,
        exchange: &str,
    ) -> Option<MarketIndex> {
        let url = format!(
            "{}/stocks_data/v1/tr_live_data?project={}&exchange={}",
            GROWW_API_BASE, search_id, exchange
        );

        let response = self
            .client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0 (compatible; KaddyNews/1.0)")
            .header("Accept", "application/json")
            .send()
            .await
            .ok()?;

        if !response.status().is_success() {
            eprintln!("[Groww] {} returned {}", name, response.status());
            return None;
        }

        let data: LiveDataResponse = response.json().await.ok()?;
        let ltp = data.ltp?;
        let change = data.day_change.unwrap_or(0.0);
        let percent_change = data.day_change_perc.unwrap_or(0.0);

        Some(MarketIndex {
            name: name.to_string(),
            symbol: search_id.to_string(),
            ltp,
            change,
            percent_change,
            open: data.open.unwrap_or(0.0),
            high: data.high.unwrap_or(0.0),
            low: data.low.unwrap_or(0.0),
            prev_close: data.previous_close.unwrap_or(0.0),
            is_positive: change >= 0.0,
        })
    }
}
