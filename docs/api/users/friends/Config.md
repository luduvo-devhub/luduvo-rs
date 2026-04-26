# Config

the configuration for the [`Client`] struct # arguments * `client` - the [`reqwest::Client`] to use * `base_url` - the base url of the api * `cache_timeout` - the amount of time it takes for cache entries to go stale

## fields

- client: `ReqwestClient`
- base_url: `String`
- cache_timeout: `u64`

## methods

### new
**arguments**
- client: `Option<ReqwestClient>`
- base_url: `Option<String>`
- cache_timeout: `Option<u64>`

**returns**
- `Config`

### default
**returns**
- `Config`

