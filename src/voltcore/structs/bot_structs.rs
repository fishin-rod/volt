use serde::{Deserialize, Serialize};

//************
// Owned bots
//***********

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bot {
    pub bot: OwnedBot,
    pub user: User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OwnedBot {
    #[serde(rename = "_id")]
    pub id: String,
    pub owner: String,
    pub token: String,
    pub public: bool,   
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: Avatar,
    pub bot: Bot2,
    pub relationship: String,
    pub online: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Avatar {
    #[serde(rename = "_id")]
    pub id: String,
    pub tag: String,
    pub filename: String,
    pub metadata: Metadata,
    pub content_type: String,
    pub size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "type")]
    pub type_field: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bot2 {
    pub owner: String,
}

//*************** 
// All owned bots
//*************** 

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bots {
    pub bots: Vec<OwnedBot>,
    pub users: Vec<Users>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Users {
    #[serde(rename = "_id")]
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: Avatar,
    pub relations: Vec<String>,
    pub status: Status,
    pub profile: Profile,
    pub bot: BotOwner,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Status {
    pub presence: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    pub content: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BotOwner {
    pub owner: String,
}

//************
// Public Bots 
//************ 

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicBot {
    #[serde(rename="_id")]
    id: String,
    username: String,
    avatar: String,
    description: String,
}

