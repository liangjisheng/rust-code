# rust

[rust 学习资源](https://juejin.cn/post/7194422977872527397)
[rust 官网](https://www.rust-lang.org/)
[Rust 官方文档](https://doc.rust-lang.org/)  
[Rust 在线工具](https://play.rust-lang.org/)  
[Rust Github 地址](https://github.com/rust-lang/rust)
[Rust 中文社区](https://github.com/rustcc)
[RustCn 社区](https://github.com/rustlang-cn)
[rustup 项目主页](https://github.com/rust-lang-nursery/rustup.rs)
[rust](https://prev.rust-lang.org/zh-CN/documentation.html)

## github

[rust learning](https://github.com/ctjhoa/rust-learning)
[chinanf-boy](https://github.com/chinanf-boy)

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
```

## articles

[cargo](https://course.rs/cargo/intro.html)
[cargo](https://www.bookstack.cn/read/RustPrimer/cargo-detailed-cfg-cargo-detailed-cfg.md)
[features](https://course.rs/cargo/reference/features/intro.html)
