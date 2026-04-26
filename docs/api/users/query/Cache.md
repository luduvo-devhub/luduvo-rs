# Cache

a cache of user profiles, keyed by user id. this is used internally by [`Client`] to cache profiles.

## fields

- cache: `HashMap<String, CacheEntry>`
- cache_timeout: `u64`

## methods

### new
creates a new [`Cache`] with the specified cache timeout.

**arguments**
- cache_timeout: `u64`

**returns**
- `Self`
- a new [`Cache`] instance

### now
**returns**
- `u64`

### get
retrieves a profile from the cache by its id.

**arguments**
- query: `&str`

**returns**
- `Option<Query>`
- the profile if it is still valid (not expired)
- if the profile is expired or missing

### insert
inserts a profile into the cache.

**arguments**
- query: `String`
- users: `Vec<User>`

### remove
removes a profile from the cache by its username.

**arguments**
- query: `&str`

