use core::str;
use reqwest::Url;
use exitfailure::ExitFailure;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    pub name: String,
    pub postal_code: i16,
    pub building: Option<String>,
}

impl Address {
    pub async fn get(url: String) -> Result<Self, ExitFailure> {
        let url: Url = Url::parse(&url)?;
        let res: Address = reqwest::get(url).await?.json::<Address>().await?;

        Ok(res)
    }
}
