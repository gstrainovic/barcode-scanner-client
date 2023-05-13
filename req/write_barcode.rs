use serde::Deserialize;
use serde_json::{Map, Value, json};
use config::STRAPI_URL;

#[derive(Deserialize, Debug)]
pub struct IdAtr {
    id: i16,
    attributes: Map<String, Value>,
}


#[derive(Deserialize, Debug)]
pub struct BarcodeData {
    data: IdAtr,
}

#[tokio::main]
pub async fn write_barcode(
    barcode: String,
    user: i16,
    jwt: &str,
) -> Result<BarcodeData, reqwest::Error> {
    let url = format!("{}/api/barcodes", STRAPI_URL);

    let client = reqwest::Client::builder().build()?;

    let res = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", jwt))
        .json(&json!({
          "data": {
            "barcode": barcode,
            "users_permissions_user": user
          }
        }))
        .send()
        .await?;

    let body = res.text().await?;

    println!("Body:\n{}", body);

    Ok(serde_json::from_str(&body).unwrap())
}