use crate::core::structs::bot_structs::{Bot, Bots, PublicBot};
use crate::core::Cache;
use crate::core::TokenBucket;

use reqwest::{Client as ReqwestClient};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Serialize, Deserialize};
use tokio::time;

use std::time::Duration;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct BotClient{
    auth: String,
    cache: Cache,
    url: String,
    bucket: Arc<Mutex<TokenBucket>>,
    bot: String,
    json: String,
    data: Data,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub(crate) enum Data {
    Bot(),
    PublicBots(),
    Bots(),
    String(),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum BotResult {
    Bot(Bot),
    PublicBot(PublicBot),
    Bots(Bots),
    String(String),
}

impl BotClient {
    
    pub fn new(auth: String) -> BotClient {
        BotClient {
            auth,
            cache: Cache::new(),
            url: String::new(),
            bucket: Arc::new(Mutex::new(TokenBucket::new(10))),
            bot: String::new(),
            json: String::new(),
            data: Data::Bot(),
        }
    }

    /// Post
    pub fn create_bot(&mut self, name: &str) -> &mut Self {
        self.url = "https://api.revolt.chat/bots/create".to_string();
        self.json = format!("{{\"name\":\"{}\"}}", name);
        self
    }

    /// Get for other peoples bots
    pub fn fetch_public_bot(&mut self, target: &str) -> &mut Self {
        self.data = Data::PublicBots();
        self.url = format!("https://api.revolt.chat/bots/{}/invite", target);
        self.bot = target.to_string();
        self
    }

    /// Post
    /// 
    /// bot, then server/group id
    pub fn invite_bot(&mut self, target: &str, server: &str) -> &mut Self {
        self.url = format!("https://api.revolt.chat/bots/{}/invite", target);
        self.json = format!("{{\"group\":\"{}\"}}", server);
        self
    }

    /// Get
    /// 
    /// ONLY YOUR BOTS
    pub fn fetch_owned_bot(&mut self, target: &str) -> &mut Self {
        self.data = Data::Bot();
        self.url = format!("https://api.revolt.chat/bots/{}", target);
        self.bot = target.to_string();
        self
    }

    /// Get
    pub fn fetch_owned_bots(&mut self) -> &mut Self {
        self.data = Data::Bots();
        self.url = "https://api.revolt.chat/bots/@me".to_string();
        self
    }

    /// Del
    pub fn delete_bot(&mut self, target: &str) -> &mut Self {
        self.url = format!("https://api.revolt.chat/bots/{}", target);
        self
    }

    /// patch
    #[cfg(feature="experimental")]
    pub fn edit_bot(&mut self, target: &str, changes: &str) -> &mut Self {
        self.url = format!("https://api.revolt.chat/bots/{}", target);
        self.json = changes.to_string();
        self
    }

    #[tokio::main]
    pub async fn get(&mut self) -> BotResult {
        
        if self.cache.get(&self.bot).is_some() {
            let text = self.cache.get(&self.bot).unwrap();
            match text.1.to_lowercase().as_str(){
                "bot" => {
                    if self.data == Data::Bot() {
                        let parsed_value: Bot = serde_json::from_str(&text.0).unwrap();
                        return BotResult::Bot(parsed_value);
                    }
                },
                "publicbot" => {
                    if self.data == Data::PublicBots() {
                        let parsed_value: PublicBot = serde_json::from_str(&text.0)
                        .expect(&format!("The bot you are searching for {} is not listed as public or the id is inccorect!", self.bot));
                        return BotResult::PublicBot(parsed_value);
                    }
                },
                _ => {
                    panic!("Error retriving item from cache")
                }
            }

        }

        let bucket = self.bucket.clone();
        let mut bucket = bucket.lock().unwrap();

        let mut header: HeaderMap = HeaderMap::new();
        header.insert("x-session-token", HeaderValue::from_str(&self.auth).unwrap()); 

        while !bucket.try_acquire() {
            time::sleep(Duration::from_millis(100)).await;
        }

        let client = ReqwestClient::new().get(&self.url).headers(header).send().await.unwrap();

        // Might be a problem in the future
        let body = client.text().await.unwrap();

        return self.return_data(body)
    }

    fn return_data(&mut self, body: String) -> BotResult{
        match self.data{
            Data::Bot() => {
                let value: Bot = serde_json::from_str(&body).unwrap();
                self.cache.add(self.bot.clone(), (body, "Bot".to_string()));
                return BotResult::Bot(value);
            },
            Data::PublicBots() => {
                let value: PublicBot = serde_json::from_str(&body)
                .expect(&format!("The bot you are searching for {} is not listed as public or the id is inccorect!", self.bot));
                self.cache.add(self.bot.clone(), (body, "PublicBot".to_string()));
                return BotResult::PublicBot(value);
            },
            Data::Bots() => {
                let value: Bots = serde_json::from_str(&body).unwrap();
                return BotResult::Bots(value);
            }
            _ => return BotResult::String(body)
        }
    }
}