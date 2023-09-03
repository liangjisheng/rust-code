use std::io::Result;

fn main() -> Result<()> {
    // tonic_build::compile_protos("protos/voting.proto")?;

    tonic_build::configure()
        .build_server(true) // 是否编译生成用于服务端的代码
        .build_client(true) // 是否编译生成用于客户端的代码
        .out_dir("protos") // 输出的路径，此处指定为项目根目录下的protos目录
        // 指定要编译的proto文件路径列表，第二个参数是提供protobuf的扩展路径，
        // 因为protobuf官方提供了一些扩展功能，自己也可能会写一些扩展功能，
        // 如存在，则指定扩展文件路径，如果没有，则指定为proto文件所在目录即可
        .compile(&["protos/voting.proto"], &["protos"])?;

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("protos")
        .compile(&["protos/hello.proto"], &["protos"])?;

    Ok(())
}
