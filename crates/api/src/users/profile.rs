use reqwest::{Client as ReqwestClient, StatusCode};
use serde::Deserialize;
use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};
use thiserror::Error;

use super::BASE_URL;

/// errors that can occur when fetching a profile.
#[derive(Error, Debug)]
pub enum Error {
    /// the profile with the specified id was not found.
    #[error("profile with id `{0}` not found")]
    ProfileNotFound(String),

    /// the id is invalid.
    #[error("invalid id `{0}`")]
    InvalidId(String),

    /// the user has sent too many requests to the api.
    #[error("too many requests")]
    TooManyRequests(),

    /// an internal http client error occurred.
    #[error("request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    /// an error with luduvo servers occurred.
    #[error("there was an error with the luduvo servers: `{0}`")]
    InternalError(String),
}

/// represents the color configuration of a user's avatar.
///
/// all fields are hex color strings. by default, they are all set to `#C8C8C8`.
#[derive(Clone, Debug, Deserialize)]
pub struct ProfileAvatar {
    pub head_color: String,
    pub torso_color: String,

    pub left_arm_color: String,
    pub right_arm_color: String,

    pub left_leg_color: String,
    pub right_leg_color: String,
}

/// represents a user's obtained badge
#[derive(Clone, Debug, Deserialize)]
pub struct Badge {
    /// the id of the badge
    pub id: u64,

    /// the name of the badge
    pub name: String,

    /// a human-readable version of the badge's name (according to wikipedia)
    pub slug: String,

    /// the description of the badge
    pub description: String,

    /// the url for the shown icon of the badge
    pub icon_url: String,
}

/// represents a user's equipped item
#[derive(Clone, Debug, Deserialize)]
pub struct EquippedItem {
    // todo: fill this in later
}

/// represents a user profile returned by the luduvo api.
#[derive(Clone, Debug, Deserialize)]
pub struct Profile {
    /// the users id.
    /// this is unique to each profile.
    pub user_id: u64,

    /// the users username.
    /// this is unique to each profile.
    pub username: String,

    /// display name shown to other users.
    ///
    /// when the account is first created, this defaults to [`username`](Self::username). it can be changed by the user at any time.
    pub display_name: String,

    /// optional long-form description of the profile.
    pub bio: Option<String>,

    /// a status code of what the user is currently doing.
    pub status: Option<String>,

    /// the user's avatar appearance configuration.
    /// currently, it is just hex codes for the avatar's limbs.
    pub avatar: ProfileAvatar,

    /// a list of the user's equipped items.
    pub equipped_items: Vec<EquippedItem>,

    /// a list of badge identifiers earned by the user.
    pub badges: Vec<Badge>,

    /// the total number of friends the user has.
    pub friend_count: u64,

    /// the total number of owned places the user has.
    pub place_count: u64,

    /// the total number of owned items the user has.
    pub item_count: u64,

    /// last active timestamp (in unix seconds).
    ///
    /// this is a `None` if the user has never logged in.
    pub last_active: Option<u64>,

    /// account creation timestamp (in unix seconds).
    pub member_since: Option<u64>,

    /// whether others are allowed to join this user.
    pub allow_joins: bool,

    /// whether the current viewer owns the resource being viewed.
    pub is_owner: bool,
}

/// a cached profile entry, containing a profile and its last updated timestamp.
///
/// this is used internally by [`Cache`] to store profile data.
#[derive(Clone)]
pub struct CacheEntry {
    pub profile: Profile,
    pub last_updated: u64,
}

/// a cache of user profiles, keyed by user id.
///
/// this is used internally by [`Client`] to cache profiles.
#[derive(Clone)]
pub struct Cache {
    cache: HashMap<u64, CacheEntry>,
    cache_timeout: u64,
}

/// the implementation for the Cache struct.
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
    pub fn get(&mut self, id: u64) -> Option<Profile> {
        if let Some(entry) = self.cache.get(&id) {
            let age = Self::now() - entry.last_updated;

            if age <= self.cache_timeout {
                return Some(entry.profile.clone());
            }
        }

        // expired or missing profile entry
        // remove stale entry
        self.cache.remove(&id);

        None
    }

    /// inserts a profile into the cache.
    ///
    /// # arguments
    ///
    /// * `profile` - the profile to insert.
    pub fn insert(&mut self, profile: Profile) {
        let id = profile.user_id;
        let cached = CacheEntry {
            profile,
            last_updated: Self::now(),
        };

        self.cache.insert(id, cached);
    }

    /// removes a profile from the cache by its id.
    ///
    /// # arguments
    ///
    /// * `id` - the id of the profile to remove.
    pub fn remove(&mut self, id: u64) {
        self.cache.remove(&id);
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

/// a client for interacting with the luduvo user profile api.
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
    /// - this internally manages the cache for profile data. the cache is not publicly exposed.
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

    /// fetches a user profile by id.
    ///
    /// # notes
    ///
    /// - this function is asynchronous.
    ///
    /// # arguments
    ///
    /// * `id` - the user id as a string.
    ///
    /// # errors
    ///
    /// returns:
    /// - [`Error::ProfileNotFound`] if the profile does not exist (HTTP 404)
    /// - [`Error::RequestFailed`] for network or decoding errors
    /// - [`Error::InvalidId`] if the id is not a valid string
    /// - [`Error::TooManyRequests`] if the user has sent too many requests within a short timespan
    /// - [`Profile`] if successful
    ///
    /// # example
    ///
    /// ```no_run
    /// use luduvo_rs::users::profile::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut wrapper = Client::new(None);
    ///
    ///     match wrapper.get_user("1".to_string()).await {
    ///         Ok(profile) => {
    ///             println!("{:#?}", profile);
    ///         },
    ///
    ///         Err(e) => {
    ///             eprintln!("error caught while attempting to get profile: '{}'", e);
    ///         },
    ///     }
    /// }
    /// ```
    pub async fn get_user(&mut self, id: String) -> Result<Profile, Error> {
        let id_num: u64 = id.parse().map_err(|_| Error::InvalidId(id.clone()))?;

        if let Some(profile) = self.cache.get(id_num) {
            return Ok(profile);
        }

        let url = format!("{}/{}/profile", self.config.base_url, id);
        let response = self.config.client.get(&url).send().await?;

        let status = response.status();

        if status == StatusCode::NOT_FOUND {
            return Err(Error::ProfileNotFound(id));
        } else if status == StatusCode::TOO_MANY_REQUESTS {
            return Err(Error::TooManyRequests());
        } else if status == StatusCode::INTERNAL_SERVER_ERROR {
            let reason = status.canonical_reason().unwrap_or("no error supplied");

            return Err(Error::InternalError(reason.to_string()));
        }

        let response = response.error_for_status()?;
        let profile = response.json::<Profile>().await?;

        self.cache.insert(profile.clone());

        Ok(profile)
    }

    /// fetches a user profile by username.
    ///
    /// # arguments
    ///
    /// * `username` - the username of the user.
    ///
    /// # errors
    ///
    /// returns:
    /// - [`Error::ProfileNotFound`] if the profile does not exist (HTTP 404)
    /// - [`Error::RequestFailed`] for network or decoding errors
    /// - [`Error::TooManyRequests`] if rate limited
    /// - [`Error::InternalError`] if server error occurs
    pub async fn get_user_by_username(&mut self, username: String) -> Result<Profile, Error> {
        let url = format!("{}/by-username/{}/profile", self.config.base_url, username);

        let response = self.config.client.get(&url).send().await?;
        let status = response.status();

        if status == StatusCode::NOT_FOUND {
            return Err(Error::ProfileNotFound(username));
        } else if status == StatusCode::TOO_MANY_REQUESTS {
            return Err(Error::TooManyRequests());
        } else if status == StatusCode::INTERNAL_SERVER_ERROR {
            let reason = status.canonical_reason().unwrap_or("no error supplied");

            return Err(Error::InternalError(reason.to_string()));
        }

        let response = response.error_for_status()?;
        let profile = response.json::<Profile>().await?;

        self.cache.insert(profile.clone());

        Ok(profile)
    }
}
