use config::STRAPI_URL;
use crate::loginfn::User;

#[tokio::main]
pub async fn get_users(jwt: String) -> Result<Vec<User>, reqwest::Error> {
    let url = format!("{}{}", STRAPI_URL, "/api/users");

    let client = reqwest::Client::new();

    let res = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", jwt))
        .send()
        .await?
        .json::<Vec<User>>()
        .await?;

    return Ok(res);
}