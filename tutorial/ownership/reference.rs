// Rust 将这样创建一个变量的引用的这种行为称为 借用 (Borrowing)
// 即 Rust 允许一个引用借用一个变量的值来使用

// Rust 也不允许同时存在一个或多个可变引用和不可变引用。因为其它不可变引用不希望正在使用的时候，
// 所引用的值发生了改变，这与它本身的性质是冲突的

fn main() {
    let s1 = String::from("hello");
    let len = str_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut str = String::from("Imagine");
    change(&mut str);
    println!("str is {str}");

    // 不能同时借用多次可变变量
    // 这样做的目的是为了保证数据的安全性，防止发生数据竞争
    // let s1 = &mut str;
    // let s2 = &mut str;
    // println!("{} {}", s1, s2);

    // ref1();
    // ref2();
    ref3();
}

// 引用默认为不可变引用
fn str_length(s: &String) -> usize {
    s.len()
}

fn change(str: &mut String) {
    str.push_str("Miracle");
}

fn ref1() {
    let mut str = String::from("Imagine");

    {
        let s1 = &mut str; // 第一次创建可变引用

        s1.push_str("Miracle");

        println!("s1 = {}", s1);
    }
    // 2个可变引用不在同一个作用域下，可以创建多次
    let s2 = &mut str; // 第二次创建可变引用

    s2.push_str(" Say Hello!");

    println!("s2 = {}", s2);
}

fn ref2() {
    let mut str = String::from("Imagine");

    let s1 = &str;
    let s2 = &str;

    println!("{} {}", s1, s2);
    // 从此之后再也不用 s1 s2

    // 在定义可变引用之前使用过了不可变引用，而在创建可变引用之后，再未使用过之前的不可变引用
    let s3 = &mut str;
    s3.push_str("Miracle");
    println!("{}", s3);

    // 只要你们可变引用与不可变引用二者（两类引用）河水不犯井水，那么你们就可以共存。
    // 也就是说编译器会判断引用的生命周期，若两类引用的生命周期没有发生重叠，那么编译器将允许其共存

    // 引用作用域的结束位置从花括号变成最后一次使用的位置，因此 s1 借用和 s2 借用在 println! 后，
    // 就结束了，此时 s3 可以顺利借用到可变引用
}

fn ref3() {
    let mut a = 5;
    let mut b = &mut a;
    let c = &mut b;

    // 对c运算，需要手动解引用多次。但下面println!不需要，因为编译器可以自动识别并进行多次解引用
    **c += 10;

    println!("c = {:?}", c);
    println!("b = {:?}", b);
    println!("a = {:?}", a);
}
