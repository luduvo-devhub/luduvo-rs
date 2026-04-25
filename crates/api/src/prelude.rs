//! # prelude
//!
//! this module re-exports commonly used types in luduvo-rs.
//!
//! ## why use the prelude
//!
//! instead of importing individual types/structs like:
//!
//! ```no_run
//! use luduvo_rs::users::profile::{Profile, ProfileWrapper};
//! use luduvo_rs::users::friends::{Friends, FriendsWrapper};
//! ```
//!
//! you can simply do:
//!
//! ```no_run
//! use luduvo_rs::prelude::*;
//! ```
//!
//! this is especially useful in small scripts, examples, or when you are using multiple parts of the crate at once.
//!
//! ## re-exported items
//!
//! ### profile api
//! - [`Profile`]
//! - [`ProfileClient`]
//! - [`ProfileError`]
//!
//! ### friends api
//! - [`Friends`]
//! - [`FriendsClient`]
//! - [`FriendsError`]
//!
//!
//! ### query api
//! - [`Query`]
//! - [`QueryClient`]
//! - [`QueryError`]
//!
//! ## example
//!
//! ```no_run
//! use luduvo_rs::prelude::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut client = ProfileClient::new(None);
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
//!
//! ## when not to use prelude
//!
//! if you prefer more explicit imports (which is recommended for larger projects), you may want to import items directly from their modules instead.
//!
//! this avoids namespace pollution and makes dependencies clearer.

#[cfg(feature = "friends")]
pub use super::users::friends::{
    Client as FriendsClient, Config as FriendsConfig, Error as FriendsError, Friends,
};

#[cfg(feature = "profile")]
pub use super::users::profile::{
    Client as ProfileClient, Config as ProfileConfig, Error as ProfileError, Profile,
};

#[cfg(feature = "query")]
pub use super::users::query::{
    Client as QueryClient, Config as QueryConfig, Error as QueryError, Query,
};

#[cfg(feature = "places")]
pub use super::places::{
    Client as PlacesClient, Config as PlacesConfig, Error as PlacesError, Place, Places,
};
