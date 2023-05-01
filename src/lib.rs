//! # Volt
//! 
//! An API wrapper for Revolt written in rust!
//! 
//! Volts goal is to make working with revolts API simple and easy
//! 
//! ### What volt offeres:
//! - An easy way to make requests
//! - Pre Serilization of the data
//! - Cacheing to improve performance of large requests
//! - Inbuilt rate limits (25 requests per second)
//! - Error handeling 
//! 

#[cfg(feature="core")]
pub(crate) mod voltcore;

/// The core functionality of the crate
/// 
/// Without theses functions many of the crates features would not be able to work
/// 
/// Most of these functions are limited to inside crate use only but
/// some are enabled to be used in your own applications even thougn 
/// it is not recomened
#[cfg(feature="core")]
pub mod core{

    #[cfg(feature="core")]
    pub(crate) use crate::voltcore::cache::Cache;

    #[cfg(feature="core")]
    pub use crate::voltcore::imageserver::ImageServer;

    #[cfg(feature="core")]
    pub(crate) use crate::voltcore::ratelimits::TokenBucket;

    #[cfg(feature="core")]
    pub(crate) use crate::voltcore::structs::{user_structs};

}

#[cfg(feature="client")]
pub mod user;

#[cfg(feature="client")]
pub(crate) mod client;

#[cfg(feature="client")]
pub mod clients{

    #[cfg(feature="client")]
    pub use crate::client::websocket::socket as websocket;

}