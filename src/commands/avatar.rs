use crate::utils::user::user_from_str;

use chrono::prelude::{DateTime, Utc};
use serenity::{
    framework::standard::{
        CommandResult,
        macros::command,
    },
    model::channel::Message,
    prelude::Context,
    utils::Colour,
};
use std::time::SystemTime;

#[command]
async fn avatar(ctx: &Context, msg: &Message) -> CommandResult {
    let words: Vec<&str> = msg.content.split_whitespace().collect();
    let prefix = words[0].starts_with("!");
    let index = if prefix { 1 } else { 2 };
    let user_id: (u64, String) = if words.len() > index {
        if prefix {
            user_from_str(&String::from(words[1]), &msg, &ctx).await
        } else {
            user_from_str(&String::from(words[2]), &msg, &ctx).await
        }
    } else {
        (msg.author.id.0, String::from(""))
    };

    let user_payload =  ctx.http.get_user(user_id.0).await;
    msg.channel_id.send_message(&ctx.http, |message| {
        message.embed(|embed| {
            let time: DateTime<Utc> = SystemTime::now().into();

            if user_payload.is_ok() {
                let user = user_payload.unwrap();
                embed.title("Avatar")
                    .image(if user.avatar_url().is_some() {
                        user.avatar_url().unwrap()
                    } else {
                        user.default_avatar_url()
                    })
                    .colour(Colour::ROHRKATZE_BLUE)
                    .timestamp(&time)
            } else {
                if user_id.1 == "" {
                    embed.title("Avatar")
                        .description("Invalid User ID: ".to_owned() + &user_id.0.to_string().as_str())
                        .colour(Colour::RED)
                        .timestamp(&time)
                } else {
                    embed.title("Avatar")
                        .description(user_id.1)
                        .colour(Colour::RED)
                        .timestamp(&time)
                }
            }
        });
        message
    }).await?;

    Ok(())
}