//! # places api
//!
//! this module contains structs related to fetching luduvo place data.

use reqwest::{Client as ReqwestClient, StatusCode};
use serde::Deserialize;
use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};
use thiserror::Error;

pub const BASE_URL: &str = "https://api.luduvo.com/places";

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

/// represents a single place.
#[derive(Clone, Debug, Deserialize)]
pub struct Place {
    pub id: u64,
    pub owner_id: u64,
    pub owner_username: String,
    pub title: String,
    pub description: String,
    pub access: String,
    pub max_players: u64,
    pub visit_count: u64,
    pub thumbs_up: u64,
    pub thumbs_down: u64,
    pub active_players: u64,
    pub created_at: u64,
    pub updated_at: u64,
    pub thumbnail_url: String,
}

/// represents places returned by the luduvo api.
#[derive(Clone, Debug, Deserialize)]
pub struct Places {
    /// the list of places.
    pub places: Vec<Place>,

    /// the total amount of places given.
    pub total: u64,

    /// the total amount of places requested.
    pub limit: u64,

    /// the current page of places.
    pub offset: u64,
}

/// a cached place entry, containing a places struct and its last updated timestamp.
///
/// this is used internally by [`Cache`] to store place data.
#[derive(Clone)]
pub struct CacheEntry {
    pub places: Places,
    pub last_updated: u64,
}

/// a cache of places, keyed by place id.
///
/// this is used internally by [`Client`] to cache place data.
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

    /// retrieves an entry from the cache by its id.
    ///
    /// # arguments
    ///
    /// * `id` - the id of the entry to retrieve.
    ///
    /// # returns
    ///
    /// - the entry if it is still valid (not expired)
    /// - `None` if the entry is expired or missing
    pub fn get(&mut self, id: &str) -> Option<Places> {
        if let Some(entry) = self.cache.get(id) {
            let age = Self::now() - entry.last_updated;

            if age <= self.cache_timeout {
                return Some(entry.places.clone());
            }
        }

        // expired or missing result entry
        // remove stale entry
        self.cache.remove(id);

        None
    }

    /// inserts an entry into the cache.
    ///
    /// # arguments
    ///
    /// * `id` - the id of the place to insert into the cache.
    /// * `places` - a vec list of the places.
    pub fn insert(&mut self, id: String, places: Places) {
        let cached = CacheEntry {
            places,
            last_updated: Self::now(),
        };

        self.cache.insert(id, cached);
    }

    /// removes an entry from the cache by its id.
    ///
    /// # arguments
    ///
    /// * `id` - the id of the place to remove.
    pub fn remove(&mut self, id: &str) {
        self.cache.remove(id);
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

/// a client for interacting with the luduvo places api.
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
    /// - this internally manages the cache for place data. the cache is not publicly exposed.
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

    /// fetches a list of places by name.
    ///
    /// # notes
    ///
    /// - this function is asynchronous.
    ///
    /// # arguments
    ///
    /// * `query` - the query as a string.
    /// * `limit` - the maximum number of profiles to fetch.
    ///
    /// # errors
    ///
    /// returns:
    /// - [`Error::RequestFailed`] for network or decoding errors
    /// - [`Error::TooManyRequests`] if the user has sent too many requests within a short timespan
    /// - [`Error::InternalError`] if something went wrong within the luduvo api
    /// - [`Places`] if successful
    ///
    /// # example
    ///
    /// ```no_run
    /// use luduvo_rs::places::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut wrapper = Client::new(None);
    ///
    ///     match wrapper.get_places("Luduvo".to_string(), None).await {
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
    pub async fn get_places(
        &mut self,
        query: String,
        limit: Option<String>,
    ) -> Result<Places, Error> {
        if let Some(cached) = self.cache.get(&query) {
            return Ok(cached);
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
        let places = response.json::<Places>().await?;

        self.cache.insert(query, places.clone());

        Ok(places)
    }
}
