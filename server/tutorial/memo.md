``` sh
docker run -d  \
  -v webapi_mvp_db_data:/var/lib/postgresql/data \
  -e POSTGRES_USER=postres_user \
  -e POSTGRES_PASSWORD=postres_pass \
  -e POSTGRES_DB=rust_mvp \
  -p 9988:5432 \
  postgres:15-bullseye


rustup show
rustup default stable

cargo install diesel_cli --no-default-features --features postgres

export DATABASE_URL=postgres://postgres_user:postgres_pass@localhost:9988/rust_mvp
diesel setup

## migration
diesel migration generate create_projects
diesel migration generate create_technologies
diesel migration generate create_adopts

diesel migration run
diesel migration revert


cargo init
```

マクロの確認

``` sh
cargo install cargo-expand

cargo expand
```
