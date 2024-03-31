# cross

rust 跨平台编译

```shell
#安装 cross
cargo install cross
cross --version

#编译前需要先启动 docker
#分别构建 Windows、Mac和Linux上 的二进制文件
#构建完成后，你将在
# target/x86_64-pc-windows-gnu/release、
# target/x86_64-apple-darwin/release
# target/x86_64-unknown-linux-gnu/release
# 目录中找到三个二进制文件

cross build --release --target x86_64-pc-windows-gnu
cross build --release --target x86_64-apple-darwin
cross build --release --target x86_64-unknown-linux-gnu
```
