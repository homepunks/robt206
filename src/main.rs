use robt206::audio;
use robt206::tg::{self, cli};
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
            let Some(msg) = update.message.as_ref() else {
                continue;
            };
            if let Some(txt) = &msg.text {
                let Some(reply) = msg.reply_to_message.as_deref() else {
                    continue;
                };
                let Some(file_id) = reply.audio_file_id() else {
                    continue;
                };
                if let Some(cmd) = cli::detect_cmd(txt) {
                    match cmd {
                        cli::Cli::CHIPMUNK => {
                            let result = async {
                                let oga_in = client.extract_bytes(file_id).await?;
                                let pcm = audio::decode_voice(&oga_in)?;
                                let chipped = audio::effect::chipmunk(&pcm, 1.67);
                                let oga_out = audio::encode_voice(&chipped)?;
                                client.send_voice(msg.chat.id, oga_out).await?;
                                anyhow::Ok(())
                            }
                            .await;

                            let username = msg
                                .from
                                .as_ref()
                                .and_then(|u| u.username.as_deref())
                                .map(|u| format!("@{u}"))
                                .unwrap_or_else(|| "<anonymous>".to_string());
                            println!("INFO: received voice from @{}", username);

                            if let Err(e) = result {
                                eprintln!(
                                    "INFO: skipping {} due to error: {:#}",
                                    update.update_id, e
                                );
                            }
                        }
                    }
                }
            } else {
                continue;
            }
        }
    }

    Ok(())
}
