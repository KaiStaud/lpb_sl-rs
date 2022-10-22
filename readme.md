export DATABASE_URL="sqlite:todos.db"
$ sqlx db create
$ sqlx migrate run
cargo run -- add "todo description"
cargo run

## Checking GPIO
export Pins:
`echo 16 > /sys/class/gpio/export`
`echo "out" > /sys/class/gpio/gpio16/direction`
`echo 1 > /sys/class/gpio/gpio16/value`