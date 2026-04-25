<p align="center">
  <img src="https://github.com/luduvo-devhub/luduvo-rs/blob/main/gh-assets/wordmark.png?raw=true" alt="luduvo-rs wordmark" height="256"/>
</p>

<p align="center">
  <a href="https://luduvo-devhub.github.io/luduvo-rs">docs</a> | <a href="https://crates.io/crates/luduvo-rs">crates.io</a> | <a href="https://discord.gg/FcjTvuWKRk">luduvo development hub</a>
</p>

---

<p align="center">
  <i>luduvo-rs</i> is a rust library designed for interacting with the [luduvo](luduvo.com) api.
</p>

> [!WARNING]
> this crate is completely fanmade and has no affiliation with the luduvo devs.

> [!IMPORTANT]
> this library is in a pre-1.0.0 state! expect breaking changes between versions.

---

## features

> [!TIP]
> most users will want to import the prelude, via `luduvo_rs::prelude::*`

- user profile data (search by id, one result)
- user friends data (search by id, multiple results)
- user querying (search by username, multiple results)
- places data (search by name, multiple results)

## quick start

```rust
use luduvo_rs::prelude::*;

#[tokio::main]
async fn main() {
    let mut client = ProfileClient::new(None);

    let id = "1".to_string();
    let profile = client.get_user(id).await.unwrap();

    println!("hello, {}!", profile.username);
}
```

## contributors

> [!NOTE]
> this crate is MIT-licensed. feel free to do whatever with it! all contributions (pull requests, issues) are welcomed, including to the docs.

- [Eeviika](https://github.com/Eeviika) for [#1](https://github.com/luduvo-devhub/luduvo-rs/pull/1) (Small changes)

## need help?

- contact me on discord! my discord username is `@primiti_ve`.
    - my preferred method of communication is joining the [luduvo development hub](https://discord.gg/FcjTvuWKRk)! it's full of like-minded developers who will gladly help you out with any issues.
- [create an issue](https://github.com/luduvo-devhub/luduvo-rs/issues)! this is better for organisation purposes, although you should also join the luduvo development hub aswell.
