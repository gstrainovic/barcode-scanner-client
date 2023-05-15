use serde::Deserialize;
use serde_json::{Map, Value, json};
use config::STRAPI_URL;

#[derive(Deserialize, Debug)]
pub struct IdAtr {
    pub id: Value,
    pub attributes: Map<String, Value>,
}

#[derive(Deserialize, Debug)]
pub struct BarcodeData {
    pub data: IdAtr,
}

#[tokio::main]
pub async fn write_barcode(
    barcode: String,
    user: String,
    jwt: &str,
    lager_user_ids: Vec<i16>,
) -> Result<BarcodeData, reqwest::Error> {

    println!("Barcode: {}", barcode);
    println!("User: {}", user);
    println!("JWT: {}", jwt);
    println!("Lager Users: {:?}", lager_user_ids);

    let url = format!("{}/api/barcodes", STRAPI_URL);

    let client = reqwest::Client::builder().build()?;

    let res = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", jwt))
        .json(&json!({
          "data": {
            "barcode": barcode,
            "users_permissions_user": user,
            "lager_mitarbeiter": lager_user_ids
          }
        }))
        .send()
        .await?;

    let body = res.text().await?;

    println!("Body:\n{}", body);

    Ok(serde_json::from_str(&body).unwrap())
}