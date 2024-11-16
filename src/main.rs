use teloxide::prelude::*;
use teloxide::types::{ChatId, ChatKind, InlineKeyboardButton, InlineKeyboardMarkup};
use dotenv::dotenv;
use teloxide::utils::command::BotCommands;
use std::env;
use url::Url;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Invite to YC group")]
    Invite,
}

fn allow_invite() -> bool {
    true
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let bot = Bot::from_env();
    let group_id = env::var("TELEGRAM_GROUP_ID").expect("TELEGRAM_GROUP_ID not set");
    let group_id = group_id.parse::<i64>().unwrap();

    Command::repl(bot, move |bot: Bot, msg: Message, cmd: Command| async move {
        match cmd {
            Command::Invite => {
                if let ChatKind::Private(_) = msg.chat.kind {
                    if msg.from().is_some() && allow_invite() {
                        match bot.create_chat_invite_link(ChatId(group_id))
                            .member_limit(1)
                            .await
                        {
                            Ok(invite_link) => {
                                match Url::parse(&invite_link.invite_link) {
                                    Ok(url) => {
                                        let keyboard = InlineKeyboardMarkup::new(vec![vec![
                                            InlineKeyboardButton::url("Join YC Group", url)
                                        ]]);

                                        bot.send_message(msg.chat.id, "Click the button below to join the YC group:")
                                            .reply_markup(keyboard)
                                            .await?;
                                    },
                                    Err(_) => {
                                        bot.send_message(msg.chat.id, "Sorry, there was an error processing the invite link.")
                                            .await?;
                                    }
                                }
                            }
                            Err(err) => {
                                log::error!("Failed to create invite link: {:?}", err);
                                bot.send_message(msg.chat.id, "Sorry, I couldn't create an invite link. Please try again later.")
                                    .await?;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    })
    .await;
}
