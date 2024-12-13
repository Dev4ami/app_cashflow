mod models;
mod api_telegram;

use teloxide::{prelude::*, utils::command::BotCommands};
use dotenv::dotenv;
use std::env;




#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN harus diatur di file .env");
    pretty_env_logger::init();
    log::info!("Starting command bot...");
    println!("server started");
    let bot = Bot::new(token);
    Command::repl(bot, answer).await;

}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    Start,
    Help,
    #[command(parse_with = "split")]
    Add { product: String, amount: i32 },
    Delete(i32),
    Get(i32),
    Report(i32),
}


async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => {
            let start = api_telegram::cmd_start().await;
            bot.send_message(msg.chat.id, start).await?
        }
        Command::Help => {
            let help = help().await;
            bot.send_message(msg.chat.id, help).await?
        }
        Command::Add { product, amount } => {
            let add_data = api_telegram::add_expenses(&product, amount, &msg.chat.id).await;
            let last = api_telegram::get_expanses_by_last(1).await;
            bot.send_message(msg.chat.id, format!("{}\n{}", add_data, last))
                .await?
        }
        Command::Delete (id) => {
            let data = api_telegram::get_expanses_by_id(id).await;
            let delete_data = api_telegram::delete_expenses(id).await;
            bot.send_message(msg.chat.id, format!("{}\n{}", data, delete_data))
                .await?
        }
        Command::Get (amount) => {
            let get_data = api_telegram::get_expanses_by_last(amount).await;
            bot.send_message(msg.chat.id, get_data)
                .await?
        }
        Command::Report (amount) => {
            let get_data = api_telegram::get_expanses_by_day(amount).await;
            bot.send_message(msg.chat.id, get_data)
                .await?
        }
    };

    Ok(())
}


pub async fn help() -> String {
    format!("
This Bot For Private Management Expense

/start  ==> Cek Server
/help  ==> Bantuan
/add  ==> /add {{product_name}} {{price}}
/delete  ==> /delete {{id}}
/get  ==> /get {{last}}
/report  ==> /report {{total_day}}")

}

