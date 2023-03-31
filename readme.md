## Building from source
export DATABASE_URL="sqlite:todos.db"
$ sqlx db create
$ sqlx migrate run
cargo run -- add "todo description"
cargo run -- --config config.json --file test.json