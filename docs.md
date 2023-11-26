# rust

install (需要科学上网)

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

rustup update
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

#可以将一部分命令行参数传递给 cargo build，而将另外一部分传递给生成的二进制文件。
# 为了分隔这两种参数，需要先列出传递给 cargo build 的参数，接着是分隔符 --，
# 再之后是传递给二进制文件的参数。
cargo build -- sample.txt
```

test

```shell
#想要更加精确的控制线程的数量，可以传递 --test-threads 参数和希望使用线程的数量给测试二进制文件
cargo test -- --test-threads=1

#如果你希望也能看到通过的测试中打印的值，也可以在结尾加上 --show-output 告诉 Rust 显示成功测试的输出。
cargo test -- --show-output

#可以向 cargo test 传递任意测试的名称来只运行这个测试
cargo test one_hundred

#我们可以指定部分测试的名称，任何名称匹配这个名称的测试会被运行。例如，
#因为头两个测试的名称包含 add，可以通过 cargo test add 来运行这两个测试

#如果我们只希望运行被忽略的测试，可以使用 cargo test -- --ignored
#如果你希望不管是否忽略都要运行全部测试，可以运行 cargo test -- --include-ignored

#使用 cargo test 的 --test 后跟文件的名称来运行某个特定集成测试文件中的所有测试
cargo test --test integration_test
```
