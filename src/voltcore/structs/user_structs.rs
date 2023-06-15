use serde::{Serialize, Deserialize};

// ******************************
// The main structs for a user!!!
// ******************************

/// Represents a user
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct User{
    #[serde(rename = "_id")]
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<Image>,
    pub badges: i32,
    pub status: Option<Status>,
    pub relationship: Option<String>,
    pub online: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Image{
    #[serde(rename = "_id")]
    pub id: String,
    pub tag: String,
    pub filename: String,
    pub metadata: Metadata,
    pub content_type: String,
    pub size: usize,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Metadata{
    #[serde(rename="type")]
    pub types: String,
    pub height: usize,
    pub width: usize,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Status{
    pub text: Option<String>,
    pub presence: String,
}

// **********************
// Structs for user flags
// **********************

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct UserFlags{
    pub flags: usize,
}

// ****************************
// Structs for the user profile
// ****************************

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct UserProfile{
    pub content: String,
    pub background: Image,
}

// ************************
// Direct Messsage Channels
// ************************

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct UserDmChannels{
    pub channel_type: String,
    #[serde(rename = "_id")]
    pub channel_id: String,
    pub user: Option<String>,
    pub name: Option<String>,
    pub owner: Option<String>,
    pub recipients: Option<Vec<String>>,
    pub active: Option<bool>,
    pub last_message_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct UserDms{
    pub channel_type: String,
    #[serde(rename = "_id")]
    pub channel_id: String,
    pub recipients: Option<Vec<String>>,
    pub active: Option<bool>,
    pub last_message_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct UserMutuals{
    pub users: Vec<String>,
    pub servers: Vec<String>,
}