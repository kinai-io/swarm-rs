use std::any::Any;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{
    agent::{Action, Agent, Output},
    prelude::Swarm,
};

use super::searx::{search, SearxQuery, SearxResponse, SearxResultEntry};

#[derive(Serialize, Deserialize)]
pub struct SearchQuery {
    pub terms: String,
    pub lang: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SearxAgent {
    id: String,
    endpoint: String,
    sites: Vec<String>,
}

impl SearxAgent {

    pub fn get_id(&self) -> String{
        self.id.to_string()
    }
}

#[async_trait]
impl Agent for SearxAgent {

    fn as_any(&self) -> &dyn Any {
        self
    }

    async fn execute(&self, action: &Action, _swarm: &Swarm) -> Output {
        let query: SearchQuery = action.get_payload().unwrap();
        let mut searx_query = SearxQuery::new(&query.terms);
        if let Some(lang) = &query.lang {
            searx_query.set_lang(lang);
        }
        if self.sites.is_empty() {
            let results = search(&self.endpoint, &searx_query).await;
            if results.success {
                Output::new_success(results)
            } else {
                Output::new_error("Search Error")
            }
        } else {
            let mut results: Vec<SearxResultEntry> = vec![];
            let mut success = true;
            for site in &self.sites {
                searx_query.set_website(site);
                let mut site_results = search(&self.endpoint, &searx_query).await;
                if site_results.success {
                    results.append(&mut site_results.results);
                } else {
                    // TODO Handle error
                    success = false;
                    break;
                }
            }
            if success {
                let resp = SearxResponse {
                    success: success,
                    results: results,
                };
                Output::new_success(resp)
            } else {
                Output::new_error("Search Error")
            }
        }
    }
}
