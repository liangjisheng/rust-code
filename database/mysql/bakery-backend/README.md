# bakery

```shell
# In case you have not installed `sea-orm-cli`
$ cargo install sea-orm-cli
# Generate entity files of database `bakeries_db` to `src/entities`
$ sea-orm-cli generate entity \
    -u mysql://root:password@localhost:3306/bakeries_db \
    -o src/entities
```
