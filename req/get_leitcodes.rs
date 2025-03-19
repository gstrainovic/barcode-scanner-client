use serde::Deserialize;
use config::STRAPI_URL;

#[derive(Deserialize, Debug)]
pub struct Data {
    pub data: Vec<IdAtr>,
}

#[derive(Deserialize, Debug)]
pub struct IdAtr {
    pub id: i16,
    pub attributes: Leitcode,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Leitcode {
    pub Beschreibung: String,
    pub Mindeslaenge: i32,
    pub Leitcode_Buchstabe: DataBuchstaben,
    pub Produktion: bool,
}
#[derive(Deserialize, Debug)]
pub struct DataBuchstaben {
    pub data: Vec<IdAtrBuchstaben>,
}

#[derive(Deserialize, Debug)]
pub struct IdAtrBuchstaben {
    pub id: i16,
    pub attributes: LeitcodeBuchstabe,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct LeitcodeBuchstabe {
    pub Buchstabe: String,
    pub Position_Null_Beginnend: i32,
}

// get all exceptions from the database
#[tokio::main]
pub async fn get_leitcodes(jwt: &str) -> Result<Data, reqwest::Error> {
    let url = format!("{}/api/leitcodes?populate=*", STRAPI_URL);
    let client = reqwest::Client::new();

    let res = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", jwt))
        .send()
        .await?
        .json::<Data>()
        .await?;
    
    Ok(res)
}