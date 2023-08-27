// turbo: 涡轮，涡轮增压发动机
// charge: v. 充电，指责，责备，要价； n. 电量，价钱，控告
// turbo charger 涡轮增压器

// https://www.jianshu.com/p/9107685ece03
// https://www.jianshu.com/p/1dd0dbbf539b
// https://matematikaadit.github.io/posts/rust-turbofish.html

// 有时候，我们希望更明确解析、或者创建的数据类型，
// 所以需要 turbofish 语法 ::<> 来指定泛型的具体类型

fn t1() {
    // a 不起作用，因为它不能推断变量类型
    // let a = (0..255).sum(); //error, cannot infer type
    let b = (0..255).sum::<u32>();
    let c: u32 = (0..255).sum();

    // let num = (0..9);
    // 编写_以让编译器自动推断类型
    // 将元组中的数据收集到 vec 中,默认是 i32, 也可以指定为 u64 或者其他的类型
    let vec_i32 = (0..10).collect::<Vec<_>>();
    let vec_u64 = (0..10).collect::<Vec<u64>>();
    println!("{:?}\n{:?}", vec_i32, vec_u64);

    // 开辟一个容量为 8 的vec
    let _vec_string = Vec::<String>::with_capacity(8);
    let _vec_u8 = Vec::<u8>::with_capacity(8);
}

fn t2() {
    // 编译器不知道 vec 的元素类型, 报错
    // type annotations needed for `Vec<T>`
    // let v = Vec::new();
    // println!("{}", v);

    // 显式指定类型
    let v: Vec<bool> = Vec::new();
    println!("{:?}", v);

    // 通过一个叫做 turbofish ::<> 的语法来绑定泛型参数 T
    let v = Vec::<bool>::new();
    println!("{:?}", v);
}

use std::mem::size_of;

fn t3() {
    // u8 字节数
    println!("{}", size_of::<u8>());
    // u32 字节数
    println!("{}", size_of::<u32>());
    // i32 字节数
    println!("{}", size_of::<i32>());

    // 使用 turbofish 来告诉应该从str解析的类型
    let r_v = "1024".parse::<u32>();
    let v = r_v.unwrap_or(0);
    println!("v {}", v);
}

// 枚举的 turbofish 语法不同于我们的经验法则
// RUST的枚举 turbofish 限定没有放在枚举类型后面，面是放在枚举变量之后
fn t4() {
    let o = Result::Ok::<u8, ()>(10);
    let e = Result::Err::<u8, ()>(());
    println!("o {:?}", o);
    println!("e {:?}", e);

    // 也可以省略 Result
    let o1 = Ok::<u8, ()>(10);
    let e1 = Err::<u8, ()>(());
}

fn main() {
    // t1();
    // t2();
    // t3();
    t4();
}
