
use teloxide::{prelude::*, types::InputFile};


#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    // token from env file
    dotenv::dotenv().ok();

    let token = std::env::var("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN is not set in .env file");


    let bot = Bot::new(token);

    let bot_name = bot.get_me().await.unwrap().user.username.unwrap();
    println!("Starting {} bot...", bot_name);
    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        
        //print income message

        println!("{:?}", msg.text().unwrap());

        let text = msg.text().unwrap();
        let splitted_text = text.split(" ").collect::<Vec<&str>>();
        match splitted_text[0] {
            "/start" => {
                bot.send_message(msg.chat.id, "Hello! I'm dice bot. I can throw dice for you. Just type /throw_dice").await?;
            },
            "/throw_dice" => {
                bot.send_message(msg.chat.id, "Throwing dice...").await?;
                bot.send_dice(msg.chat.id).await?;
            },
            "/download" => {
                bot.send_message(msg.chat.id, "Downloading...").await?;
                //bot.send_document(msg.chat.id, InputFile::url( Url::parse("https://www.rust-lang.org/logos/rust-logo-512x512.png")));
            },
            "chatgpt" => {
                bot.send_message(msg.chat.id, "Chatting...").await?;
                bot.send_message(msg.chat.id, "https://www.chatbots.org/ai_zone/viewthread/376/").await?;
            },
            _ => {
                bot.send_message(msg.chat.id, "I don't understand you. Type /throw_dice").await?;
            }
        }

        // bot.send_message(msg.chat.id, "Throwing dice...").await?;
        // bot.send_dice(msg.chat.id).await?;
        Ok(())
    })
    .await;
}
