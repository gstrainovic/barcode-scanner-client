use config::STRAPI_URL;
use crate::loginfn::User;

// get all users with the role 'Lager'
#[tokio::main]
pub async fn get_lager_users(jwt: &str) -> Result<Vec<String>, reqwest::Error> {
    let url = format!("{}{}", STRAPI_URL, "/api/users");

    let client = reqwest::Client::new();

    let res = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", jwt))
        .send()
        .await?
        .json::<Vec<User>>()
        .await?;

    // filter the users with the role 'Lager' and get the usernames
    let lager_users: Vec<String> = res
        .into_iter()
        .filter(|user| user.rolle == "Lager")
        .map(|user| user.username)
        .collect();

    return Ok(lager_users);
}