// 比较关键的是split()方法。TCP连接是全双工通信的，无论是TCP连接的客户端还是服务端，
// 每一端都可以进行读操作和写操作。为了方便描述，此处将其称为读端和写端。
// 即，客户端有读端和写端，服务端也有读端和写端。

// 通过TcpStream，可进行读操作，也可以进行写操作，正如前面几个示例代码所示。
// 但是，通过TcpStream同时进行读写有时候会很麻烦，甚至无解。很多时候，需要将
// TcpStream的读端和写端进行分离，然后将分离的读、写两端放进独立的异步任务中
// 去执行读或写操作(此时需跨线程)，即一个线程(或异步任务)负责读，另一个线程(或异步任务)负责写。

// split()和into_split()正是用来分离TcpStream的读写两端的。
// split()可将TcpStream分离为ReadHalf和WriteHalf，ReadHalf用于读，WriteHalf用于写

// let mut conn = TcpStream::connect("127.0.0.1:8888").await.unwrap();
// let (mut read_half, mut write_half) = conn.split();

// split()并没有真正将TcpStream的读写两端进行分离，仅仅只是引用TcpStream中的读端和写端。
// 因此，split()得到的读写两端只能在当前任务中进行读写操作，不允许跨线程跨任务。

// into_split()是split()的owned版，分离后可得到OwnedReadHalf和OwnedWriteHalf
// 它是真正地分离TcpStream的读写两端，它会消费掉TcpStream
// OwnedReadHalf和OwnedWriteHalf可跨任务进行读写操作

// let conn = TcpStream::connect("127.0.0.1:8888").await.unwrap();
// let (mut read_half, mut write_half) = conn.into_split();
