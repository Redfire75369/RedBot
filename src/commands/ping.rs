use serenity::{
    framework::standard::{
        CommandResult,
        macros::command,
    },
    model::channel::Message,
    prelude::Context,
};

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Ping! ".to_owned() + (chrono::offset::Utc::now() - msg.timestamp).num_milliseconds().to_string().as_str() + "ms").await?;

    Ok(())
}