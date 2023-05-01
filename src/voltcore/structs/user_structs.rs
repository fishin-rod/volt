use serde::{Serialize, Deserialize};

// ******************************
// The main structs for a user!!!
// ******************************

/// Represents a user
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct User{
    #[serde(rename = "_id")]
    pub(crate) id: String,
    pub(crate) username: String,
    pub(crate) avatar: Option<Image>,
    pub(crate) badges: Option<i32>,
    pub(crate) status: Option<Status>,
    pub(crate) relationship: Option<String>,
    pub(crate) online: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Image{
    #[serde(rename = "_id")]
    pub(crate) id: String,
    pub(crate) tag: String,
    pub(crate) filename: String,
    pub(crate) metadata: Metadata,
    pub(crate) content_type: String,
    pub(crate) size: usize,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub(crate) struct Metadata{
    pub(crate) height: usize,
    pub(crate) width: usize,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub(crate) struct Status{
    pub(crate) text: Option<String>,
    pub(crate) presence: String,
}

// **********************
// Structs for user flags
// **********************

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct UserFlags{
    pub(crate) flags: usize,
}

// ****************************
// Structs for the user profile
// ****************************

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct UserProfile{
    pub(crate) content: String,
    pub(crate) background: Image,
}
