// OUT_DIR 环境变量说明了构建脚本的输出目录，也就是最终生成的代码文件的存放地址
// OUT_DIR 默认是 ./target/debug/build/xxx-project/out
// include!(concat!(env!("OUT_DIR"), "/hello.rs"));

// 可以在 build.rs 中修改输出目录, 先创建 hello 目录
include!("hello/hello.rs");

fn main() {
    println!("{}", message());
}
