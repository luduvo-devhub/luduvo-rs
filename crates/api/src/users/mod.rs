/// base url for the luduvo users api.
pub const BASE_URL: &str = "https://api.luduvo.com/users";

#[cfg(feature = "friends")]
pub mod friends;

#[cfg(feature = "profile")]
pub mod profile;

#[cfg(feature = "query")]
pub mod query;
