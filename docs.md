# rust

install (需要科学上网)

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# uninstall
rustup self uninstall
```

```shell
#编译
rustc helloworld.rs
# 或者优化编译
rustc helloworld.rs -O
```

cargo 编译

```shell
# new 一个二进制程序 (默认)
cargo new hello_world
cargo new hello_world --bin

$ cargo build
$ cargo build --release # 这个属于优化编译

cargo run # 编译和运行合在一起
cargo run --release # 同上，区别是是优化编译的

#只编译src/lib.rs作为入口的 library crate, (因为一个package里面只能有一个 library crate，所以无需指定名字)
carge build --lib
#只编译 src/main.rs 作为入口的 binary crate，执行
cargo build --bin package-name
#只编译 src/bin/xxx.rs 作为入口的 binary crate
cargo build --bin xxx
#运行某个 binary crate
cargo run --bin package-name
cargo run --bin xxx

cargo run --example ex1

#生成 crate 文档并打开
cargo doc --open

#添加依赖并开始相应的 features
cargo add tokio --features full
```
