use anyhow::Result;
use serde::{Deserialize, Serialize};

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
struct DeeplResponse {
    translations: Vec<Translation>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Translation {
    text: String,
    detected_source_language: String,
}

#[tokio::main]
pub async fn deepl(endpoint: &str, api_key: &str, message: &str, lang: &str) -> Result<()> {
    let mut params = HashMap::new();
    params.insert("text", &message);
    params.insert("target_lang", &lang);

    let client = reqwest::Client::new();
    let res = client
        .post(endpoint)
        .header(AUTHORIZATION, "DeepL-Auth-Key ".to_owned() + api_key)
        .header(CONTENT_TYPE, "json")
        .form(&params)
        .send()
        .await?;

    if !&res.status().is_success() {
        eprintln!("failed to request: {:?}", res);
    }

    let p: DeeplResponse = serde_json::from_str(&res.text().await?)?;
    println!("{}", &p.translations[0].text);

    Ok(())
}
