use serde_json::{json, Map, Value};

use crate::config::Config;

pub struct Flash {
    client: reqwest::Client,
    config: Config,
}

impl Flash {
    pub fn new(config: Config) -> Self {
        Flash {
            client: reqwest::Client::new(),
            config,
        }
    }

    pub async fn update(self: &Self, script: Option<String>) {
        let Flash { client, config } = self;
        let res = client
            .post(format!("{}/api/flash/update", config.host))
            .header("Content-Type", "application/json")
            .bearer_auth(&config.token)
            .body(create_body(config, script))
            .send()
            .await
            .unwrap();
        res.error_for_status().unwrap();
    }
}

fn create_body(config: &Config, script: Option<String>) -> String {
    let mut value: Map<String, Value> = Map::new();
    value.insert(String::from("flashId"), json!(config.id));
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
