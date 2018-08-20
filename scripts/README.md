# scripts

This module contains useful scripts to automate tasks.

## Scripts

### Clean DB

Removes everything in the DB.

```sh
cargo run --package scripts --bin scripts -- clean_db
```

### Reset DB

Removes everything in the DB and then creates the collections and other default values.

```sh
cargo run --package scripts --bin scripts -- reset_db
```