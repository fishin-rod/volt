//! The core functionality of the crate
//! 
//! Without theses functions many of the crates features would not be able to work
//! 
//! Most of these functions are limited to inside crate use only but
//! some are enabled to be used in your own applications even thougn 
//! it is not recomened

pub(crate) mod cache;

pub mod imageserver;

pub(crate) mod ratelimits;

pub(crate) mod structs;