

//! An abstraction library to easily fetch/post/modify/delete e621.net content
//! 
//! FluffyF (or fluffyf) allows for very simple and painless abstracted API
//! requests to the mature furry imageboard e621.net and e926.net
//! 
//! 
//! 
//! 
//! 
//! 
//! 

extern crate reqwest;
extern crate bytes;
extern crate tokio;
extern crate serde;
extern crate log;
extern crate time;
extern crate async_trait;
extern crate futures;

pub mod api;
pub mod connect;
pub mod utils;
