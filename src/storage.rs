use crate::models::Article;
use chrono::Utc;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub fn save_to_file(articles: &[Article], path: &Path) -> std::io::Result<()> {
    let mut by_country: BTreeMap<&str, Vec<&Article>> = BTreeMap::new();

    for article in articles {
        by_country
            .entry(article.country.as_str())
            .or_default()
            .push(article);
    }

    let mut output = String::new();

    output.push_str("================================================================================\n");
    output.push_str("                          NEWS ARTICLES REPORT\n");
    output.push_str("================================================================================\n");
    output.push_str(&format!("Generated : {}\n", Utc::now().format("%Y-%m-%d %H:%M UTC")));
    output.push_str(&format!("Total     : {} articles\n", articles.len()));
    output.push_str(&format!("Countries : {}\n", by_country.len()));
    output.push_str("================================================================================\n");

    for (country, country_articles) in &by_country {
        output.push_str(&format!("\n\n{}\n", "=".repeat(80)));
        output.push_str(&format!("  COUNTRY: {}  ({} articles)\n", country, country_articles.len()));
        output.push_str(&format!("{}\n", "=".repeat(80)));

        for (i, article) in country_articles.iter().enumerate() {
            output.push_str(&format!("\n  {}. {}\n", i + 1, article.title));
            output.push_str(&format!("     Source   : {}\n", article.source));
            if let Some(ref pub_date) = article.published {
                output.push_str(&format!("     Published: {}\n", pub_date));
            }
            if !article.url.is_empty() {
                output.push_str(&format!("     URL      : {}\n", article.url));
            }
            if !article.description.is_empty() {
                let desc = truncate(&article.description, 200);
                output.push_str(&format!("     Summary  : {}\n", desc));
            }
        }
    }

    output.push_str("\n\n================================================================================\n");
    output.push_str("                               END OF REPORT\n");
    output.push_str("================================================================================\n");

    fs::write(path, output)
}

fn truncate(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max_chars).collect();
        format!("{}...", truncated.trim_end())
    }
}
