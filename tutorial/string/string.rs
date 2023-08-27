// Rust 语言提供了两种字符串
// Rust 核心内置的数据类型 &str，字符串字面量
// Rust 标准库中的一个公开 pub 结构体，字符串对象 String

// 字符串字面量的核心代码可以在模块 std::str 中找到。Rust 中的字符串字面量被称之为字符串切片。
// 因为它的底层实现是切片
// 字面量 &str 就是在编译时就知道其值的字符串类型，它也是字符的集合，被硬编码赋值给一个变量
// 字符串字面量模式是静态的，所以，字符串字面量从创建时开始会一直保存到程序结束

// 字符串对象并不是 Rust 核心内置的数据类型，它只是标准库中的一个公开pub的结构体
// 字符串对象是使用 UTF-8 作为底层数据编码格式,长度可变的集合
// 字符串对象在堆 heap 中分配，可以在运行时提供字符串值以及相应的操作方法

fn f1() {
    let s = "hello world";
    // 变量 s 的类型是 &str，它是一个指向二进制程序特定位置的切片
    // &str 是不可变引用，所以字符串字面值也是不可变的
    println!("s: {}", s);
    // 将字符串转换为字符串对象
    let s = s.to_string();
    println!("s: {}", s);
    // 字符串对象转换为字符串字面量
    let s = s.as_str();
    println!("s: {}", s);

    let s1 = String::from("test");
    println!("s1: {}, len: {}", s1, s1.len());

    let mut s2 = String::new();
    s2.push_str("s2");
    println!("s2: {}, len: {}", s2, s2.len());

    // str::trim() 去除字符串前后空格或回车或换行
    let mut str = "   @@ImagineMiracle##   **  \n\n\n";
    // println!("{}", str);
    str = str.trim();
    println!("{}", str);

    // 字符串字面量默认是 静态 的，我们也可以主动添加 static 关键字。只不过语法格式有点怪，所以通常被忽略
    let company: &'static str = "编程教程";
    let location: &'static str = "中国";
    println!("公司名 : {} 位于 :{}", company, location);

    let name1 = company.to_string();
    let name2 = name1.replace("程", "cheng");
    println!("name2: {}", name2);

    let mut s3 = "rus".to_string();
    // 追加字符
    s3.push('t');
    println!("s3: {}", s3);
    // 追加字符串
    s3.push_str(" rust");
    println!("s3: {}", s3);

    // len() 方法会统计所有的字符，包括空白符
    // 空白符是指 制表符 \t、空格 、回车 \r、换行 \n 和回车换行 \r\n 等等
    println!("s3 len: {}", s3.len());

    s3 = " \t rust rust \r\n ".to_string();
    // 去除字符串头尾的空白符 trim()
    println!("s3: {}", s3.trim());

    // 使用空白符分割字符串 split_whitespace()
    s3 = " \t rust rust \r\n ".to_string();
    let mut i = 1;
    for token in s3.split_whitespace() {
        println!("token {} {}", i, token);
        i += 1;
    }

    let mut iter = "A few words".split_whitespace();
    assert_eq!(Some("A"), iter.next());
    assert_eq!(Some("few"), iter.next());
    assert_eq!(Some("words"), iter.next());

    assert_eq!(None, iter.next());

    // 根据指定模式分割字符串 split()
    // split() 会根据传递的指定 模式 （字符串分割符） 来分割字符串，
    // 并返回分割后的字符串子串组成的切片上的迭代器。我们可以通过这个迭代器来迭代分割的字符串子串。
    // split() 方法最大的缺点是不可重入迭代，也就是迭代器一旦使用，则需要重新调用才可以再用。
    // 但我们可以先在迭代器上调用 collect() 方法将迭代器转换为 向量 Vector ，这样就可以重复使用了
    let fullname = "李白，诗仙，唐朝";
    for token in fullname.split("，") {
        println!("token is {}", token);
    }

    // 存储在一个向量中
    println!();
    let tokens: Vec<&str> = fullname.split("，").collect();
    println!("姓名 is {}", tokens[0]);
    println!("称号 {}", tokens[1]);
    println!("朝代 {}", tokens[2]);

    // 将字符串打散为字符数组 chars()
    let s3 = "rust".to_string();
    for c in s3.chars() {
        println!("{}", c);
    }

    let s4 = "rust".to_string();
    let s5 = " rust".to_string();
    let s6 = s4 + &s5; // 需要 s5 的引用
    println!("s6: {}", s6);

    // 如果需要将其它类型转换为字符串类型，可以直接调用 to_string() 方法
    let n1 = 2023;
    let n1_as_string = n1.to_string();
    println!("n1_as_string: {}", n1_as_string);
    println!("{}", n1_as_string == "2023");

    // 如果要把不同的变量或对象拼接成一个字符串，我们可以使用 格式化宏 ( format! )
    let s7 = "rust".to_string();
    let s8 = format!("{} {}", s7, n1);
    println!("s8: {}", s8);
}

fn f2() {
    let str = "27149";

    // 使用 match 处理 Result
    let num: u32 = match str.parse::<u32>() {
        Ok(n1) => n1,
        Err(_) => {
            println!("Error.");

            0
        }
    };

    println!("str to num: {str} -> {num}");

    // 使用 expect 处理 Result
    let num: u32 = str.parse::<u32>().expect("Failed to u32");
    println!("str to num: {str} -> {num}");

    // 使用 unwrap 处理 Result，出错后将会程序将会报 panic 错误同时崩溃并退出
    let num: u32 = str.parse().unwrap();
    println!("str to num: {str} -> {num}");
}

fn f3() {
    // 在 Rust 中除了 String 类型，还存在一种字符串类型 str，str 被称为字符串切片 (slice) 类型
    // 所谓的字符串切片 (slice)，实际上是一个字符串的部分或全部引用，由于其是 String 的引用因此类型前也需要加上 & 表示引用
    let s: String = String::from("ImagineMiracle");

    let s1: &str = &s[0..7];
    let s2: &str = &s[7..s.len()];

    println!("s = {}\ns1 = {}\ns2 = {}", s, s1, s2);

    // &s[0...7]、&s[7...14] 分别表示，截取从 0 号到 6 号索引的内容，截取从 7 号到 13 号索引的内容
    // &s[...7] 表示截取从开始到 6 号索引的内容，与 &s[0..7] 相同
    // &s[7...] 表示截取从 7 号索引到结尾的所有内容，与 &s[7..14] 相同
    // &s[0...14]、&s[0...s.len()]、&s[...s.len()]、&s[...] 均表示截取整个字符串
}

fn f4() {
    // 回归到字符串常量，为什么它不能被修改呢。我们都知道的是，由于 ImagineMiracle 这段字符串会被
    // 编译器放进编译后的二进制文件的只读数据段 (.rodata)，因此不能被修改，这个分析也是没问题的

    // 一个字符串常量原来是对一个常规字符串值的引用。默认的引用是不允许修改其值的，
    // 这也就回答了 Rust 是如何判断字符串常量不允许被修改的原因。因为它就是一个不可变的引用
    let str_s = "ImagineMiracle-01 str";

    let str = String::from("ImagineMiracle-02 String");

    print_str(str_s);
    print_str(&str_s[space_item(str_s)..str_s.len()]);
    print_str(&str);
    print_str(&str[space_item(&str)..str.len()]);

    let str_s = put_str(str_s);
    print_str(str_s);

    let str = put_str(&str);
    print_str(&str);
}

fn print_str(str: &str) {
    println!("{}", str);
}

fn put_str(s: &str) -> &str {
    &s[7..14]
}

fn space_item(str: &str) -> usize {
    let bytes = str.as_bytes();

    for i in 0..str.len() {
        if bytes[i] == b' ' {
            return i + 1;
        }
    }

    str.len()
}

fn f5() {
    // 有的字符串字面值都拥有 'static 生命周期，我们也可以选择像下面这样显式标注出来
    let s: &'static str = "I have a static lifetime.";
    println!("s {}", s);
    // s 等价于下面的这个
    let s1 = "I have a static lifetime.";
    println!("s1 {}", s1);
}

fn f6() {
    let s: &str = "test".into();
    println!("s: {}", s);
    let s1: String = "this".into();
    println!("s1: {}", s1);

    let mut chars = s1.chars();
    assert_eq!(Some('t'), chars.next());
    assert_eq!(Some('h'), chars.next());
    assert_eq!(Some('i'), chars.next());
    assert_eq!(Some('s'), chars.next());
    assert_eq!(None, chars.next());

    let s2 = s1 + " that";
    println!("s2: {}", s2);
    // 不能用索引访问 String。其实道理很简单，因为有些字符的编码可能是多个字节
    // String[index] 这种形式访问 String 中的字符，不好处理。
    // 因此， Rust 不支持用索引访问 String 中的字符
    // println!("s2: {}", s2[0]);

    // 返回迭代器
    for i in s2.chars() {
        print!("{} ", i);
    }
    println!();
    for i in s2.bytes() {
        print!("{} ", i);
    }
    println!();

    // 如果有合法的 UTF-8 字节数组，可以用它创建 String 变量
    let sparkle_heart = vec![240, 159, 146, 150];
    // 因为这些字节数据是合法的，所以直接使用 unwrap() 解包
    let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();
    assert_eq!("💖", sparkle_heart);
    println!("sparkle_heart: {}", sparkle_heart);

    let bytes = sparkle_heart.into_bytes();
    assert_eq!(bytes, [240, 159, 146, 150]);

    // String 总是 “有效的” UTF-8。如果你需要一个非UTF-8字符串，
    // 则考虑使用 OsString 。它是类似的，但是没有uTF-8约束
}

// String 实现了 Deref<Target=str>，因此继承了 str 的所有方法
// 这意味着，函数中 &str 类型的参数都可以接受 &String 类型的变量

// 在某些情况下，Rust没有足够的信息进行此转换，称为 Deref 强制
// 在下面的示例中，字符串片段&str 实现了 TraitExample，函数
// example_func 接受实现该特性的任何内容。在这种情况下，由于 String 没有实现特性
// TraitExample，没办法直接把 String 类型的变量传递给函数 example_func
trait TraitExample {}
impl<'a> TraitExample for &'a str {}

fn example_func<A: TraitExample>(example_arg: A) {}

fn f7() {
    fn takes_str(s: &str) {
        println!("{}", s);
    }
    let s = String::from("Hello");
    takes_str(&s);

    let example_string = String::from("example_string");
    // s1 为 str 类型, compile error: doesn't have a size known at compile-time
    // let s1 = *example_string;
    // println!("{}", s1);

    // example_func(&example_string); // 编译 error

    // 编译 ok
    example_func(example_string.as_str());
    // 首先把 String 类型解引用成 [`str`] 类型，然后再通过引用 [`&str`] 类型
    example_func(&*example_string);
}

fn f8() {
    use std::mem;

    let story = String::from("Once upon a time...");

    // FIXME Update this when vec_into_raw_parts is stabilized

    // 防止自动释放字符串数据
    let mut story = mem::ManuallyDrop::new(story);

    let ptr = story.as_mut_ptr();
    let len = story.len();
    let capacity = story.capacity();
    println!("ptr: {:?}", ptr);
    println!("len: {}", len);
    println!("capacity: {}", capacity);

    // story 有 19 个字节的数据
    assert_eq!(19, len);

    // 可以用 ptr、 len、 和 capacity 重新构建 String。这个过程是不安全的，
    // 我们必须保证自己的代码的可靠性。
    let s = unsafe { String::from_raw_parts(ptr, len, capacity) };
    assert_eq!(String::from("Once upon a time..."), s);

    let mut s1 = String::new();
    println!("{}", s1.capacity());
    for _ in 0..5 {
        s1.push_str("hello");
        println!("{}", s1.capacity());
    }
    println!();

    // 可以用 with_capacity 方法申请一定数量的初始内存
    let mut s2 = String::with_capacity(25);
    println!("{}", s2.capacity());
    for _ in 0..5 {
        s2.push_str("hello");
        println!("{}", s2.capacity());
    }
}

fn main() {
    // f1();
    // f2();
    // f3();
    // f4();
    // f5();
    // f6();
    // f7();
    f8();
}
