# Notes

## install

Dependencies:

```bash
apt-get install build-essential
apt-get install libsqlite3-dev sqlite3
apt-get install default-libmysqlclient-dev
apt-get install libpq-dev
```

Run project:

```bash
cargo install diesel_cli
diesel setup
diesel migration run
cargo run
```