use crate::models::{Article, NewsSource};
use reqwest::Client;

pub fn get_news_sources() -> Vec<NewsSource> {
    vec![
        // United States
        NewsSource { name: "CNN",              feed_url: "http://rss.cnn.com/rss/edition.rss",                                      default_country: "United States" },
        NewsSource { name: "NPR",              feed_url: "https://feeds.npr.org/1001/rss.xml",                                      default_country: "United States" },
        NewsSource { name: "AP News",          feed_url: "https://feeds.apnews.com/apnews/topnews",                                 default_country: "United States" },
        // United Kingdom
        NewsSource { name: "BBC News",         feed_url: "https://feeds.bbci.co.uk/news/rss.xml",                                  default_country: "United Kingdom" },
        NewsSource { name: "The Guardian",     feed_url: "https://www.theguardian.com/world/rss",                                  default_country: "United Kingdom" },
        // Germany
        NewsSource { name: "Deutsche Welle",   feed_url: "https://rss.dw.com/xml/rss-en-all",                                      default_country: "Germany" },
        // France
        NewsSource { name: "France 24",        feed_url: "https://www.france24.com/en/rss",                                        default_country: "France" },
        // Japan
        NewsSource { name: "Japan Times",      feed_url: "https://www.japantimes.co.jp/feed/",                                     default_country: "Japan" },
        // Australia
        NewsSource { name: "ABC Australia",    feed_url: "https://www.abc.net.au/news/feed/51120/rss.xml",                         default_country: "Australia" },
        // India
        NewsSource { name: "Times of India",   feed_url: "https://timesofindia.indiatimes.com/rssfeedstopstories.cms",             default_country: "India" },
        // Canada
        NewsSource { name: "CBC News",         feed_url: "https://www.cbc.ca/cmlink/rss-topstories",                               default_country: "Canada" },
        // International / Middle East
        NewsSource { name: "Al Jazeera",       feed_url: "https://www.aljazeera.com/xml/rss/all.xml",                              default_country: "International" },
        // Turkey
        NewsSource { name: "TRT World",        feed_url: "https://www.trtworld.com/rss",                                           default_country: "Turkey" },
        // Pakistan
        NewsSource { name: "Dawn",             feed_url: "https://www.dawn.com/feed",                                              default_country: "Pakistan" },
        // Indonesia
        NewsSource { name: "Jakarta Post",     feed_url: "https://www.thejakartapost.com/rss/index.xml",                           default_country: "Indonesia" },
        // South Korea
        NewsSource { name: "Korea Herald",     feed_url: "http://www.koreaherald.com/rss/articlelist.php?id=0",                    default_country: "South Korea" },
        // China
        NewsSource { name: "CGTN",             feed_url: "https://www.cgtn.com/subscribe/rss/section/world.xml",                   default_country: "China" },
        // Nigeria
        NewsSource { name: "Vanguard Nigeria", feed_url: "https://www.vanguardngr.com/feed/",                                      default_country: "Nigeria" },
        // South Africa
        NewsSource { name: "Daily Maverick",   feed_url: "https://www.dailymaverick.co.za/rss/",                                   default_country: "South Africa" },
        // Ukraine
        NewsSource { name: "Kyiv Independent", feed_url: "https://kyivindependent.com/feed/",                                      default_country: "Ukraine" },
        // Israel
        NewsSource { name: "Jerusalem Post",   feed_url: "https://www.jpost.com/rss/rssfeedsfrontpage.aspx",                       default_country: "Israel" },
        // Poland
        NewsSource { name: "Notes from Poland",feed_url: "https://notesfrompoland.com/feed/",                                      default_country: "Poland" },
        // New Zealand
        NewsSource { name: "RNZ",              feed_url: "https://www.rnz.co.nz/rss/news.xml",                                     default_country: "New Zealand" },
        // Argentina
        NewsSource { name: "MercoPress",       feed_url: "https://en.mercopress.com/rss",                                          default_country: "Argentina" },
        // Mexico
        NewsSource { name: "Mexico News Daily",feed_url: "https://mexiconewsdaily.com/feed/",                                      default_country: "Mexico" },
        // Netherlands
        NewsSource { name: "NL Times",         feed_url: "https://nltimes.nl/rss.xml",                                             default_country: "Netherlands" },
        // Switzerland
        NewsSource { name: "Swissinfo",        feed_url: "https://www.swissinfo.ch/eng/rss/news?format=rss",                       default_country: "Switzerland" },
        // Bangladesh
        NewsSource { name: "Daily Star BD",    feed_url: "https://www.thedailystar.net/frontpage/rss.xml",                         default_country: "Bangladesh" },
        // Spain
        NewsSource { name: "El Pais English",  feed_url: "https://feeds.elpais.com/mrss-s/pages/ep/site/english.elpais.com/portada", default_country: "Spain" },
        // Italy
        NewsSource { name: "ANSA",             feed_url: "https://www.ansa.it/english/news/general_news/general_news_rss.xml",     default_country: "Italy" },
        // Sweden
        NewsSource { name: "The Local Sweden", feed_url: "https://feeds.thelocal.com/rss/se",                                      default_country: "Sweden" },
        // Kenya
        NewsSource { name: "The Star Kenya",   feed_url: "https://www.the-star.co.ke/rss/",                                        default_country: "Kenya" },
        // Singapore
        NewsSource { name: "CNA",              feed_url: "https://www.channelnewsasia.com/api/v1/rss-outbound-feed?_format=xml&category=10416", default_country: "Singapore" },
        // Brazil
        NewsSource { name: "Rio Times",        feed_url: "https://riotimesonline.com/feed/",                                       default_country: "Brazil" },
    ]
}

pub async fn fetch_articles(client: &Client, source: &NewsSource) -> Vec<Article> {
    let response = match client
        .get(source.feed_url)
        .header("User-Agent", "Mozilla/5.0 (compatible; NewsBot/1.0)")
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            eprintln!("[WARN] {}: {}", source.name, e);
            return vec![];
        }
    };

    let bytes = match response.bytes().await {
        Ok(b) => b,
        Err(e) => {
            eprintln!("[WARN] {}: failed to read body: {}", source.name, e);
            return vec![];
        }
    };

    let feed = match feed_rs::parser::parse(bytes.as_ref()) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("[WARN] {}: failed to parse feed: {}", source.name, e);
            return vec![];
        }
    };

    feed.entries
        .into_iter()
        .map(|entry| {
            let title = entry
                .title
                .map(|t| t.content.trim().to_string())
                .unwrap_or_else(|| "(no title)".to_string());

            let description = entry
                .summary
                .map(|s| strip_html(&s.content))
                .unwrap_or_default();

            let url = entry
                .links
                .into_iter()
                .next()
                .map(|l| l.href)
                .unwrap_or_default();

            let published = entry
                .published
                .map(|dt| dt.format("%Y-%m-%d %H:%M UTC").to_string());

            Article {
                title,
                description,
                url,
                source: source.name.to_string(),
                country: source.default_country.to_string(),
                published,
            }
        })
        .collect()
}

fn strip_html(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut in_tag = false;
    for ch in input.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => output.push(ch),
            _ => {}
        }
    }
    output
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string()
}
