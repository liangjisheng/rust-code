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
    t1();
}
