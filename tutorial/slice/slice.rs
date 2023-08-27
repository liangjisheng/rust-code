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

fn main() {
    // slice1();
    // slice2();
    slice3();
}
