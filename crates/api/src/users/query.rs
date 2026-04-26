use reqwest::{Client as ReqwestClient, StatusCode};
use serde::Deserialize;
use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};
use thiserror::Error;

use super::BASE_URL;

/// errors that can occur when fetching profiles.
#[derive(Error, Debug)]
pub enum Error {
    /// the user has sent too many requests to the api.
    #[error("too many requests")]
    TooManyRequests(),

    /// an internal http client error occurred.
    #[error("request failed: `{0}`")]
    RequestFailed(#[from] reqwest::Error),

    /// an error with luduvo servers occurred.
    #[error("there was an error with the luduvo servers: `{0}`")]
    InternalError(String),
}

/// represents the color configuration of a user's avatar.
///
/// all fields are hex color strings. by default, they are all set to `#C8C8C8`.
#[derive(Clone, Debug, Deserialize)]
pub struct QueryAvatar {
    pub head_color: String,
    pub torso_color: String,

    pub left_arm_color: String,
    pub right_arm_color: String,

    pub left_leg_color: String,
    pub right_leg_color: String,
}

/// represents a user profile returned by the luduvo api.
#[derive(Clone, Debug, Deserialize)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub display_name: String,
    pub role: String,

    pub head_color: String,
    pub torso_color: String,

    pub created_at: u64,
}

/// a user query containing a list of [`User`]s
#[derive(Clone, Debug)]
pub struct Query {
    pub users: Vec<User>,
}

/// a cached profile entry, containing a profile and its last updated timestamp.
///
/// this is used internally by [`Cache`] to store profile data.
#[derive(Clone)]
pub struct CacheEntry {
    pub users: Query,
    pub last_updated: u64,
}

/// a cache of user profiles, keyed by user id.
///
/// this is used internally by [`Client`] to cache profiles.
#[derive(Clone)]
pub struct Cache {
    cache: HashMap<String, CacheEntry>,
    cache_timeout: u64,
}

/// the implementation for the profilecache struct.
impl Cache {
    /// creates a new [`Cache`] with the specified cache timeout.
    ///
    /// # arguments
    ///
    /// * `cache_timeout` - the cache timeout in seconds.
    ///
    /// # returns
    ///
    /// - a new [`Cache`] instance
    pub fn new(cache_timeout: u64) -> Self {
        Self {
            cache: HashMap::new(),
            cache_timeout,
        }
    }

    fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }

    /// retrieves a profile from the cache by its id.
    ///
    /// # arguments
    ///
    /// * `id` - the id of the profile to retrieve.
    ///
    /// # returns
    ///
    /// - the profile if it is still valid (not expired)
    /// - `None` if the profile is expired or missing
    pub fn get(&mut self, query: &str) -> Option<Query> {
        if let Some(entry) = self.cache.get(query) {
            let age = Self::now() - entry.last_updated;

            if age <= self.cache_timeout {
                return Some(entry.users.clone());
            }
        }

        // expired or missing profile entry
        // remove stale entry
        self.cache.remove(query);

        None
    }

    /// inserts a profile into the cache.
    ///
    /// # arguments
    ///
    /// * `profile` - the profile to insert.
    pub fn insert(&mut self, query: String, users: Vec<User>) {
        let cached = CacheEntry {
            users: Query { users },
            last_updated: Self::now(),
        };

        self.cache.insert(query, cached);
    }

    /// removes a profile from the cache by its username.
    ///
    /// # arguments
    ///
    /// * `username` - the username of the profile to remove.
    pub fn remove(&mut self, query: &str) {
        self.cache.remove(query);
    }
}

/// the configuration for the [`Client`] struct
///
/// # arguments
///
/// * `client` - the [`reqwest::Client`] to use
/// * `base_url` - the base url of the api
/// * `cache_timeout` - the amount of time it takes for cache entries to go stale
#[derive(Clone)]
pub struct Config {
    client: ReqwestClient,
    base_url: String,
    cache_timeout: u64,
}

impl Config {
    pub fn new(
        client: Option<ReqwestClient>,
        base_url: Option<String>,
        cache_timeout: Option<u64>,
    ) -> Config {
        let client = client.unwrap_or_default();
        let base_url = base_url.unwrap_or_default();
        let cache_timeout = cache_timeout.unwrap_or_default();

        Config {
            client,
            base_url,
            cache_timeout,
        }
    }
}

impl Default for Config {
    fn default() -> Config {
        let client = ReqwestClient::new();
        let base_url = BASE_URL.to_string();
        let cache_timeout = 30_u64;

        Config {
            client,
            base_url,
            cache_timeout,
        }
    }
}

/// a client for interacting with the luduvo profile querying api.
///
/// this struct internally initializes a reusable [`reqwest::Client`] to perform HTTP requests.
#[derive(Clone)]
pub struct Client {
    config: Config,
    cache: Cache,
}

impl Client {
    /// creates a new [`Client`].
    ///
    /// # notes
    ///
    /// - this internally initializes a reusable [`reqwest::Client`] to perform HTTP requests, which is **not** publicly exposed.
    /// - this internally manages the cache for query data. the cache is not publicly exposed.
    ///
    /// # arguments
    ///
    /// * `config` - the [`Config`] to use.
    ///
    /// # returns
    ///
    /// - a new [`Client`] instance if successful
    pub fn new(config: Option<Config>) -> Self {
        let config = config.unwrap_or_default();
        let cache = Cache::new(config.cache_timeout);

        Self { config, cache }
    }

    /// fetches a user profile by username.
    ///
    /// # notes
    ///
    /// - this function is asynchronous.
    ///
    /// # arguments
    ///
    /// * `query` - the username as a string.
    /// * `limit` - the maximum number of profiles to fetch.
    ///
    /// # errors
    ///
    /// returns:
    /// - [`Error::UserNotFound`] if the user does not exist (HTTP 404)
    /// - [`Error::RequestFailed`] for network or decoding errors
    /// - [`Error::TooManyRequests`] if the user has sent too many requests within a short timespan
    /// - [`Query`] if successful
    ///
    /// # example
    ///
    /// ```no_run
    /// use luduvo_rs::users::query::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut wrapper = Client::new(None);
    ///
    ///     match wrapper.get_user("Luduvo".to_string(), None).await {
    ///         Ok(user) => {
    ///             println!("{:#?}", user);
    ///         },
    ///
    ///         Err(e) => {
    ///             eprintln!("error caught while attempting to get user: '{}'", e);
    ///         },
    ///     }
    /// }
    /// ```
    pub async fn get_user(&mut self, query: String, limit: Option<String>) -> Result<Query, Error> {
        if let Some(users) = self.cache.get(&query) {
            return Ok(users);
        }

        let limit = limit.unwrap_or("20".to_string());

        let url = format!("{}?q={}&limit={}", self.config.base_url, query, limit);
        let response = self.config.client.get(&url).send().await?;

        let status = response.status();

        if status == StatusCode::TOO_MANY_REQUESTS {
            return Err(Error::TooManyRequests());
        } else if status == StatusCode::INTERNAL_SERVER_ERROR {
            let reason = status.canonical_reason().unwrap_or("no error supplied");

            return Err(Error::InternalError(reason.to_string()));
        }

        let response = response.error_for_status()?;
        let users = response.json::<Vec<User>>().await?;

        self.cache.insert(query, users.clone());

        Ok(Query { users })
    }
}
