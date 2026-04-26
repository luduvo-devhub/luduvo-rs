<p align="center">
  <img src="https://github.com/luduvo-devhub/luduvo-rs/blob/main/gh-assets/wordmark.png?raw=true" alt="luduvo-rs wordmark" height="256"/>
</p>

<p align="center">
  <a href="(https://crates.io/crates/luduvo-api">crates.io</a> | <a href="https://discord.gg/FcjTvuWKRk">luduvo development hub</a>
</p>

---

<p align="center">
  <i>luduvo-api</i> is a rust library designed for interacting with the <a href="luduvo.com">luduvo</a> api.
</p>

> [!WARNING]
> this crate is completely fanmade and has no affiliation with the luduvo devs.

> [!IMPORTANT]
> this library is in a pre-1.0.0 state! expect breaking changes between versions.

> [!NOTE]
> this crate is MIT-licensed. feel free to do whatever with it! all contributions (pull requests, issues) are welcomed, including to the docs.

---

## installation

you can install luduvo-api on crates.io.

```
cargo add luduvo-api
```

to install the latest (and unstable) version of luduvo-api:

```
cargo add luduvo-api --git https://github.com/luduvo-devhub/luduvo-rs.git
```

## features

> [!TIP]
> most users will want to import the prelude, via `luduvo_api::prelude::*`

### unauthorized

- user profile data (search by id, one result)
- user friends data (search by id, multiple results)
- user querying (search by username, multiple results)
- places data (search by name, multiple results)

### authorized

- none yet...

## quick start

```rust
use luduvo_api::prelude::*;

#[tokio::main]
async fn main() {
    let mut client = ProfileClient::new(None);

    let id = "1".to_string();
    let profile = client.get_user(id).await.unwrap();

    println!("hello, {}!", profile.username);
}
```

## acknowledgements

luduvo-rs would not be possible without the amazing oss community!

### packages

- [reqwest](https://crates.io/crates/reqwest) - an ergonomic http client
- [serde](https://crates.io/crates/serde) - converting api responses to rust structs
- [thiserror](https://crates.io/crates/thiserror) - graceful error handling via the error types

### contributors

- [Eeviika](https://github.com/Eeviika) for [#1](https://github.com/luduvo-devhub/luduvo-rs/pull/1) (Small changes)
