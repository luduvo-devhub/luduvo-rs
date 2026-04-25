# contributing to luduvo-rs

- make a [pull request](https://github.com/luduvo-devhub/luduvo-rs/pulls) or create an [issue](https://github.com/luduvo-devhub/luduvo-rs/issues)!
- if your pull request is accepted, you'll be added to the "acknowledgements" list in the README of the respective crate(s)

## 2. prerequisites

- you need [rust](rustup.rs) for toolchain management.
- you need [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) for project/package management.

## 1. getting started

- clone the luduvo-rs repository.
  - using `--depth=1` is recommended to reduce file sizes and download time!
- run the qa command via `cargo run -- qa`.
  - this runs tests, clippy, and builds all crates.

## 2. making changes

- the repository should be editor-agnostic! if you add an unnecessary configuration file, please add it to `.gitignore`.
  - i personally use [zed](https://zed.dev/).

## 3. before you request a review

- pull requests may be rejected for:
  - making too many breaking changes
  - introducing unaddressed breaking changes elsewhere
