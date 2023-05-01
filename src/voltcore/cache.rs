use std::collections::HashMap;

use serde::{Serialize, Deserialize};

/// The Hash Map for the cache
/// 
/// This Struct allows you to work with the hashmap to add, and get values from the cache

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cache {
    cache: HashMap<String, (String, String)>,
}

impl Cache {
    /// Creates a new cache
    pub fn new() -> Self {
        Self{
            cache: HashMap::with_capacity(12)
        }
    }

    /// Adds a value to the cache
    /// 
    /// This also check the length of the cache to keep it under 9
    /// If the key is already present in the cache it will add nothing and return nothing
    /// 
    /// # Arguments:
    /// - `key: String` - The key that you want the value to be assosiated with, this is the value that you can "lookup" to get the other value
    /// - `value: (String, String)` - The values of the cache (body, type)
    
    pub fn add(&mut self, key: String, value: (String, String)) {
        if self.cache.len() >= 9 {
            let last_key = self.cache.keys().last().unwrap().to_string();
            self.cache.remove(&last_key);
        }
        
        if self.cache.contains_key(&key) {
            return;
        }

        self.cache.insert(key, value);
    }

    /// Retrives the value of a key from the cache
    /// 
    /// If the cache does not contain the key it will return None
    /// 
    /// # Arguments:
    /// - `key: &str` - The key you want to get the value of
    
    pub fn get(&self, key: &str) -> Option<&(String, String)> {
        if !self.cache.contains_key(key) {
            return None;
        }

        self.cache.get(key)
    }

}