use config::STRAPI_URL;

use serde::Deserialize;
use serde_json::{json, Map, Value};

#[derive(Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub id: i32,
    pub rolle: String,
}

#[derive(Deserialize, Debug)]
pub struct JWT {
    pub jwt: Option<String>,
    pub error: Option<Map<String, Value>>,
    pub user: Option<User>,
}

#[tokio::main]
pub async fn loginfn(user: String, pass: String) -> Result<JWT, reqwest::Error> {
    let url = format!("{}/api/auth/local", STRAPI_URL);

    let client = reqwest::Client::builder().build()?;

    let res = client
        .post(&url)
        .json(&json!({
          "identifier": user,
          "password": pass
        }))
        .send()
        .await?;

    let body = res.text().await?;

    Ok(serde_json::from_str(&body).unwrap())
}