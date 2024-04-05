# sqlx

[sqlx](https://github.com/launchbadge/sqlx)
[realworld-axum-sqlx](https://github.com/launchbadge/realworld-axum-sqlx)

SQLx is not an ORM!

SQLx supports compile-time checked queries. It does not, however, do this by providing a Rust API or DSL (domain-specific language) for building queries. Instead, it provides macros that take regular SQL as input and ensure that it is valid for your database. The way this works is that SQLx connects to your development DB at compile time to have the database itself verify (and return some info on) your SQL queries. This has some potentially surprising implications:

Since SQLx never has to parse the SQL string itself, any syntax that the development DB accepts can be used (including things added by database extensions)
Due to the different amount of information databases let you retrieve about queries, the extent of SQL verification you get from the query macros depends on the database

```shell
# supports all databases supported by SQLx
$ cargo install sqlx-cli
```

## Migration

```shell
export DATABASE_URL="mysql://root:password@localhost/todos"
#Create/drop the database
#sqlx database create
#sqlx database drop
sqlx db create
#Create migrations
sqlx migrate add todos
#Run migrations
sqlx migrate run
```

## Usage

```shell
#Add a todo 
cargo run -- add "todo description"
#Complete a todo
cargo run -- done <todo id>
cargo run -- done 1
#List all todos
cargo run
```
