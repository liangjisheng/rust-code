use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut reader: &[u8] = b"hello";
    let mut writer: Vec<u8> = vec![];

    io::copy(&mut reader, &mut writer).await?;

    assert_eq!(&b"hello"[..], &writer[..]);
    Ok(())
}
