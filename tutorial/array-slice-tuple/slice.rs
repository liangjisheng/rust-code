// 切片就是指向一段 内存 的指针，因此切片可用于访问内存块中连续区间内的数据。
// Rust 语言中，在内存中连续区间存储数据的 数据结构 有: 数组、向量、字符串。
// 也就是说，切片可以和数组、 向量、字符串一起使用，它使用 数字索引 来访问它所指向的数据

// [start_index..end_index] 是一个左闭又开区间 [start_index,end_index)
// end_index 所表示的索引的字符并不包含在切片里面

fn slice1() {
    let n1 = "Tutorials".to_string();
    println!("length of string is {}", n1.len());
    let c1 = &n1[4..9];

    // fetches characters at 4,5,6,7, and 8 indexes
    println!("{}", c1);

    // 字符串s拥有三个元素：指向heap数据的指针、字符串长度、字符串容量
    // 字符串切片world拥有两个元素：指向heap数据起始位置的指针、切片长度
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}, {}", hello, world);

    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2];

    // 包含最后一个字节
    let len = s.len();
    let slice = &s[4..len];
    let slice = &s[4..];
    //截取完整的 String 切片
    let slice = &s[0..len];
    let slice = &s[..];
}

// 切片还可以作为函数的参数。使用切片可以把数组、向量、字符串中的连续子集通过引用的方式传递给函数
fn use_slice(slice: &[i32]) {
    // is taking a slice or borrowing a part of an array of i32s
    println!("length of slice is {:?}", slice.len());
    println!("{:?}", slice);
}

fn slice2() {
    let data = [10, 20, 30, 40, 50];
    use_slice(&data[1..4]);
    //this is effectively borrowing elements for a while
}

// 默认情况下 切片 是不可变更的。
// 虽然，看起来切片是指向原数据，但是默认情况下我们并不能改变切片的元素。
// 也就说默认情况下不能通过更改切片的元素来影响原数据。
// 但这不是绝对的，如果我们声明的原数据是可变的，同时定义切片的时候添加了 &mut 关键字，
// 那么我们就可以通过更改切片的元素来影响原数据

fn use_slice1(slice: &mut [i32]) {
    println!("切片的长度为：{:?}", slice.len());
    println!("{:?}", slice);
    slice[0] = 1010; // replaces 20 with 1010
}

fn slice3() {
    let mut data = [10, 20, 30, 40, 50];
    // passes references of
    // 20, 30 and 40
    use_slice1(&mut data[1..4]);
    println!("{:?}", data);
}

// 获取字符串中第一个单词
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn main() {
    slice1();
    // slice2();
    // slice3();
}

// 建议传给函数的字符串引用参数以切片的形式给出，即&str，这样可以保证函数
// 既可以接收到&String类型的参数，也可以接收&str类型的参数。
// 注意：&String和&str是两个不同的类型：
// &String 是字符串值的引用
// &str 是字符串值的切片

// 对于一个以 &str 为入参的函数：fn first_word(s: &str) -> &str { /*...*/ }
// 当入参是一个 &String 类型时，会创建一个完整的切片来调用该函数，即 &str_name[..]
// 当入参是一个字符串切片(&str)时，会直接调用该函数。
// 定义函数时使用字符串切片来代替字符串引用，可以是API更加通用，且不会损失任何功能
