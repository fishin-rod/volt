use volt::core::ImageServer;

extern crate dotenv;
use dotenv::from_path;

use std::env::var;

#[test]
fn run(){
    let path = r#"C:\Users\conno\Downloads\volt\tests\.env"#;
    from_path(path).unwrap();

    let user: String = var("USER_KEY").unwrap();

    // Credit to pan for the image
    let server = ImageServer::new(user, r#"C:\Users\conno\Downloads\volt\tests\images\fox_by_pan.png"#).send();
    println!("{}", server);
}