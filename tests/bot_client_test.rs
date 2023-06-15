use volt::bot::BotClient;

use dotenv::from_path;

use std::env::var;
use std::time::Instant;

#[test]
fn run() -> Result<(), Box<dyn std::error::Error>>{
    let start = Instant::now();

    let path = r#"C:\Users\conno\Downloads\volt\tests\.env"#;
    from_path(path).unwrap();

    let user: String = var("USER_KEY").unwrap();

    let mut client = BotClient::new(user);
    let test = client.fetch_public_bot("01FVB28WQ9JHMWK8K7RD0F0VCW").get();
    let bah = client.fetch_public_bot("01GVK8WRSG2J876G90EFDPEQTX").get();
    let a = client.fetch_public_bot("01FVB28WQ9JHMWK8K7RD0F0VCW").get();
    //let test = client.fetch_owned_bot("01GVN60A2J1WCRDC3XNZCSRD9V").get();
    //println!("{:?}", test);
    println!("{:?} \n {:?} \n {:?}", test, bah, a);

    let end = Instant::now();
    println!("{:?}", end - start);
    Ok(())
}