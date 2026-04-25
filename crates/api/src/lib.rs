//! # luduvo-rs
//!
//! *luduvo-rs* is a rust library for interacting with the [luduvo](luduvo.com) api.
//!
//! ## example
//!
//! ```no_run
//! use luduvo_rs::users::profile::Client;
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut client = Client::new(None);
//!     let id = "1".to_string();
//!
//!     match client.get_user(id.clone()).await {
//!         Ok(profile) => {
//!             println!("profile for id `{id}`: {:#?}", profile);
//!         }
//!
//!         Err(e) => {
//!             eprintln!(
//!                 "error caught while attempting to get profile for id `{id}`: '{:#?}'",
//!                 e
//!             );
//!         }
//!     }
//! }
//! ```

#![allow(unused)]

#[cfg(feature = "prelude")]
pub mod prelude;

#[cfg(feature = "users")]
pub mod users;

#[cfg(feature = "places")]
pub mod places;
