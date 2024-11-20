use core::str;
use reqwest::Url;
use exitfailure::ExitFailure;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub email: Option<String>, // optional struct
}

impl User {
    pub async fn get(url: String) -> Result<Self, ExitFailure> {
        let url: Url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.json::<User>().await?;

        Ok(res)
    }
}
