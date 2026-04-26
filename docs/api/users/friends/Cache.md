# Cache

a cache of user friends data, keyed by user id. this is used internally by [`Client`] to cache friends.

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
retrieves a result from the cache by its id.

**arguments**
- id: `u64` - the id of the result to retrieve.

**returns**
- `Option<Friends>`
- the result if it is still valid (not expired)
- if the result is expired or missing

### insert
inserts a result into the cache.

**arguments**
- id: `u64`
- result: `Friends` - the result to insert.

### remove
removes a result from the cache by its id.

**arguments**
- id: `u64` - the id of the result to remove.

