// 异步 IO 主要应用在文件处理、网络处理等场合，而这些场合的数据结构都已经实现了对应的接口
// 比如 File 或者 TcpStream，它们也已经实现了 AsyncRead / AsyncWrite，所以基本上
// 不用自己实现异步 IO 接口。不过有些情况，可能会把已有的数据结构封装在自己的数据结构中，
// 此时需要自己实现相应的异步 IO 接口

use anyhow::Result;
use pin_project::pin_project;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio::{
    fs::File,
    io::{AsyncRead, AsyncReadExt, ReadBuf},
};

#[pin_project]
struct FileWrapper {
    #[pin]
    file: File,
}

impl FileWrapper {
    pub async fn try_new(name: &str) -> Result<Self> {
        let file = File::open(name).await?;
        Ok(Self { file })
    }
}

impl AsyncRead for FileWrapper {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        self.project().file.poll_read(cx, buf)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut file = FileWrapper::try_new("./Cargo.toml").await?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).await?;
    println!("{}", buffer);
    Ok(())
}
