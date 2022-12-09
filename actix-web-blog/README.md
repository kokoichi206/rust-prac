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

## Dependencies

追加するクレート

- libsqlite3-sys: SQLite3 接続用
- anyhow, thiserror: エラーを簡単に扱うため

```sh
cargo add actix-web
cargo add serde -F derive
cargo add diesel -F r2d2 -F sqlite -F returning_clauses_for_sqlite_3_35
cargo add libsqlite3-sys -F bundled
cargo add anyhow thiserror
```

```sh
cargo make --env-file .env watch

curl -X POST 'http://localhost:8080/posts' -H 'Content-Type: application/json' -d '{"title": "Hello", "body": "my first time"}'
```
