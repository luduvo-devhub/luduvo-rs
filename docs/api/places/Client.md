# Client

a client for interacting with the luduvo places api. this struct internally initializes a reusable [`reqwest::Client`] to perform HTTP requests.

## fields

- config: `Config`
- cache: `Cache`

## methods

### new
creates a new [`Client`].

**arguments**
- config: `Option<Config>`

**returns**
- `Self`
- a new [`Client`] instance if successful

### get_places
fetches a list of places by name.

**arguments**
- query: `String` - the query as a string.
- limit: `Option<String>`

**returns**
- `Result<Places, Error>`

