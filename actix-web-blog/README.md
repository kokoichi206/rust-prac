## Setup

```sh
cargo install diesel_cli --no-default-features --features sqlite-bundled

diesel setup
diesel migration generate create_posts

# up, down を編集
diesel migration run

# sqlite3 がある場合
sqlite3 posts.db
sqlite > .schema posts

# cargo-make では --env-file で環境変数に追加してくれる
cargo make --env-file .env run
```
