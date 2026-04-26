<p align="center">
  <img src="https://github.com/luduvo-devhub/luduvo-rs/blob/main/gh-assets/wordmark.png?raw=true" alt="luduvo-rs wordmark" height="256"/>
</p>

<p align="center">
  <a href="(https://crates.io/crates/luduvo-verify">crates.io</a> | <a href="https://discord.gg/FcjTvuWKRk">luduvo development hub</a>
</p>

---

<p align="center">
  <i>luduvo-verify</i> is a rust library designed for verifying a users luduvo account.
</p>

> [!WARNING]
> this crate is completely fanmade and has no affiliation with the luduvo devs.

> [!IMPORTANT]
> this library is in a pre-1.0.0 state! expect breaking changes between versions.

> [!NOTE]
> this crate is MIT-licensed. feel free to do whatever with it! all contributions (pull requests, issues) are welcomed.

---

## installation

you can install luduvo-verify on crates.io.

```
cargo add luduvo-verify
```

to install the latest (and unstable) version of luduvo-verify:

```
cargo add luduvo-verify --git https://github.com/luduvo-devhub/luduvo-rs.git
```

## features

> [!NOTE]
> this crate requires an asynchronous runtime (e.g tokio) for the `client.is_verified` function.

- creating a `Client` instance using `Client::new`
- generating a verification code using `client.generate_code`
- checking whether or not the user is verified via `client.is_verified`

## quick start

```rust
use luduvo_verify::{Client, DiscordUser, Settings, codegen::CodeComplexity};
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    let settings = Settings::new(None, None);
    let mut client = Client::new(Some(settings));

    let code = client.generate_code(Some(CodeComplexity::Low)).to_string();
    let user = DiscordUser { id: 0 };

    println!("verification code: {}", code);

    loop {
        print!("press enter if the code is in your bio");

        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        match client
            .is_verified("primiti_ve".to_string(), user, code.clone())
            .await
        {
            Ok(true) => {
                println!("user is verified!");

                break;
            }

            Ok(false) => println!("not verified yet, try again:\n"),
            Err(e) => eprintln!("error checking verification: {}\n", e),
        }
    }
}

```

## acknowledgements

luduvo-verify would not be possible without the amazing oss community!

### packages

- [thiserror](https://crates.io/crates/thiserror) - graceful error handling via the error type
- [rand](https://crates.io/crates/rand) - random verification code generation
- [luduvo-api](https://crates.io/crates/luduvo-api) - fetching user data