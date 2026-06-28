#[derive(serde::Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub username: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum Response<T> {
    Ok {
        result: T,
    },
    Err {
        description: String,
        error_code: i32,
    },
}

#[derive(serde::Deserialize, Debug)]
pub struct Update {
    pub update_id: i64,
    pub message: Option<Message>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Message {
    pub message_id: i64,
    pub date: i64,
    pub from: Option<User>,
    pub chat: Chat,
    pub voice: Option<Voice>,
    pub audio: Option<Audio>,
    pub text: Option<String>,
    pub reply_to_message: Option<Box<Message>>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Chat {
    pub id: i64,
    #[serde(rename = "type")]
    pub chat_type: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct File {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: Option<i64>,
    pub file_path: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Voice {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: i32,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Audio {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: i32,
    pub mime_type: Option<String>,
    pub file_size: Option<i64>,
    pub title: Option<String>,
    pub performer: Option<String>,
}

impl<T> Response<T> {
    pub fn into_result(self) -> anyhow::Result<T> {
        match self {
            Self::Ok { result } => Ok(result),
            Self::Err {
                error_code,
                description,
            } => {
                anyhow::bail!("Telegram error {error_code}: {description}")
            }
        }
    }
}

impl Message {
    pub fn audio_file_id(&self) -> Option<&str> {
        self.voice
            .as_ref()
            .map(|v| v.file_id.as_str())
            .or_else(|| self.audio.as_ref().map(|a| a.file_id.as_str()))
    }
}
