# Client

a client for interacting with the luduvo profile querying api. this struct internally initializes a reusable [`reqwest::Client`] to perform HTTP requests.

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

### get_user
fetches a user profile by username.

**arguments**
- query: `String` - use luduvo_rs::users::query::Client;
- limit: `Option<String>`

**returns**
- `Result<Query, Error>`

