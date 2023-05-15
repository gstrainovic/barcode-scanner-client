use config::STRAPI_URL;
use crate::loginfn::User;

// get all users with the role 'Lager'
#[tokio::main]
pub async fn get_lager_users(jwt: String) -> Result<Vec<User>, reqwest::Error> {
    let url = format!("{}{}", STRAPI_URL, "/api/users");

    let client = reqwest::Client::new();

    let res = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", jwt))
        .send()
        .await?
        .json::<Vec<User>>()
        .await?;

    let lager_users: Vec<User> = res
        .into_iter()
        .filter(|user| user.rolle == "Lager")
        .collect();

    return Ok(lager_users);
}