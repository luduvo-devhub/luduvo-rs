//! # users api
//!
//! this module contains endpoints related to luduvo user data.
//!
//! ## available endpoints
//!
//! - [`profile`] - fetch user profile data *by user id*
//! - [`friends`] - fetch a user's friends list
//! - [`query`] - fetch multiple users profile data *by username*
//!
//! each endpoint provides a dedicated wrapper struct for interacting with the luduvo api.

/// base url for the luduvo users api.
pub const BASE_URL: &str = "https://api.luduvo.com/users";

#[cfg(feature = "friends")]
pub mod friends;

#[cfg(feature = "profile")]
pub mod profile;

#[cfg(feature = "query")]
pub mod query;
