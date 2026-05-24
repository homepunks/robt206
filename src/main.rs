use robt206::tg::{self, Message};
use tokio::fs;
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
        let updates = client.get_updates(offset, 25).await?;
        for update in updates {
            offset = update.update_id + 1;
            if let Some(file_id) = &update.message.as_ref().and_then(Message::audio_file_id) {
                match client.extract_bytes(file_id).await {
                    Ok(audio_raw) => {
                        let cache = format!("voice_{}.oga", update.update_id);
                        fs::write(&cache, &audio_raw).await?;
                        println!("Saved {} ({} bytes)", cache, audio_raw.len());
                    },
                    Err(e) => eprintln!("Skipping {} due to error: {:#}", update.update_id, e),
                }
            }
        }
    }

    Ok(())
}
