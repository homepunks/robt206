use crate::tg::models::{Response, User};

pub struct Client {
    http: reqwest::Client,
    token: String,
}

impl Client {
    pub fn new(token: String) -> Self {
        Self {
            http: reqwest::Client::new(),
            token,
        }
    }

    pub async fn get_me(&self) -> anyhow::Result<User> {
        let url = format!("https://api.telegram.org/bot{}/getMe", self.token);

        reqwest::get(&url)
            .await?
            .json::<Response<User>>()
            .await?
            .into_result()
    }
}
