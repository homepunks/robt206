use robt206::tg;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let token = env::var("TG_BOT_TOKEN")?;
    let client = tg::Client::new(token);

    println!("INFO: starting polling...");
    let mut offset: i64 = 0;
    loop {
        let updates = client.get_updates(offset, 25).await?;
        for update in updates {
            offset = update.update_id + 1;
            println!("{update:#?}");
        }
    }
    Ok(())
}
