use std::fs::File;
use std::io::Read;

use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::multipart::{Form, Part};
use serde::{Serialize, Deserialize};

/// Sends images to revolts image server, 

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageServer {
    auth: String,
    path: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Image{
    id: String,
}

impl ImageServer{
    /// Creates a new image server instance
    /// 
    /// # Parameters:
    /// 
    /// - `auth: String` - Your revolt key
    /// - `path: &str` - The path to the image you want to send

    pub fn new(auth: String, path: &str) -> Self{
        Self{
            auth,
            path: path.to_string(),
        }
    }

    /// Sends the request to autumn 
    /// 
    /// # Returns
    /// 
    /// A string with the autumn id of the image
    
    #[tokio::main]
    pub async fn send(&self) -> String{
        let mut headers = HeaderMap::new();
        headers.insert("x-session-token", HeaderValue::from_str(&self.auth).unwrap());

        let mut file = File::open(&self.path).unwrap();
        let mut image_data = Vec::new();
        file.read_to_end(&mut image_data).unwrap();

        let form = Form::new()
        .part("image", Part::bytes(image_data).file_name("image.png"));

        let url= "https://autumn.revolt.chat/attachments";
        let client = Client::new().post(url).headers(headers).multipart(form).send().await.unwrap().text().await.unwrap();

        let image: Image = serde_json::from_str(&client).unwrap();
        image.id
    }
}