use crate::audio;
use crate::tg::cli::{self, Cli};
use crate::tg::{Client, Message, Update};

pub async fn run(client: &Client) -> anyhow::Result<()> {
    println!("INFO: starting polling...");
    let pending = client.get_updates(-1, 0).await?;
    let mut offset = pending.last().map(|u| u.update_id + 1).unwrap_or(0);

    loop {
        for update in client.get_updates(offset, 25).await? {
            offset = update.update_id + 1;
            if let Err(e) = handle_update(client, &update).await {
                eprintln!("INFO: skipping {} due to error: {:#}", update.update_id, e);
            }
        }
    }
}

async fn handle_update(client: &Client, update: &Update) -> anyhow::Result<()> {
    let Some(msg) = update.message.as_ref() else {
        return Ok(());
    };
    let Some(txt) = msg.text.as_deref() else {
        return Ok(());
    };
    let Some(cmd) = cli::detect_cmd(txt) else {
        return Ok(());
    };

    match cmd {
        Cli::CHIPMUNK => handle_chipmunk(client, msg).await,
        Cli::REVERSE  => handle_reverse(client, msg).await,
    }
}

async fn handle_chipmunk(client: &Client, msg: &Message) -> anyhow::Result<()> {
    let Some(reply) = msg.reply_to_message.as_deref() else {
        return Ok(());
    };
    let Some(file_id) = reply.audio_file_id() else {
        return Ok(());
    };

    let oga_in = client.extract_bytes(file_id).await?;
    let pcm = audio::decode_voice(&oga_in)?;
    let chipped = audio::effect::chipmunk(&pcm, 1.67);
    let oga_out = audio::encode_voice(&chipped)?;
    client.send_voice(msg.chat.id, oga_out).await?;

    println!("INFO: chipmunked voice from {}", sender_label(msg));
    Ok(())
}

async fn handle_reverse(client: &Client, msg: &Message) -> anyhow::Result<()> {
    let Some(reply) = msg.reply_to_message.as_deref() else {
        return Ok(());
    };
    let Some(file_id) = reply.audio_file_id() else {
        return Ok(());
    };

    let oga_in = client.extract_bytes(file_id).await?;
    let pcm = audio::decode_voice(&oga_in)?;
    let reversed = audio::effect::reverse(&pcm);
    let oga_out = audio::encode_voice(&reversed)?;
    client.send_voice(msg.chat.id, oga_out).await?;

    println!("INFO: reversed voice from {}", sender_label(msg));
    Ok(())
}

fn sender_label(msg: &Message) -> String {
    msg.from
        .as_ref()
        .and_then(|u| u.username.as_deref())
        .map(|u| format!("@{u}"))
        .unwrap_or_else(|| "<anonymous>".to_string())
}
