use tungstenite::{connect, Message};
use url::Url;

use std::time::Duration;
use std::thread;

#[tokio::main]
pub async fn socket(token: String) {
    let url = Url::parse("wss://ws.revolt.chat?version=1&format=json").unwrap();
    let (mut socket, _) = connect(url).expect("Failed to connect");
    
    let auth_request = format!("{{\"type\": \"Authenticate\", \"token\":\"{}\"}}", token);
    socket.write_message(Message::Text(auth_request)).unwrap();

    let message = r#"{"type": "Ping", "data": 0}"#;

    loop {
        thread::sleep(Duration::from_secs(20));

        socket.write_message(Message::Text(message.into())).unwrap();
    }
}
