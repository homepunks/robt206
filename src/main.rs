use std::env;

#[derive(serde::Deserialize, Debug)]
struct TgResponse<T> {
    ok: bool,
    result: Option<T>,
    description: Option<String>,
    ec: Option<i32>,
}

#[derive(serde::Deserialize, Debug)]
struct User {
    id: i64,
    is_bot: bool,
    first_name: String,
    username: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let token = env::var("TG_BOT_TOKEN")?;
    let getme_url = format!("https://api.telegram.org/bot{}/getMe", token);

    let resp = reqwest::get(getme_url).await?.json::<TgResponse<User>>().await?;
    println!("{:#?}", resp);

    Ok(())
}
