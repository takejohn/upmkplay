use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct Config {
    pub host: String,

    pub token: String,

    pub id: String,

    pub title: Option<String>,

    pub summary: Option<String>,

    pub visibility: Option<Visibility>,
}

impl Config {
    pub fn load(filename: &str) -> Config {
        let input = fs::read_to_string(filename).unwrap();
        return Self::parse(&input);
    }

    fn parse(input: &str) -> Config {
        return serde_json::from_str(input).unwrap();
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Public,

    Private,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config() {
        assert_eq!(
            Config::parse(
                r#"{
                    "host": "https://misskey.io",
                    "token": "abc",
                    "id": "123",
                    "title": "Test Play"
                }"#
            ),
            Config {
                host: String::from("https://misskey.io"),
                token: String::from("abc"),
                id: String::from("123"),
                title: Some(String::from("Test Play")),
                summary: None,
                visibility: None,
            }
        );

        assert_eq!(
            Config::parse(
                r#"{
                    "host": "https://misskey.io",
                    "token": "abc",
                    "id": "123",
                    "title": "Test Play",
                    "summary": "Hello, world!",
                    "visibility": "private"
                }"#
            ),
            Config {
                host: String::from("https://misskey.io"),
                token: String::from("abc"),
                id: String::from("123"),
                title: Some(String::from("Test Play")),
                summary: Some(String::from("Hello, world!")),
                visibility: Some(Visibility::Private),
            }
        );
    }
}
