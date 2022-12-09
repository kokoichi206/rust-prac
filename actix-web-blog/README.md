## Setup

```sh
cargo install diesel_cli --no-default-features --features sqlite-bundled

diesel setup
diesel migration generate create_posts
```
