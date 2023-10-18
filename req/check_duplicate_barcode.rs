use serde::Deserialize;
use config::STRAPI_URL;

#[derive(Deserialize, Debug)]
pub struct Data {
    pub data: Option<Vec<serde_json::Value>>,
}

#[tokio::main]
pub async fn is_barcode_duplicate(jwt: &str, barcode: &str) -> Result<bool, reqwest::Error> {
    let url = format!("{}/api/barcodes?filters[barcode][$eq]={}", STRAPI_URL, barcode);
    let client = reqwest::Client::new();

    let res = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", jwt))
        .send()
        .await?
        .json::<Data>()
        .await?;
    
    if let Some(data) = res.data {
        if data.len() > 0 {
            return Ok(true);
        }
    }

    return Ok(false)
}