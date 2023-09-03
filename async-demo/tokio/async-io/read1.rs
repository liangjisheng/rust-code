// AsyncReadExt提供了以下几个方法：
// read(): 读取数据并填充到指定的buf中
// read_buf(): 读取数据并追加到指定的buf中，和read()的区别稍后解释
// read_exact(): 尽可能地读取数据填充满buf，即按照buf长度来读取
// read_to_end(): 一直读取并不断填充到指定的Vec Buf中，直到读取时遇到EOF
// read_to_string(): 一直读取并不断填充到指定的String Buf中，直到读取时遇到EOF，要求所读取的是有效的UTF-8数据
// take(): 消费掉Reader并返回一个Take，Take限制了最多只能读取指定数量的数据
// chain(): 将多个Reader链接起来，读完一个目标的数据之后可以接着读取下一个目标

// a.log 其中有10个字节的数据 abcdefghij (没有换行符)

use tokio::{
    self,
    fs::File,
    io::{self, AsyncReadExt},
};

use bytes::BytesMut;

async fn read1() {
    // 打开文件，用于读取，因读取数据时需要更新文件指针位置，因此要加上mut
    let mut f = File::open("a.log").await.unwrap();

    // 提供buf，用来存放每次读取的数据，因要写入buf，因此加上mut
    // 读取的数据都是字节数据，即u8类型，因此为u8类型的数组或vec(或其它类型)
    let mut buf = [0u8; 5];
    // 使用 Vec 也是可以的
    // let mut vec_buf = vec![0u8, 5];

    // 读取数据，可能会等待，也可能立即返回，对于文件读取来说，会立即返回，
    // 每次读取数据都从buf的index=0处开始覆盖式写入到buf，
    // buf容量为5，因此一次读取最多5个字节。
    // 返回值为本次成功读取的字节数。
    let n = f.read(&mut buf).await.unwrap();
    // 由于只读取了n个字节保存在buf中，如果buf容量大于n，
    // 那么index=n和后面的数据不是本次读取的数据
    // 因此，截取buf到index=n处，这部分才是本次读取的数据
    let str = std::str::from_utf8(&buf[..n]);
    println!("first read {} bytes: {:?}", n, str);

    // 第二次读取5个字节，本次读取之后，a.log中的10个字节都已经被读取，
    let n = f.read(&mut buf).await.unwrap();
    // 因每次读取都将所读数据从buf的index=0处开始覆盖保存，
    // 因此，仍然是通过`&buf[0..n]`来获取本次读取的数据
    println!(
        "second read {} bytes: {:?}",
        n,
        std::str::from_utf8(&buf[..n])
    );

    // a.log中的数据在第二次read时已被读完，再次读取，将遇到EOF，
    // 遇到EOF时，read将返回Ok(0)表示读取的数据长度为0，
    // 但返回的长度为0不一定代表遇到了EOF，也可能是buf的容量为0
    let n = f.read(&mut buf).await.unwrap();
    // 因遇到EOF，没有读取到任何数据保存到buf，
    // 因此`&buf[..n]`为空slice，转换为字符串则为空字符串
    println!(
        "third read {} bytes: {:?}",
        n,
        std::str::from_utf8(&buf[..n])
    );
}

// read()每次将读取的数据从buf的index=0处开始覆盖时保存到buf。另一个方法read_buf()
// 则是追加式保存，这要求每次读取数据时都会自动维护buf的指针位置，因此直接使用数组和Vec
// 作为buf是不允许的。事实上，read_buf()参数要求了应使用实现了bytes::buf::BufMut Trait
// 的类型，它会在维护内部的位移指针，且在需要时会像vec一样自动扩容
// cargo add bytes
async fn read2() {
    let mut f = File::open("a.log").await.unwrap();
    // 初始容量为4
    let mut buf = BytesMut::with_capacity(4);

    // 第一次读取，读取容量大小的数据，即4字节数据，
    // 此时BytesMut内部的位移指针在offset = 3处
    let n = f.read_buf(&mut buf).await.unwrap();
    println!("first read {} bytes: {:?}", n, std::str::from_utf8(&buf));

    // 第二次读取，因buf已满，这次将一次性读取剩余所有数据(只请求一次读系统调用)，
    // BytesMut也将自动扩容以便存放更多数据，且可能会根据所读数据的多少进行多次扩容，
    // 所读数据都将从index=4处开始追加保存
    let n = f.read_buf(&mut buf).await.unwrap();
    println!("second read {} bytes: {:?}", n, std::str::from_utf8(&buf));
}

// read_exact()方法是根据buf的容量来决定读取多少字节的数据，和read()一样的是，
// 每次读取都会将所读数据从buf的index=0处开始覆盖(到了这里应该可以发现一点，
// 除非是内部自动维护buf位置的，都会从index=0处开始覆盖式存储)，和read()不一样的是，
// read_exact()明确了要读取多少字节的数据后(即buf的容量)，如果没有读取到这么多数据，
// 就会报错，比如提前遇到了EOF，将报 ErrorKind::UnexpectedEof 错误
async fn read3() {
    let mut f = File::open("a.log").await.unwrap();
    let mut buf = [0u8; 40];

    let n = f.read_exact(&mut buf).await.unwrap();
    println!(
        "first read {} bytes: {:?}",
        n,
        std::str::from_utf8(&buf[..n])
    );
}

// read_to_end()方法提供了一次性读取所有数据的功能，它会不断读取，直到遇到EOF
// 在不断读取的过程中，buf可能会进行多次扩容，因此buf不是固定大小的数组，而是Vec
// 这是非常好用的功能，不过对于文件的读取来说，tokio::fs::read()提供了更简单更
// 高效的一次性读取文件所有内容的方式。
// 另外需要注意的是，read_to_end()所读取的数据是在现有Vec数据的基础上进行追加的，
// 因此，Vec一定会有至少一次的扩容。
// 例如，a.log文件有10个字节数据，初始Vec buf容量为5，那么这10个数据将从Vec的
// index=5处开始追加存储
async fn read4() {
    let mut f = File::open("a.log").await.unwrap();
    // buf初始容量为5
    let mut buf = vec![0u8; 5];

    // read_to_end读取的数据，从buf的index=5处开始追加保存
    // 返回成功读取的字节数
    let n = f.read_to_end(&mut buf).await.unwrap();
    println!("first read {} bytes: {:?}", n, buf);
    println!(
        "first read {} bytes: {:?}",
        n,
        std::str::from_utf8(&buf[(buf.len() - n)..])
    );
}

// read_to_string()方法类似于read_to_end()，不同的是它将读取的字节数据直接解析为UTF-8字符串。
// 因此，该方法需指定String作为buf。同样的，read_to_string()所读取的数据会追加在当前String buf的尾部
async fn read5() {
    let mut f = File::open("a.log").await.unwrap();
    let mut buf = "xyz".to_string();

    let n = f.read_to_string(&mut buf).await.unwrap();
    println!("first read {} bytes: {:?}", n, buf);
}

// take()方法可限制最多只读取几个字节的数据，该方法会消费掉Reader，
// 并返回一个Take类型的实例。Take实例内部会保留原来的Reader，
// 并添加了一个限制接下来最多只能读取多少字节的字段limit_

// 当已读数据量达到了限制的数量后，下次再读，将强制遇到EOF，尽管这时候可能还没有
// 遇到内部Reader的EOF。不过，可以通过Take的set_limit()重新修改接下来可最多
// 读取的字节数，set_limit()会重置已读数量，相当于重新返回了一个新的Take实例。
// 当然，需要的时候可以通过remaining()来确定还可允许读取多少数量的数据。

// 例如，a.log文件有20字节的数据，先通过take()得到限制最多读取5字节Take，
// 通过Take读取2字节，再读取3字节，将遇到 EOF, 再通过 Take的set_limit()
// 修改限制再最多读取10字节
async fn read6() {
    let f = File::open("a.log").await.unwrap();
    let mut t = f.take(5);

    let mut buf = [0u8; 2];
    let n = t.read(&mut buf).await.unwrap();
    println!(
        "first read {} bytes: {:?}",
        n,
        std::str::from_utf8(&buf[..n])
    );

    let mut buf = [0u8; 3];
    let n = t.read(&mut buf).await.unwrap();
    println!(
        "second read {} bytes: {:?}",
        n,
        std::str::from_utf8(&buf[..n])
    );

    let mut buf = [0u8; 15];
    t.set_limit(10);
    let n = t.read(&mut buf).await.unwrap();
    println!(
        "third read {} bytes: {:?}",
        n,
        std::str::from_utf8(&buf[..n])
    );

    let n = t.read(&mut buf).await.unwrap();
    println!(
        "fourth read {} bytes: {:?}",
        n,
        std::str::from_utf8(&buf[..n])
    );
}

// 另外一个方法chain()，可将两个Reader串联起来(可多次串联)，当第一个Reader遇到EOF时，
// 继续读取将自动读取第二个Reader的数据。实际上，当第一个Reader遇到EOF时，串联后得到的
// Reader不会因此而遇到EOF，只是简单地将内部的done_first字段设置为true，表示第一个
// Reader已经处理完。只有第二个Reader遇到EOF时，串联后的Reader才遇到EOF。
// 多数时候用来读取多个文件的数据，当然，也可以将同一个文件串联多次
async fn read7() {
    let f1 = File::open("a.log").await.unwrap();
    let f2 = File::open("b.log").await.unwrap();
    let mut f = f1.chain(f2);

    let mut buf = [0u8; 20];
    let n = f.read(&mut buf).await.unwrap();
    println!("data {} bytes: {:?}", n, std::str::from_utf8(&buf[..n]));

    let n = f.read(&mut buf).await.unwrap();
    println!("data {} bytes: {:?}", n, std::str::from_utf8(&buf[..n]));
}

// 从上面示例的结果可知，虽然读完第一个Reader后chain Reader不会EOF，
// 但是读取却会在此停止，下次读取才会继续读取第二个Reader。
// 但如果使用 read_to_end() 或 read_to_string() 则会一次性读完所有数据，
// 因为这两个方法内部会多次读取直到遇到 EOF
async fn read8() {
    let f1 = File::open("a.log").await.unwrap();
    let f2 = File::open("b.log").await.unwrap();
    let mut f = f1.chain(f2);

    let mut data = String::new();
    let n = f.read_to_string(&mut data).await.unwrap();
    println!("data {} bytes: {:?}", n, data);
}

#[tokio::main]
async fn main() {
    // read1().await;
    // read2().await;
    // read3().await;
    // read4().await;
    // read5().await;
    // read6().await;
    // read7().await;
    read8().await;
}
