use teloxide::prelude::*;

use parsers::calc;

mod parsers;

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting calc bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |cx| async move {
        log::info!("user {:?}; message {:?};", cx.update.chat.id, cx.update.text());
        match cx.update.text_owned() {
            None => cx.answer_str("Tell me expression").await?,
            Some(ans) => {
                let ans = calc(ans.as_str());
                log::info!("user {:?}; ans {:?}", cx.update.chat.id, ans);
                cx.answer_str(ans).await?
            }
        };
        ResponseResult::<()>::Ok(())
    })
        .await;
}
