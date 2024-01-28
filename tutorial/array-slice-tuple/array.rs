// 数组的定义其实就是为分配一段 连续的相同数据类型 的内存块
// 数组是静态的
// 这意味着一旦定义和初始化，则永远不可更改它的长度
// 数组的元素有着相同的数据类型，每一个元素都独占者数据类型大小的内存块。
// 数组的内存大小等于数组的长度乘以数组的数据类型。
// 数组中的每一个元素都按照顺序依次存储，这个顺序号既代表着元素的存储位置
// 也是数组元素的唯一标识。我们把这个标识称之为 数组下标

fn array1() {
    // 与元组不同的是数组中的所有元素类型必须一致，Rust 中的 Array 与其它语言不太一样
    // 因为其 Array 的长度是固定的和元组一样
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr[0]: {}, arr[4]: {}", arr[0], arr[1]); // arr[0]: 1, arr[4]: 2

    // 省略数组类型的语法
    let arr = [10, 20, 30, 40];
    println!("arr: {:?}, len: {}", arr, arr.len());

    // 因为数组的长度在编译时就时已知的，因此我们可以使用 for ... in 语句来遍历数组
    let len = arr.len();
    for i in 0..len {
        println!("index: {}, value: {}", i, arr[i]);
    }

    // 使用 iter() 函数为数组生成一个迭代器。然后就可以使用 for in 语法来迭代数组
    for val in arr.iter() {
        println!("value: {}", val);
    }

    // 指定默认初始值的语法，这种语法有时候称为 默认值初始化。
    // 如果不想为每一个元素指定初始值，则可以为所有元素指定一个默认的初始值
    let arr: [i32; 4] = [-1; 4];
    println!("arr: {:?}, len: {}", arr, arr.len());

    // 使用 let 声明的变量，默认是只读的，数组也不例外。也就是说，默认情况下，数组是不可变的
    let mut arr: [i32; 4] = [10, 20, 30, 40];
    arr[1] = 0;
    println!("{:?}", arr);

    let mut arr = [10, 20, 30];
    update_value(arr);
    println!("{:?}", arr);

    update_ref(&mut arr);
    println!("{:?}", arr);

    // 声明数组时长度必须指定为整数字面量或者整数常量。如果数组长度是一个变量，则会报编译错误
    // let N: usize = 20;
    // let arr = [0; N]; //错误: A non-constant value was used in a constant expression
    // 报错的原因是： N 不是一个常量。
    // 注意，虽然 N 默认是只读的，但它仍然是一个变量，只不过是一个只读变量而已，
    // 只读变量不是常量。因为变量的值是在运行时确定的，而常量的值是在编译器确定的
    // print!("{}", arr[10]);

    const N: usize = 20;
    // 固定大小
    let arr = [0; N];
    println!("{}", arr[10]);
}

// 值传递
fn update_value(mut arr: [i32; 3]) {
    for i in 0..3 {
        arr[i] = 0;
    }
    println!("Inside update {:?}", arr);
}

// 引用传递
fn update_ref(arr: &mut [i32; 3]) {
    for i in 0..3 {
        arr[i] = 0;
    }
    println!("Inside update {:?}", arr);
}

fn array2() {
    // String 没有实现 Copy trait, 不能这样写
    // let array = [String::from("rust is good!"); 8];

    let array = [
        String::from("rust is good!"),
        String::from("rust is good!"),
        String::from("rust is good!"),
    ];
    println!("{:#?}", array);

    //优化后写法
    let array: [String; 8] = std::array::from_fn(|_i| String::from("rust is good!"));
    println!("{:#?}", array);

    //数组切片
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    // 切片的长度可以与数组不同，并不是固定的，而是取决于你使用时指定的起始和结束位置
    // 创建切片的代价非常小，因为切片只是针对底层数组的一个引用
    // 切片类型[T]拥有不固定的大小，而切片引用类型&[T]则具有固定的大小，因为 Rust
    // 很多时候都需要固定大小数据类型，因此&[T]更有用,&str字符串切片也同理
}

fn array3() {
    // 编译器自动推导出one的类型
    let one = [1, 2, 3];
    // 显式类型标注
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];

    // arrays是一个二维数组，其中每一个元素都是一个数组，元素类型是[u8; 3]
    let arrays: [[u8; 3]; 4] = [one, two, blank1, blank2];

    // 借用arrays的元素用作循环中
    for a in &arrays {
        print!("{:?}: ", a);
        // 将a变成一个迭代器，用于循环
        // 你也可以直接用for n in a {}来进行循环
        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n + 10);
        }

        let mut sum = 0;
        // 0..a.len,是一个 Rust 的语法糖，其实就等于一个数组，元素是从0,1,2一直增加到到a.len-1
        for i in 0..a.len() {
            sum += a[i];
        }
        println!("\t({:?} = {})", a, sum);
    }
}

// 数组类型容易跟数组切片混淆，[T;n]描述了一个数组的类型，而[T]描述了切片的类型，
// 因为切片是运行期的数据结构，它的长度无法在编译期得知，因此不能用[T;n]的形式去描述
// [u8; 3]和[u8; 4]是不同的类型，数组的长度也是类型的一部分
// 在实际开发中，使用最多的是数组切片[T]，我们往往通过引用的方式去使用&[T]，因为后者
// 有固定的类型大小

fn array4() {
    let arr = [11, 22, 33, 44, 55];
    let s1 = &arr[0..3]; // [11,22,33]
    let s2 = &arr[1..=3]; // [22, 33, 44]
    let s3 = &arr[..]; // [11, 22, 33, 44, 55]

    println!("{:?}", arr);
    println!("{:?}", s1);
    println!("{:?}", s2);
    println!("{:?}", s3);

    for i in 1..5 {
        println!("{}", i); // 1 2 3 4
    }

    // 下面两个表示范围的方式是等价的
    // let x = 0..5;
    let y = std::ops::Range { start: 0, end: 5 };
    println!("{:?}", y);

    assert_eq!((3..5), std::ops::Range { start: 3, end: 5 });
    assert_eq!(3 + 4 + 5, (3..6).sum());
}

// 元组是将多个不同数值组合为一个复合类型的常见方法，元组拥有固定长度，一旦声明无法更改
// 我们通过解构的方式，分别从声明的元组中取出数据
// 因为元组是一个 复合类型，要输出复合类型的数据，必须使用 println!("{:?}", tuple_name)

fn t1() {
    let tup: (i32, f64, char) = (1, 1.01, ' ');
    println!("tup: {:?}", tup);
    // 元组解构赋值
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z); // x: 1, y: 1.01, z:

    let tup1: (i32, bool, f64) = (1, true, 1.1);
    print(tup1);

    // 可通过数值的索引来访问元组中的数据
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;
    println!("x: {}, y: {}, z: {}", x, y, z); // x: 1, y: 1.01, z:
}

// 元组也可以作为函数的参数
fn print(x: (i32, bool, f64)) {
    println!("Inside print method");
    println!("{:?}", x);
}

// 可以使用元组返回多个值
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn main() {
    // array1();
    // array2();
    array3();
}
