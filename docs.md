# rust

[cargo 进阶](https://course.rs/cargo/reference/specify-deps.html)

目前国内 Rust 工具链镜像源，可以将其存储到系统环境的个性化设置文件中，如 .bashrc 或 .profile
rustup 设置国内镜像源

```shell
#中国科学技术大学源
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup

#上海交通大学源
export RUSTUP_DIST_SERVER=https://mirrors.sjtug.sjtu.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.sjtug.sjtu.edu.cn/rust-static/rustup

#清华大学源
export RUSTUP_UPDATE_ROOT=https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup
export RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup
```

install (需要科学上网)

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

#更新 stable 发布通道到最新稳定版本
rustup update stable

#rustup 可以灵活的在三类版本间切换，切换后 rustup 会自动同步该类版本到最新版
rustup default stable
rustup default beta
rustup default nightly

#安装3种发布通道的最新版本
rustup install stable
rustup install beta
rustup install nightly

#安装指定版本
rustup install 1.77.0
#安装指定版本的工具链
rustup toolchain install 1.77.0-x86_64-apple-darwin
#可以切换到指定的某个版本
rustup default 1.77.0-x86_64-apple-darwin
#卸载某个版本的工具链
rustup toolchain uninstall 1.77.0-x86_64-apple-darwin

rustup self uninstall

#查看已安装的工具链
rustup show
rustup toolchain list
```

cargo 设置国内镜像源
覆盖默认的镜像地址, 不需要修改 Cargo.toml 文件，它是直接使用新注册服务来替代默认的 crates.io
在 $HOME/.cargo/config.toml 或者 $HOME/.cargo/config 中添加以下内容：

```toml
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'

[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
```

```toml
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
# 指定镜像
replace-with = '镜像源名' # 如：tuna、sjtu、ustc，或者 rustcc

# 注：以下源配置一个即可，无需全部

# 中国科学技术大学
[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
# >>> 或者 <<<
#registry = "git://mirrors.ustc.edu.cn/crates.io-index"

# 上海交通大学
[source.sjtu]
registry = "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index/"

# 清华大学
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

# rustcc社区
[source.rustcc]
registry = "https://code.aliyun.com/rustcc/crates.io-index.git"
```

这样创建一个新的镜像源 [source.ustc]，然后将默认的 crates-io 替换成新的镜像源: replace-with = 'ustc'
只要这样配置后，以往需要去 crates.io 下载的包，会全部从科大的镜像地址下载

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

# If a Cargo.lock file is available, this command will ensure that all of the git dependencies and/or registry dependencies are downloaded and locally available. Subsequent Cargo commands will be able to run offline after a cargo fetch unless the lock file changes.
cargo fetch

cargo build
cargo build --release # 这个属于优化编译

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

#快速的检查一下代码能否编译通过
cargo check

#cargo install 命令用于在本地安装和使用二进制 crate, 所有来自 cargo install 
#的二进制文件都安装到 Rust 安装根目录的 bin 文件夹中
#如果需要一次安装多个，通过空格分隔即可
cargo install ripgrep mdbook

#更新包的版本
cargo update rand
```

examples

```shell
cargo run --example ex1
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

#cargo test为何使用新线程？因为它需要并行的运行测试用例，与之相反，cargo bench只需要
# 顺序的执行，因此 main 线程足矣
```

## bench

[tutorial](https://course.rs/test/benchmark.html)  

执行 benches 目录下的文件

```shell
cargo bench
```

## cache

Cargo使用缓存来提高构建效率，当执行构建命令时，它会把下载的依赖包存放在CARGO_HOME目录下
该目录默认位于用户的home目录下的.cargo文件夹内

bin/：包含了通过cargo install或rustup下载的可执行文件。你可以把此目录加入到$PATH环境变量中，以便直接访问这些可执行文件。
git/：存储git资源，其中：
git/db：当包依赖某个git仓库时, Cargo会将仓库克隆到此目录。
git/checkouts：指定git源和commit后，仓库会从git/db中复制到此目录。
registry/：包含注册中心的元数据和包：
registry/index：包含所有可用包的元数据的git仓库。
registry/cache：保存已下载依赖的压缩档案，以.crate后缀名保存。
registry/src：已下载的.crate档案会在此解压，rustc将在此寻找.rs文件

为了避免在CI过程中重复下载依赖，可以对$CARGO_HOME目录进行缓存。但不应当缓存整个目录，
因为这会导致文件被重复缓存，例如serde包的源文件可能同时出现在registry/cache和registry/src目录中。

出于效率的考虑，在CI构建时，我们应该只缓存以下目录：

bin/
registry/index/
registry/cache/
git/db/

## publish

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

Cargo.toml 是 cargo 特有的项目数据描述文件。它存储了项目的所有元配置信息，如果 Rust
开发者希望 Rust 项目能够按照期望的方式进行构建、测试和运行，那么，必须按照合理的方式构建 Cargo.toml。

Cargo.lock 文件是 cargo 工具根据同一项目的 toml 文件生成的项目依赖详细清单，因此我们
一般不用修改它，只需要对着 Cargo.toml 文件撸就行了。

什么情况下该把 Cargo.lock 上传到 git 仓库里？很简单，当你的项目是一个可运行的程序时，
就上传 Cargo.lock，如果是一个依赖库项目，那么请把它添加到 .gitignore 中。

3种外部依赖库的写法

```toml
[dependencies]
rand = "0.3"
hammer = { version = "0.5.0"}
color = { git = "https://github.com/bjz/color-rs" }
geometry = { path = "crates/geometry" }
```

## 切换 rust 版本

```shell
#安装 nightly 版本
rustup install nightly
#查看已安装版本
rustup toolchain list
#进入某个项目的根目录,将该项目使用的 rust 设置为 nightly
rustup override set nightly
#使用 1.77.0 特定版本进行构建 rust 项目
rustup override set 1.77.0
#切换回 stable 版本
rustup override set stable
#将设置的override规则取消掉
rustup override unset
```

Rust还提供了另外一种override toolchain版本的方法，我们可以在项目的根目录下
放置一个名为rust-toolchain.toml的文件

```toml
[toolchain]
channel = "1.77.0"
```

在项目根目录下执行 rustc -V 得到 rustc 1.77.0 (aedd173a2 2024-03-17)
而在其他目录下执行 rustc -V 得到 rustc 1.78.0 (9b00956e5 2024-04-29)

Rust规定版本override的优先级顺序由高到低依次是：

1. plus语法："rustc +1.66.0 -V"
2. RUSTUP_TOOLCHAIN环境变量 (default: none)
3. rustup override命令
4. rust-toolchain.toml
5. 默认toolchain
