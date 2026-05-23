#[derive(serde::Deserialize, Debug)]
pub struct User {
    id: i64,
    first_name: String,
    username: Option<String>,
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
    update_id: i64,
    message: Option<Message>,
}

#[derive(serde::Deserialize, Debug)]
struct Message {
    message_id: i64,
    date: i64,
    from: Option<User>,
    chat: Chat,
    voice: Option<Voice>,
    audio: Option<Audio>,
}

#[derive(serde::Deserialize, Debug)]
struct Chat {
    id: i64,
    #[serde(rename = "type")]
    chat_type: String,
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
