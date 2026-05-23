use robt206::tg;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let token = env::var("TG_BOT_TOKEN")?;
    let client = tg::Client::new(token);

    let resp = client.get_me().await?;
    println!("{:#?}", resp);

    Ok(())
}
