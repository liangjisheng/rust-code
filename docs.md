# rust

install (需要科学上网)

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

#更新到最新稳定版本
rustup default stable

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

#生成的 HTML 文档放入 target/doc 目录
cargo doc
#生成 crate 文档并打开
cargo doc --open

#添加依赖并开始相应的 features
cargo add tokio --features full

#可以将一部分命令行参数传递给 cargo build，而将另外一部分传递给生成的二进制文件。
# 为了分隔这两种参数，需要先列出传递给 cargo build 的参数，接着是分隔符 --，
# 再之后是传递给二进制文件的参数。
cargo build -- sample.txt
```

install

cargo install 命令用于在本地安装和使用二进制 crate, 所有来自 cargo install 的二进制文件都安装到 Rust 安装根目录的 bin 文件夹中

```shell
cargo install ripgrep
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

publish

```shell
#去 crates.io 注册账号
cargo login api-token
#在 Cargo.toml 中添加所必须的元数据
cargo publish

#虽然你不能删除之前版本的 crate，但是可以阻止任何将来的项目将它们加入到依赖中
#Cargo 支持 撤回（yanking）某个版本
#撤回某个版本会阻止新项目依赖此版本，不过所有现存此依赖的项目仍然能够下载和依赖这个版本
#撤回 并没有 删除任何代码。举例来说，撤回功能并不能删除不小心上传的秘密信息。如果出现了这种情况，请立即重新设置这些秘密信息。
cargo yank --vers 1.0.1
#也可以撤销撤回操作，并允许项目可以再次开始依赖某个版本，通过在命令上增加 --undo
cargo yank --vers 1.0.1 --undo
```

Cargo 的设计使得开发者可以通过新的子命令来对 Cargo 进行扩展，而无需修改 Cargo 本身。
如果 $PATH 中有类似 cargo-something 的二进制文件，就可以通过 cargo something 
来像 Cargo 子命令一样运行它。像这样的自定义命令也可以运行 cargo --list 来展示出来。
能够通过 cargo install 向 Cargo 安装扩展并可以如内建 Cargo 工具那样运行它们是 
Cargo 设计上的一个非常方便的优点！
