use robt206::tg;
use std::env;

#[tokio::main]
#[allow(unreachable_code)]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let token = env::var("TG_BOT_TOKEN")?;
    let client = tg::Client::new(token);

    println!("INFO: starting polling...");
    let pending = client.get_updates(-1, 0).await?;
    let mut offset: i64 = pending.last().map(|upd| upd.update_id + 1).unwrap_or(0);
    loop {
        let mut updates = client.get_updates(offset, 25).await?;
        for update in updates {
            offset = update.update_id + 1;
            if update.is_audio() {
                println!("{update:#?}");
            }
        }
    }

    Ok(())
}
