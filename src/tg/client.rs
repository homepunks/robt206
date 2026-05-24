use crate::tg::models::{Response, Update, User, File};

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

    pub async fn get_file(&self, file_id: &str) -> anyhow::Result<File> {
        let url = format!("https://api.telegram.org/bot{}/getFile", self.token);
        self.http
            .get(&url)
            .query(&[("file_id", file_id)])
            .send()
            .await?
            .json::<Response<File>>()
            .await?
            .into_result()
    }

    pub async fn download_file(&self, file_path: &str) -> anyhow::Result<Vec<u8>> {
        let url = format!("https://api.telegram.org/file/bot{}/{}", self.token, file_path);
        let bytes = self.http
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .bytes().await?;

        Ok(bytes.to_vec())
    }

    /// I need some time to think if I need get_file and download_file methods
    /// to be decoupled or merged within extract_bytes.
    pub async fn extract_bytes(&self, file_id: &str) -> anyhow::Result<Vec<u8>> {
        let file = self.get_file(file_id).await?;
        let Some(file_path) = file.file_path.as_deref() else {
            anyhow::bail!("Telegram API did not return a file_path. Likely, the file is too big (>20Mb)");
        };
        
        self.download_file(file_path).await
        
    }
}
