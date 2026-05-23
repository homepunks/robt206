use crate::tg::models::{Response, Update, User};

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

        self.http
            .get(&url)
            .send()
            .await?
            .json::<Response<User>>()
            .await?
            .into_result()
    }

    pub async fn get_updates(&self, offset: i64, timeout: u32) -> anyhow::Result<Vec<Update>> {
        let url = format!("https://api.telegram.org/bot{}/getUpdates", self.token);
        self.http
            .get(&url)
            .query(&[
                ("offset", offset.to_string()),
                ("timeout", timeout.to_string()),
            ])
            .send()
            .await?
            .json::<Response<Vec<Update>>>()
            .await?
            .into_result()
    }
}
