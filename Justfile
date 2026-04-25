set shell := ["pwsh.exe", "-Command"]

ci *args:
    cargo run -- {{ args | join(" ") }}
