## Setting Up Database

``` bash
# install sqlx-cli
$ cargo install sqlx-cli --no-default-features --features postgres

# create database specified in DATABASE_URL
$ sqlx database create

# create migration files
$ sqlx migrate add create_token

# fill the files with the required tables and data, then run the migration
$ sqlx migrate run

# other commands
$ sqlx database drop # drop database
```
