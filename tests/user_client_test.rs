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
    let test = client.fetch_user("01EXAF3KX65608AJ4NG27YG1HM").run();
    println!("{:?}", test.badges());

    let end = Instant::now();
    println!("{:?}", end - start);
    Ok(())
}