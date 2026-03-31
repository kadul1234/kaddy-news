use crate::models::Article;

const COUNTRY_KEYWORDS: &[(&str, &[&str])] = &[
    ("United States",    &["united states", " u.s.", "usa", "american", "washington d.c", "white house", "pentagon", "new york", "california", "texas", "chicago", "biden", "trump", "harris"]),
    ("United Kingdom",   &["united kingdom", "britain", "british", "england", "london", "scotland", "wales", "downing street", "westminster", "starmer", "sunak"]),
    ("China",            &["china", "chinese", "beijing", "shanghai", "xi jinping", "hong kong", "taiwan strait", "prc"]),
    ("Russia",           &["russia", "russian", "moscow", "putin", "kremlin", "siberia"]),
    ("India",            &["india", "indian", "new delhi", "mumbai", "modi", "bjp", "delhi", "chennai", "kolkata", "bengaluru"]),
    ("Germany",          &["germany", "german", "berlin", "munich", "hamburg", "scholz", "bundesbank", "bundestag"]),
    ("France",           &["france", "french", "paris", "macron", "marseille", "lyon", "élysée"]),
    ("Japan",            &["japan", "japanese", "tokyo", "osaka", "kyoto", "ishiba", "kishida"]),
    ("Australia",        &["australia", "australian", "sydney", "canberra", "melbourne", "brisbane", "albanese"]),
    ("Canada",           &["canada", "canadian", "ottawa", "toronto", "vancouver", "trudeau", "carney"]),
    ("Brazil",           &["brazil", "brazilian", "brasília", "sao paulo", "rio de janeiro", "lula"]),
    ("Argentina",        &["argentina", "argentinian", "buenos aires", "milei"]),
    ("Mexico",           &["mexico", "mexican", "mexico city", "guadalajara", "monterrey", "sheinbaum"]),
    ("South Korea",      &["south korea", "korean", "seoul", "busan", "yoon", "han duck-soo"]),
    ("North Korea",      &["north korea", "pyongyang", "kim jong", "dprk"]),
    ("Ukraine",          &["ukraine", "ukrainian", "kyiv", "zelensky", "zelenskyy", "kharkiv", "odessa"]),
    ("Israel",           &["israel", "israeli", "tel aviv", "jerusalem", "netanyahu", "idf", "gaza", "west bank"]),
    ("Iran",             &["iran", "iranian", "tehran", "khamenei", "irgc", "rouhani"]),
    ("Saudi Arabia",     &["saudi arabia", "saudi", "riyadh", "mbs", "aramco"]),
    ("Turkey",           &["turkey", "turkish", "ankara", "istanbul", "erdogan", "akp"]),
    ("Pakistan",         &["pakistan", "pakistani", "islamabad", "karachi", "lahore", "imran khan", "sharif"]),
    ("Indonesia",        &["indonesia", "indonesian", "jakarta", "bali", "prabowo"]),
    ("Nigeria",          &["nigeria", "nigerian", "abuja", "lagos", "tinubu"]),
    ("South Africa",     &["south africa", "south african", "johannesburg", "cape town", "ramaphosa", "anc"]),
    ("Kenya",            &["kenya", "kenyan", "nairobi", "ruto"]),
    ("Egypt",            &["egypt", "egyptian", "cairo", "el-sisi", "suez"]),
    ("Ethiopia",         &["ethiopia", "ethiopian", "addis ababa", "abiy ahmed"]),
    ("Ukraine",          &["ukraine", "ukrainian", "kyiv", "zelensky"]),
    ("Poland",           &["poland", "polish", "warsaw", "tusk", "sejm"]),
    ("Netherlands",      &["netherlands", "dutch", "amsterdam", "the hague", "wilders", "rutte"]),
    ("Sweden",           &["sweden", "swedish", "stockholm", "kristersson"]),
    ("Spain",            &["spain", "spanish", "madrid", "barcelona", "sanchez"]),
    ("Italy",            &["italy", "italian", "rome", "milan", "meloni"]),
    ("Switzerland",      &["switzerland", "swiss", "bern", "zurich", "geneva"]),
    ("Singapore",        &["singapore", "singaporean"]),
    ("New Zealand",      &["new zealand", "auckland", "wellington", "luxon", "ardern"]),
    ("Bangladesh",       &["bangladesh", "bangladeshi", "dhaka", "yunus"]),
    ("Ukraine",          &["ukraine", "ukrainian", "kyiv"]),
    ("Syria",            &["syria", "syrian", "damascus", "aleppo", "al-sharaa"]),
    ("Iraq",             &["iraq", "iraqi", "baghdad", "mosul"]),
    ("Lebanon",          &["lebanon", "lebanese", "beirut"]),
    ("Venezuela",        &["venezuela", "venezuelan", "caracas", "maduro"]),
    ("Colombia",         &["colombia", "colombian", "bogota", "petro"]),
    ("Chile",            &["chile", "chilean", "santiago", "boric"]),
    ("Peru",             &["peru", "peruvian", "lima", "boluarte"]),
    ("Philippines",      &["philippines", "philippine", "manila", "marcos"]),
    ("Thailand",         &["thailand", "thai", "bangkok"]),
    ("Vietnam",          &["vietnam", "vietnamese", "hanoi", "ho chi minh"]),
    ("Myanmar",          &["myanmar", "burmese", "rangoon", "yangon", "naypyidaw"]),
    ("Afghanistan",      &["afghanistan", "afghan", "kabul", "taliban"]),
    ("Sudan",            &["sudan", "sudanese", "khartoum", "rsf"]),
    ("Somalia",          &["somalia", "somali", "mogadishu", "al-shabaab"]),
    ("Libya",            &["libya", "libyan", "tripoli", "benghazi"]),
    ("Yemen",            &["yemen", "yemeni", "sanaa", "houthi"]),
    ("Palestine",        &["palestine", "palestinian", "ramallah", "hamas", "hezbollah"]),
    ("Jordan",           &["jordan", "jordanian", "amman", "king abdullah"]),
    ("Morocco",          &["morocco", "moroccan", "rabat", "casablanca"]),
    ("Algeria",          &["algeria", "algerian", "algiers"]),
    ("Tunisia",          &["tunisia", "tunisian", "tunis"]),
    ("Zimbabwe",         &["zimbabwe", "zimbabwean", "harare", "mnangagwa"]),
    ("Ghana",            &["ghana", "ghanaian", "accra", "mahama"]),
    ("Tanzania",         &["tanzania", "tanzanian", "dar es salaam"]),
    ("Uganda",           &["uganda", "ugandan", "kampala", "museveni"]),
    ("Senegal",          &["senegal", "senegalese", "dakar", "faye"]),
    ("Democratic Republic of Congo", &["congo", "congolese", "kinshasa", "drc"]),
    ("Czech Republic",   &["czech", "prague", "fiala"]),
    ("Hungary",          &["hungary", "hungarian", "budapest", "orban"]),
    ("Romania",          &["romania", "romanian", "bucharest"]),
    ("Greece",           &["greece", "greek", "athens", "mitsotakis"]),
    ("Portugal",         &["portugal", "portuguese", "lisbon", "montenegro"]),
    ("Austria",          &["austria", "austrian", "vienna"]),
    ("Belgium",          &["belgium", "belgian", "brussels"]),
    ("Denmark",          &["denmark", "danish", "copenhagen", "frederiksen"]),
    ("Finland",          &["finland", "finnish", "helsinki", "orpo"]),
    ("Norway",           &["norway", "norwegian", "oslo", "støre"]),
];

pub fn classify_country(article: &Article) -> String {
    let text = format!("{} {}", article.title, article.description).to_lowercase();

    let mut best_country = article.country.clone();
    let mut best_score = 0usize;

    for (country, keywords) in COUNTRY_KEYWORDS {
        let score = keywords.iter().filter(|&&kw| text.contains(kw)).count();
        if score > best_score {
            best_score = score;
            best_country = country.to_string();
        }
    }

    best_country
}

pub fn categorize(mut articles: Vec<Article>) -> Vec<Article> {
    for article in &mut articles {
        article.country = classify_country(article);
    }
    articles
}
