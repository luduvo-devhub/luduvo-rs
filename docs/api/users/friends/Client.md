# Client

a client for interacting with the luduvo friends api. this struct internally initializes a reusable [`reqwest::Client`] to perform HTTP requests.

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

### get_friends
fetches a users friends by id.

**arguments**
- id: `String` - the user id as a string.

**returns**
- `Result<Friends, Error>`

