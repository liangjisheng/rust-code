// Rust也支持Slice操作，Rust中的切片操作只允许获取一段连续的局部数据，切片操作获取到的数据称为切片数据
// 切片操作允许使用usize类型的变量作为切片的边界。例如，n是一个usize类型的变量，那么s[..n]是允许的切片操作
// Slice类型是一个胖指针，它包含两份元数据：
// 第一份元数据是指向源数据中切片起点元素的指针
// 第二份元数据是切片数据中包含的元素数量，即切片的长度

// Slice类型的描述方式为[T]
// 由于切片数据的长度无法在编译期间得到确认(比如切片操作的边界是变量时s[..n])
// 而编译器是不允许使用大小不定的数据类型的，因此无法直接去使用切片数据(比如无法直接将它赋值给变量)
// 也因此，在Rust中几乎总是使用切片数据的引用。切片数据的引用对应的数据类型描述为&[T]或&mut [T]
// 前者不可通过Slice引用来修改源数据，后者可修改源数据。

// 注意区分Slice类型和数组类型的描述方式
// 数组类型表示为[T; N]，数组的引用类型表示为&[T; N]，Slice类型表示为[T]，Slice的引用类型表示为&[T]

fn s1() {
    let _arr = [11, 22, 33, 44, 55];
    let _n: usize = 3;

    // 编译错误，无法直接使用切片类型
    // let arr_s = arr[0..n];

    let mut arr = [11, 22, 33, 44];

    // 不可变slice
    let arr_slice1 = &arr[..=1];
    println!("{:?}", arr_slice1); // [11,22];

    // 可变slice
    let arr_slice2 = &mut arr[..=1];
    arr_slice2[0] = 1111;
    println!("{:?}", arr_slice2); // [1111,22];
    println!("{:?}", arr); // [1111,22,33,44];

    // 需要特别注意的是，String的切片和普通的切片有些不同。
    // 一方面，String的切片类型是str，而非[String]，String 切片的引用是 &str 而非 &[String]
    // 另一方面，Rust为了保证字符串总是有效的Unicode字符，它不允许用户直接修改字符串中的字符，
    // 所以也无法通过切片引用来修改源字符串，除非那是ASCII字符(ASCII字符总是有效的unicode字符)。
    // 事实上，Rust只为&str提供了两个转换ASCII大小写的方法来修改源字符串，除此之外，
    // 没有为字符串切片类型提供任何其他原地修改字符串的方法
    let mut s = String::from("HELLO");
    let ss = &mut s[..];

    // make_ascii_lowercase()
    // make_ascii_uppercase()
    ss.make_ascii_lowercase();
    println!("{}", s); // hello

    // 经常会看到将Array的引用&[T;n]当作Slice来使用
    let arr = [11, 22, 33, 44];
    let slice = &arr; // &arr 将自动转换为 slice 类型

    // 调用slice类型的方法first()返回slice的第一个元素
    println!("{}", slice.first().unwrap()); // 11

    // 可以直接将数组的引用当成slice来使用。即&arr和&mut arr当作不可变slice和可变slice来使用
    // 在调用方法的时候，由于.操作符会自动创建引用或解除引用，因此Array可以直接调用Slice的所有方法
}

fn s2() {
    // String Slice可用的方法较少，上面给出官方手册中，除了方法名中有"ascii"的方法
    // (如is_ascii()方法)是String Slice可使用的方法外，其他方法都不能被String Slice调用
    let arr = [11, 22, 33];
    println!("{}", arr.len()); // 3
    println!("{:?}", arr.repeat(2)); // [11, 22, 33, 11, 22, 33]
    println!("{:?}", arr.contains(&22)); // true

    // reverse()
    let mut arr = [11, 22, 33];
    arr.reverse();
    println!("{:?}", arr); // [33,22,11]

    // join()
    println!("{}", ["alice", "bob"].join(" ")); // alice bob
    println!("{:?}", [[1, 2], [3, 4]].join(&0)); // [1,2,0,3,4]

    // swap()
    let mut arr = [1, 2, 3, 4];
    arr.swap(1, 2);
    println!("{:?}", arr); // [1,3,2,4]

    // windows()
    let arr = [10, 20, 30, 40];
    for i in arr.windows(2) {
        println!("{:?}", i); // [10,20], [20,30], [30,40]
    }

    // starts_with()，相关的方法还有ens_with()
    let arr = [10, 20, 30, 40];
    println!("{}", arr.starts_with(&[10])); // true
    println!("{}", arr.starts_with(&[10, 20])); // true
    println!("{}", arr.starts_with(&[30])); // false
}

fn main() {
    // s1();
    s2();
}
