// 数组的定义其实就是为分配一段 连续的相同数据类型 的内存块
// 数组是静态的
// 这意味着一旦定义和初始化，则永远不可更改它的长度
// 数组的元素有着相同的数据类型，每一个元素都独占者数据类型大小的内存块。
// 数组的内存大小等于数组的长度乘以数组的数据类型。
// 数组中的每一个元素都按照顺序依次存储，这个顺序号既代表着元素的存储位置
// 也是数组元素的唯一标识。我们把这个标识称之为 数组下标

fn a1() {
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

fn main() {
    // a1();
    t1();
}
