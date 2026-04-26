# Cache

a cache of user profiles, keyed by user id. this is used internally by [`Client`] to cache profiles.

## fields

- cache: `HashMap<u64, CacheEntry>`
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
- id: `u64` - the id of the profile to retrieve.

**returns**
- `Option<Profile>`
- the profile if it is still valid (not expired)
- if the profile is expired or missing

### insert
inserts a profile into the cache.

**arguments**
- profile: `Profile` - the profile to insert.

### remove
removes a profile from the cache by its id.

**arguments**
- id: `u64` - the id of the profile to remove.

