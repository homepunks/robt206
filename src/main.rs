use robt206::tg;
use robt206::audio;
use std::env;
use tokio::fs;

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
            let Some(msg) = update.message.as_ref() else { continue };
            let Some(file_id) = msg.audio_file_id() else { continue };
            let chat_id = msg.chat.id;

            let result = async {
                let oga_in = client.extract_bytes(file_id).await?;
                let pcm = audio::decode_voice(&oga_in)?;
                let chipped = audio::effect::chipmunk(&pcm, 1.67);
                let oga_out = audio::encode_voice(&chipped)?;
                client.send_voice(chat_id, oga_out).await?;
                anyhow::Ok(())
            }
            .await;

            if let Err(e) = result {
                eprintln!("INFO: skipping {} due to error: {:#}", update.update_id, e);
            }
        }
    }

    Ok(())
}
