use serde::Deserialize;
use config::STRAPI_URL;
use sqlite::get_settings::get_settings as get_settings_sqlite;

#[derive(Deserialize, Debug)]
pub struct EinstellungenData {
    pub data: IdAtr,
}

#[derive(Deserialize, Debug)]
pub struct IdAtr {
    pub id: i16,
    pub attributes: Einstellungen,
}

#[derive(Deserialize, Debug)]
pub struct Einstellungen {
    pub Barcode_Mindestlaenge: i32,
    pub Leitcodes_Aktiv: bool,
    pub Ausnahmen_Aktiv: bool,
}

// get all exceptions from the database
#[tokio::main]
pub async fn get_settings(jwt: &str) -> Result<EinstellungenData, reqwest::Error> {
    let mut res = EinstellungenData {
        data: IdAtr {
            id: 0,
            attributes: Einstellungen {
                Barcode_Mindestlaenge: 0,
                Leitcodes_Aktiv: false,
                Ausnahmen_Aktiv: false,
            },
        },
    };

    if jwt == "" {
        res = get_settings_sqlite();
    } else {
        let url = format!("{}/api/einstellung", STRAPI_URL);
        let client = reqwest::Client::new();
        
        res = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", jwt))
        .send()
        .await?
        .json::<EinstellungenData>()
        .await?;
    }
    
    Ok(res)
}