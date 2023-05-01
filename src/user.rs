//! User Client
//! 
//! Makes requests for different information about users
//! 
//! ## Examples:
//! For all of the examples you will need to replace the path with the path to 
//! your own .env file and add the key "USER_KEY" with you revolt authentication token
//! 
//! This fetchs all the data about the user and prints it out
//! 
//! ```rust
//! use volt::user::UserClient;
//! use dotenv::from_path;
//! use std::env::var;
//! 
//! let path = r#"C:\Users\conno\Downloads\volt\tests\.env"#;
//! from_path(path).unwrap();
//! let user: String = var("USER_KEY").unwrap();
//! let mut client = UserClient::new(user);
//! println!("{:?}", client.fetch_self().run());
//! ```
//!
//! This returns just the id of the user as string
//! 
//! ```rust
//! use volt::user::UserClient;
//! use dotenv::from_path;
//! use std::env::var;
//! 
//! let path = r#"C:\Users\conno\Downloads\volt\tests\.env"#;
//! from_path(path).unwrap();
//! let user: String = var("USER_KEY").unwrap();
//! let mut client = UserClient::new(user);
//! println!("{:?}", client.fetch_self().run().id());
//! ```
//! 

use crate::core::user_structs::{User, UserFlags, UserProfile, Image};
use crate::core::Cache;
use crate::core::TokenBucket;

use reqwest::{Client as ReqwestClient};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Serialize, Deserialize};
use tokio::time;

use std::time::Duration;
use std::sync::{Arc, Mutex};

/// The client to acsses user information
/// 
/// ## Returns:
/// 
/// The desired information as the inbuild `UserResult` enum type
/// 
/// ### Semantic Notes:
/// 
/// To correctly run without errors you must call UserClient::new() with your authentication token as the paramenter, then any of the funcions other then `.run()` last you must call `.run()`
/// 
/// **What will go wrong:**
/// 
/// - Only `::new()` will print out the parameters passed to the enum without refreshing it 
/// - Taking out `.run()` will again only print the parameters passed to the enum but it will refresh it to defult empty values
/// - Calling `.run() after `::new()` will panic! with an unwrap error
/// - Excluding `::new()` wont let you run due to missing parameters (auth)

#[derive(Debug, Clone)]
pub struct UserClient {
    auth: String, 
    cache: Cache,
    url: String,
    bucket: Arc<Mutex<TokenBucket>>,
    user: String,
    json: String,
    data: Data,
    get: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub(crate) enum Data {
    Users(),
    UsersFlags(),
    UsersProfile(),
    String(),
}

/// The enum for data returning
/// 
/// The variations of this enum are the types that are returned after .run() is called
/// 
/// **Note!:**
/// 
/// If any of the methods that impliment this enum are called the type will change
 
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum UserResult {
    User(User),
    UserFlags(UserFlags),
    UserProfile(UserProfile),
    String(String),
}

impl UserClient{

    /// Creates a new UserClient object
    /// 
    /// What this function does is initialize all of the values for the client to use
    /// 
    /// This function must be directly called after UserClient (ex: UserClient::new(auth))
    /// 
    /// ## Parameters
    /// 
    /// `auth: String` - The revolt authentication token of the user
    
    pub fn new(auth: String) -> UserClient {
        UserClient {
            auth, 
            cache: Cache::new(),
            url: String::new(),
            bucket: Arc::new(Mutex::new(TokenBucket::new(Duration::new(1, 0)))),
            user: String::new(),
            json: String::new(),
            data: Data::Users(),
            get: true,
        }
    }

    /// Fetchs the data of the currently authenticated user
    /// 
    /// Sets the UserResult enum to UserResult::User
    
    pub fn fetch_self(&mut self) -> &mut Self{
        self.data = Data::Users();
        self.url ="https://api.revolt.chat/users/@me".to_string();
        self
    } 

    /// Fetchs a user's data
    /// 
    /// Sets the UserResult enum to UserResult::User
    /// 
    /// ## Parameters:
    /// 
    /// `userid: &str` - The userid of the user you want data for on revolt
    
    pub fn fetch_user(&mut self, userid: &str) -> &mut Self{
        self.data = Data::Users();
        self.url = format!("https://api.revolt.chat/users/{}", userid);
        self.user = userid.to_string();
        self
    }

    /// Edits a users details
    /// 
    /// ## Parameters:
    /// 
    /// - `userid: &str` - The userid of your user
    /// - `changes: &str` - A &str repersentation of the changes you want to make to your user

    pub fn edit_user(&mut self, userid: &str, changes: &str) -> &mut Self{
        self.get = false;
        self.url = format!("https://api.revolt.chat/users/{}", userid);
        self.json = changes.to_string();
        self
    }

    /// Fetchs the flags of a user
    /// 
    /// Flags are information about the users account status on revolt
    /// 
    /// - 1: Suspended
    /// - 2: Deleted
    /// - 4: Banned
    /// 
    /// Sets the UserResult enum to UserResult::UsersFlags
    /// 
    /// ## Parameters:
    /// 
    /// `userid: &str` - The userid of the user you want data for on revolt

    pub fn fetch_user_flags(&mut self, userid: &str) -> &mut Self{
        self.data = Data::UsersFlags();
        self.url = format!("https://api.revolt.chat/users/{}/flags", userid);
        self.user = userid.to_string();
        self
    }

    /// Changes a users username
    /// 
    /// ## Parameters:
    /// 
    /// - `username: &str` - The username you want to change to
    /// - `pass: &str` - Your revolt password
    
    pub fn change_username(&mut self, username: &str, pass: &str) -> &mut Self{
        self.get = false; 
        self.url ="https://api.revolt.chat/users/@me/username".to_string();
        self.json = format!("{{\"username\": {}, \"password\": {},}}", username, pass);
        self
    }

    /// Returns the binary data of a persons avatar
    /// 
    /// This function in pair with 'avatar' in UserResult will return a string that you will have to parse to form an image
    /// 
    /// ## Parameters:
    /// 
    /// `userid: &str` - The id of the user that you want the avatar of
    
    pub fn fetch_default_avatar(&mut self, userid: &str) -> &mut Self{
        self.data = Data::String();
        self.url = format!("https://api.revolt.chat/users/{}/default_avatar", userid);
        self.user = userid.to_string();
        self    
    }

    /// fetchs the profile data of a user
    /// 
    /// ## Parameters: 
    /// 
    /// `userid: &str` - The id of the user that you want to fetch the profile of
    
    pub fn fetch_user_profile(&mut self, userid: &str) -> &mut Self{
        self.data = Data::UsersProfile();
        self.url = format!("https://api.revolt.chat/users/{}/profile", userid);
        self.user = userid.to_string();
        self
    }

    /// Runs the client
    /// 
    /// This function makes the request, applies rate limits, and caches/retrives values from the cache
    /// 
    /// **Note:**
    /// This funtion is required to come immediatly after any one of the other funtions
    /// 
    /// ## Retruns:
    /// 
    /// This function returns a UserResult enum that can be printed out 
    /// or used to get more specific data from the json data provided

    #[tokio::main]
    pub async fn run(&mut self) -> UserResult{
        // Checks the cache for the item
        if self.cache.get(&self.user).is_some(){
            let text = self.cache.get(&self.user).unwrap();
            match text.1.to_lowercase().as_str(){
                "user" => {
                    if self.data == Data::Users(){
                        let parsed_value: User = serde_json::from_str(&text.0).unwrap();
                        return UserResult::User(parsed_value);
                    }
                }
                "userflags" => {
                    if self.data == Data::UsersFlags(){
                        let parsed_value: UserFlags = serde_json::from_str(&text.0).unwrap();
                        return UserResult::UserFlags(parsed_value);
                    }
                }
                "userprofile" => {
                    if self.data == Data::UsersProfile(){
                        let parsed_value: UserProfile = serde_json::from_str(&text.0).unwrap();
                        return UserResult::UserProfile(parsed_value);
                    }
                }
                _ => panic!("Something went very wrong"),
            }
        }

        let bucket = self.bucket.clone();
        let mut bucket = bucket.lock().unwrap();

        let mut header: HeaderMap = HeaderMap::new();
        header.insert("x-session-token", HeaderValue::from_str(&self.auth).unwrap()); 

        while !bucket.try_acquire() {
            time::sleep(Duration::from_millis(100)).await;
        }

        let client = if self.get{
            ReqwestClient::new().get(&self.url).headers(header).send().await.unwrap()
        }
        else{
            ReqwestClient::new().patch(&self.url).headers(header).json(&self.json).send().await.unwrap()
        }; 

        // Might be a problem in the future
        let body = client.text().await.unwrap();

        return self.return_data(body)
    }

    // Literally returns the data for the main 'run' function 
    fn return_data(&mut self, body: String) -> UserResult{
        match self.data{
            Data::Users() => {
                let value: User = serde_json::from_str(&body).unwrap();
                self.cache.add(value.id.clone(), (body, "user".to_string()));
                return UserResult::User(value);
            },
            Data::UsersFlags() => {
                let value: UserFlags = serde_json::from_str(&body).unwrap();
                self.cache.add(self.user.clone(), (body, "userflags".to_string()));
                return UserResult::UserFlags(value);
            },
            Data::UsersProfile() => {
                let value: UserProfile = serde_json::from_str(&body).unwrap();
                self.cache.add(self.user.clone(), (body, "userprofile".to_string()));
                return UserResult::UserProfile(value);
            },
            Data::String() => {
                return UserResult::String(body);
            }
        }
    }
}

impl UserResult {

    /// Returns a users id
    /// 
    /// ## Returns:
    /// 
    /// The users id as a string
    /// 
    /// ## Panics!:
    /// 
    /// This funtion will panic if the enum variant UserResult::User has not been called
    
    pub fn id(&self) -> String {
        if let UserResult::User(user_data) = self {
            user_data.id.clone()
        } else{
            panic!("Cannot get id from: {:?}", self);
        }
    }

    /// Returns a users name
    /// 
    /// ## Returns:
    /// 
    /// The users name as a string
    /// 
    /// ## Panics!:
    /// 
    /// This funtion will panic if the enum variant UserResult::User has not been called

    pub fn name(&self) -> String {
        if let UserResult::User(user_data) = self {
            user_data.username.clone()
        } else{
            panic!("Cannot get name from: {:?}", self);
        }
    }

    /// Returns a users avatar
    /// 
    /// ## Returns:
    /// 
    /// The users avatar data in the form of a struct called image that can be parsed down to get the specific values
    /// 
    /// ## Panics!:
    /// 
    /// This funtion will panic if the enum variant UserResult::User has not been called

    pub fn avatar(&self) -> Image{
        if let UserResult::User(user_data) = self {
            match user_data.avatar.clone().is_some() {
                true => user_data.avatar.clone().unwrap(),
                false => panic!("Avatar not found!"),
            }
        } else {
            panic!("Cannot get background from: {:?}", self);
        }
    }

    /// Returns the badges a users has
    /// 
    /// Badges can be earned on Revolt for different things (will explain below)
    /// 
    /// The API shows badges as the sum of the values of all the badges
    /// 
    /// Badge Values:
    /// 
    /// - 1: Developer (Develops Revolt)
    /// - 2: Translator (Translated Revolt into another language)
    /// - 4: Supporter (Supported Revolt with a donation)
    /// - 8: Responsibly Disclosed Bug(s) (Disclosed a bug responibly)
    /// - 16: Founder (Founded Revolt)
    /// - 32: Platform Moderation (Moderates Revolt)
    /// - 64: Active Supporter (Activlty supports Revolt)
    /// - 128: Paw (🦊🦝)
    /// - 256: Early Adopter (One of the first 1000 users)
    /// - 512: ReservedRelevantJokeBadge1 (Amogus)
    /// - 1024 ReservedRelevantJokeBadge2 (Amogus Troll Face)
    /// 
    /// ## Returns
    /// 
    /// A `vec<&str>` of the badges the user has earned in order from highest value to lowest value
    /// 
    /// ## Panics!:
    /// 
    /// This funtion will panic if the enum variant UserResult::User has not been called

    pub fn badges(&self) -> Vec<&str> {
        // Credit to FatalErrorMogus for the inital design
        if let UserResult::User(user_data) = self {
            // Badges constant
            let badges: Vec<(&i32, &str)> = vec![
                (&1024, "ReservedRelevantJokeBadge1"),
                (&512, "ReservedRelevantJokeBadge1"),
                (&256, "EarlyAdopter"),
                (&128, "Paw"),
                (&64, "ActiveSupporter"),
                (&32, "PlatformModeration"),
                (&16, "Founder"),
                (&8, "ResponsibleDisclosure"),
                (&4, "Supporter"),
                (&2, "Translator"),
                (&1, "Developer"),
            ];
            let mut badge = user_data.badges.unwrap();
            let mut final_badges: Vec<&str> = Vec::new();
            for i in badges{
                if badge - i.0 >= 0 {
                    final_badges.push(i.1);
                    badge -= i.0;
                };
            }
            final_badges
        } else {
           panic!("Cannot get badges from: {:?}", self) ;
        }
    }

    /// Returns the users flags
    /// 
    /// Flags are information about the users account status on revolt
    /// 
    /// - 1: Suspended
    /// - 2: Deleted
    /// - 4: Banned
    /// 
    /// ## Returns:
    /// 
    /// The users flags as a usize
    /// 
    /// ## Panics!:
    /// 
    /// This function will panic if fetch_user_flags() was not called
    
    pub fn flags(&self) -> usize {
        if let UserResult::UserFlags(user_data) = self {
            user_data.flags.clone()
        } else {
           panic!("Cannot get flags from: {:?}", self) ;
        }
    }

    /// Returns a users background image
    /// 
    /// ## Returns:
    /// 
    /// The users background data in the form of a struct called image that can be parsed down to get the specific values
    /// 
    /// ## Panics!:
    /// 
    /// This funtion will panic if the enum variant UserResult::UserProfile has not been called

    pub fn background(&self) -> Image{
        if let UserResult::UserProfile(user_data) = self {
            user_data.background.clone()
        } else {
            panic!("Cannot get background from: {:?}", self);
        }
    }

    /// Returns the status text of a user
    /// 
    /// ## Returns:
    /// 
    /// The status text of a user as a string
    /// 
    /// **Note:**
    /// 
    /// This function takes both UserResult::UserProfile and UserResult::User
    /// 
    /// When UserResult::User is called the value may be "None" 
    /// if there was no status or there was and error parsing the status
    /// 
    /// ## Panics!:
    /// 
    /// This function will panic if fetch_user_flags was called before
    
    pub fn status(&self) -> String {
        if let UserResult::UserProfile(user_data) = self {
            user_data.content.clone()
        }
        else if let UserResult::User(user_data) = self {
            match user_data.status.clone().is_some(){
                true => user_data.status.clone().unwrap().text.unwrap_or("None".to_string()),
                false => "None".to_string(),
            }
        } else {
            panic!("Cannot get status from: {:?} \nMake sure you are not calling: \"fetch_user_flags\"", self);
        }
    }
}