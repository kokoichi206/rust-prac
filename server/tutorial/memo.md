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

data

``` sh
dc exec postgres psql -U postgres_user rust_mvp


# seeder
dc exec postgres psql -U postgres_user rust_mvp -c "INSERT INTO projects(name, url_name) VALUES('My Project', 'my-project');"
dc exec postgres psql -U postgres_user rust_mvp -c "INSERT INTO adopts(projects_id, technologies_id, created_at) VALUES(1, 1, '2024-09-16 14:00:19.000000');"
dc exec postgres psql -U postgres_user rust_mvp -c "INSERT INTO technologies (name, url_name, image_url) VALUES ('Rust', 'rust', 'https://www.rust-lang.org/logos/rust-logo-blk.svg');"

# access: http://localhost:9911/technologies/rust
```

- HTML renderer
  - [tera](https://tera.netlify.app/)
