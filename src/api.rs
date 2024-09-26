use serde_json::{json, Map, Value};
use url::Url;

use crate::config::Config;

#[derive(Debug, PartialEq, Eq)]
struct UrlSplitResult {
    base_url: Url,
    flash_id: String,
}

pub struct Flash {
    client: reqwest::Client,
    config: Config,
    base_url: Url,
    flash_id: String,
}

impl Flash {
    pub fn new(config: Config) -> Self {
        let UrlSplitResult { base_url, flash_id } = split_url(&config.url);

        Flash {
            client: reqwest::Client::new(),
            config,
            base_url,
            flash_id,
        }
    }

    pub async fn update(self: &Self, script: Option<String>) {
        let Flash {
            client,
            config,
            base_url,
            flash_id,
        } = self;
        let res = client
            .post(format!("{}api/flash/update", base_url))
            .header("Content-Type", "application/json")
            .bearer_auth(&config.token)
            .body(create_body(config, flash_id, script))
            .send()
            .await
            .unwrap();
        res.error_for_status().unwrap();
    }
}

fn split_url(input: &str) -> UrlSplitResult {
    let url = Url::parse(input).unwrap();
    let base_url = url.join("../../").unwrap();
    let flash_id = url.path_segments().unwrap().last().unwrap().to_string();
    return UrlSplitResult { base_url, flash_id };
}

fn create_body(config: &Config, flash_id: &str, script: Option<String>) -> String {
    let mut value: Map<String, Value> = Map::new();
    value.insert(String::from("flashId"), json!(flash_id));
    if let Some(title) = &config.title {
        value.insert(String::from("title"), json!(title));
    }
    if let Some(summary) = &config.summary {
        value.insert(String::from("summary"), json!(summary));
    }
    if let Some(script) = script {
        value.insert(String::from("script"), json!(script));
    }
    if let Some(visibility) = &config.visibility {
        value.insert(String::from("visibility"), json!(visibility));
    }
    Value::Object(value).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_url() {
        assert_eq!(
            split_url("https://example.com/play/abcde12345"),
            UrlSplitResult {
                base_url: Url::parse("https://example.com/").unwrap(),
                flash_id: String::from("abcde12345"),
            }
        )
    }
}
