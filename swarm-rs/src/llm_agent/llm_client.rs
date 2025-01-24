use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::{llm_message::LLMMessage, llm_response::LLMResponse};



#[derive(Serialize, Deserialize)]
pub struct LLMClient {
    endpoint: String,
    model: String,
    api_key: Option<String>,
}

impl LLMClient {
    pub fn new(endpoint: &str, model: &str, api_key: Option<String>) -> Self {
        Self {
            endpoint: endpoint.to_string(),
            model: model.to_string(),
            api_key,
        }
    }

    pub async fn autocomplete(
        &self,
        messages: &Vec<LLMMessage>,
        output_format: Option<String>,
    ) -> Result<LLMResponse, String> {
        let url = &self.endpoint;

        let mut headers = HeaderMap::new();
        if let Some(auth_key) = &self.api_key {
            let header_value = HeaderValue::from_str(&format!("Bearer {}", auth_key))
                .expect("Malformed header value");
            headers.insert(AUTHORIZATION, header_value);
        }

        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let json_body = match output_format {
            Some(output_format) => {
                let format_value: Value = serde_json::from_str(&output_format).unwrap();
                // println!("format : {:?}", format_value);
                json!({
                    "model": self.model,
                    "messages": messages,
                    "response_format" : {
                        "type": "json_schema",
                        "json_schema": format_value
                    }
                })
            }
            _ => json!({
                "model": self.model,
                "messages": messages
            }),
        };

        // println!("Query: {}", json_body);

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .headers(headers)
            .json(&json_body)
            .send()
            .await;

        match response {
            Ok(response) => {
                let body = response.json::<Value>().await;
                match body {
                    Ok(value) => {
                        // println!("LLM Response : {}", serde_json::to_string(&value).unwrap());
                        
                        let resp = serde_json::from_value::<LLMResponse>(value.clone());
                        if let Ok(resp) = resp {
                            Ok(resp)
                        } else {
                            Err("Unable to decode response".to_string())
                        }
                    }
                    Err(e) => Err(e.to_string()),
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }
}
