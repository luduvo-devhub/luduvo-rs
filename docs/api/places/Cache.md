# Cache

a cache of places, keyed by place id. this is used internally by [`Client`] to cache place data.

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
retrieves an entry from the cache by its id.

**arguments**
- id: `&str` - the id of the entry to retrieve.

**returns**
- `Option<Places>`
- the entry if it is still valid (not expired)
- if the entry is expired or missing

### insert
inserts an entry into the cache.

**arguments**
- id: `String` - the id of the place to insert into the cache.
- places: `Places` - a vec list of the places.

### remove
removes an entry from the cache by its id.

**arguments**
- id: `&str` - the id of the place to remove.

