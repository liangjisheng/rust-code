# mysql

[docs](https://diesel.rs/guides/)

```shell
#install diesel 命令行工具
cargo install diesel_cli
```

```shell
cargo new diesel_demo_step_1_mysql --lib
cd diesel_demo_step_1_mysql
diesel setup
```

如果执行 diesel setup 报下面的错误的话

```log
dyld: Library not loaded: @rpath/libmysqlclient.21.dylib
  Referenced from: ~/.cargo/bin/diesel
  Reason: image not found
zsh: abort      diesel setup
```

可以使用下面的的方法解决

```shell
sudo ln -s /usr/local/mysql/lib/libmysqlclient.21.dylib /usr/local/lib/libmysqlclient.21.dylib
#如果没有 libmysqlclient.21.dylib 这个文件，试着执行下面的语句装一下
pip uninstall mysqlclient
```

```shell
diesel migration generate create_posts
diesel migration run
```
