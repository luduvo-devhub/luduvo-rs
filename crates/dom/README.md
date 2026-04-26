<p align="center">
  <img src="https://github.com/luduvo-devhub/luduvo-rs/blob/main/gh-assets/wordmark.png?raw=true" alt="luduvo-rs wordmark" height="256"/>
</p>

<p align="center">
  <a href="(https://crates.io/crates/luduvo-dom">crates.io</a> | <a href="https://discord.gg/FcjTvuWKRk">luduvo development hub</a>
</p>

---

<p align="center">
  <i>luduvo-dom</i> is a rust library designed for interacting with the `.ldv` file format.
</p>

> [!WARNING]
> this crate is completely fanmade and has no affiliation with the luduvo devs.

> [!IMPORTANT]
> this library is in a pre-1.0.0 state! expect breaking changes between versions.

> [!NOTE]
> this crate is MIT-licensed. feel free to do whatever with it! all contributions (pull requests, issues) are welcomed.

---

## installation

you can install luduvo-dom on crates.io.

```
cargo add luduvo-dom
```

to install the latest (and unstable) version of luduvo-dom:

```
cargo add luduvo-dom --git https://github.com/luduvo-devhub/luduvo-rs.git
```

## features

> [!TIP]
> most users will want to import the prelude, via `luduvo_dom::prelude::*`

- creating a `File` instance using `File::from`
- creating a `Dom` instance using `Dom::from_file`
- exporting the `Dom` instance using `Dom.to_file`

## quick start

```rust
use luduvo_dom::{data_types::Vec3, dom::Dom, file::File};

fn main() -> std::io::Result<()> {
    let data = std::fs::read("assets/in/world.ldv")?;
    let file = File::from(&data).unwrap();

    let mut dom = Dom::from_file(&file);

    dom.create_entity(999);
    dom.set_position(999, Vec3 { x: 1.0, y: 2.0, z: 3.0 });

    let new_file = dom.to_file().unwrap();
    let bytes = new_file.to_bytes();

    std::fs::write("assets/out/world.ldv", bytes)?;

    println!("wrote to assets/out/world.ldv");

    Ok(())
}
```

## acknowledgements

luduvo-dom would not be possible without the amazing oss community!

### packages

- [fixedstr](https://crates.io/crates/fixedstr) - `str64` type, used in `LdvString`
- [thiserror](https://crates.io/crates/thiserror) - graceful error handling via the error type

### contributors

- [Uzixt](Uzixt) for [documentation on the .ldv file spec](https://github.com/Uzixt/LdvFileSpec)
