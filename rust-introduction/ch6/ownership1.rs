// 容器类型中可能包含栈中数据值(特指实现了Copy的类型)，也可能包含堆中数据值(特指未实现Copy的类型)
// 容器变量拥有容器中所有元素值的所有权

// let tup = (5, String::from("hello"));
// 当上面tup的第二个元素的所有权转移之后，tup将不再拥有它的所有权，
// 这个元素将不可使用，tup自身也不可使用，但仍然可以使用tup的第一个元素

fn o1() {
    let tup = (5, String::from("hello"));

    // 5 拷贝后赋值给x，tup仍有该元素的所有权
    // 字符串所有权转移给y，tup丢失该元素所有权
    let (x, y) = tup;
    println!("{},{}", x, y); // 正确
    println!("{}", tup.0); // 正确

    // println!("{}", tup.1); // 错误
    // println!("{:?}", tup); // 错误
}

fn main() {
    // o1();

    let tup = (5, String::from("hello"));
    // 如果想要让原始容器变量继续可用，要么忽略那些没有实现Copy的堆中数据
    // 要么clone()拷贝堆中数据后再borrow，又或者可以引用该元素

    // 方式一：忽略
    let (x, _) = tup;
    println!("{}", tup.1); //  正确

    // 方式二：clone
    let (x, y) = tup.clone();
    println!("{}", tup.1); //  正确

    // 方式三：引用
    let (x, ref y) = tup;
    println!("{}", tup.1); //  正确
}
