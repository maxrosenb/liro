use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    trace!("help() called");
    let message = "Hi, I'm liro!\n\
                   I help automate role assignments based on your rating. To get started, please \
                   link your lichess account to your Discord user by saying `ohnomy account` (or \
                   `/link`)\n\
                   After that, you can ask me to retrieve your ratings and update your Discord \
                   roles by saying `ohnomy rating` (or `/rating`)\n\
                   If you want me to forget everything I know about you, just say `ohnomy gdpr` \
                   (or `/unlink`)";
    msg.channel_id.say(&ctx.http, message).await?;

    Ok(())
}
