use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearxQuery {
    pub query: String,
    pub site: Option<String>,
    pub engines: Vec<String>,
    pub page: Option<usize>,
    pub lang: Option<String>,
}

impl SearxQuery {
    pub fn new(query: &str) -> Self {
        Self {
            query: query.to_string(),
            site: None,
            engines: vec![],
            page: None,
            lang: None,
        }
    }

    pub fn in_website(mut self, url: &str) -> Self {
        self.site = Some(url.to_string());
        self
    }

    pub fn fetch_page(mut self, page: usize) -> Self {
        self.page = Some(page);
        self
    }

    pub fn with_lang(mut self, lang: &str) -> Self {
        self.lang = Some(lang.to_string());
        self
    }

    pub fn set_lang(&mut self, lang: &str) {
        self.lang = Some(lang.to_string());
    }

    pub fn set_website(&mut self, url: &str) {
        self.site = Some(url.to_string());
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearxResponse {
    pub success: bool,
    pub results: Vec<SearxResultEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearxResultEntry {
    pub title: String,
    pub content: String,
    pub url: String,
    pub engine: String,
    pub published_date: String,
}

pub async fn search(engine_url: &str, query: &SearxQuery) -> SearxResponse {
    let scope = if let Some(site) = &query.site {
        format!("site:{}%20", site)
    } else {
        "".to_string()
    };
    let engines_params = if query.engines.len() > 0 {
        format!("&engines={}", query.engines.join(","))
    } else {
        "".to_string()
    };

    let page_opt = if let Some(page) = &query.page {
        format!("&pageno={}", page + 1)
    } else {
        "".to_string()
    };

    let lang = if let Some(lang) = &query.lang {
        format!("&language={}", lang)
    } else {
        "".to_string()
    };

    let url = format!(
        "{}/search?q={}{}{}&format=json{}{}",
        engine_url, scope, lang, query.query, engines_params, page_opt
    );

   
    // println!("Search URL : {}", url);
    let client = reqwest::Client::new();
    let server_response = client.get(&url).send().await;

    let results: SearxResponse = match server_response {
        Ok(response) => {
            // println!("-> Response : {}", response.status());
            let body = response.json::<Value>().await;

            match body {
                Ok(body) => engine_body_to_results(&body),
                Err(_) => SearxResponse {
                    success: false,
                    results: vec![],
                },
            }
        }
        Err(_) => {
            // println!("-> EmptyResponse");
            SearxResponse {
                success: false,
                results: vec![],
            }
        }
    };
    results
}

fn engine_body_to_results(json_body: &Value) -> SearxResponse {
    if let Some(results) = json_body.get("results") {
        if let Some(results_array) = results.as_array() {
            let results_entries: Vec<SearxResultEntry> = results_array
                .iter()
                .map(|entry| SearxResultEntry {
                    title: get_json_string_property_value(entry, "title", ""),
                    content: get_json_string_property_value(entry, "content", ""),
                    url: get_json_string_property_value(entry, "url", ""),
                    engine: get_json_string_property_value(entry, "engine", ""),
                    published_date: get_json_string_property_value(entry, "publishedDate", ""),
                })
                .collect();
            return SearxResponse {
                success: true,
                results: results_entries,
            };
        }
    }
    SearxResponse {
        success: false,
        results: vec![],
    }
}

fn get_json_string_property_value(json: &Value, property: &str, default_value: &str) -> String {
    if let Some(prop) = json.get(property) {
        if let Some(value) = prop.as_str() {
            return value.to_string();
        }
    }
    default_value.to_string()
}

