use volt::user::UserClient;

use dotenv::from_path;

use std::env::var;
use std::time::Instant;

#[test]
fn run() -> Result<(), Box<dyn std::error::Error>>{
    let start = Instant::now();

    let path = r#"C:\Users\conno\Downloads\volt\tests\.env"#;
    from_path(path).unwrap();

    let user: String = var("USER_KEY").unwrap();

    let mut client = UserClient::new(user);
    //let test = client.edit_user("01GVKXN2XQWCP61CVYACNSN6CW", r#"{"status":{"text":"Hey", "presence": "Online"}}"#).patch();
    //let test = client.friend_request("0chroma#4432").post();
    let test = client.fetch_self().get();
    println!("{:?}", test);

    let end = Instant::now();
    println!("{:?}", end - start);
    Ok(())
}