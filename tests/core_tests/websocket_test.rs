use volt::websocket;

extern crate dotenv;
use dotenv::from_path;

use std::env::var;

#[test]
fn run(){
    let path = r#"C:\Users\conno\Downloads\volt\tests\.env"#;
    from_path(path).unwrap();

    let bot: String = var("BOT_KEY").unwrap();

    websocket(bot)
}