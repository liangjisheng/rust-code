// 不同于Rust中的泛型参数，程序员是可以手动指定的。Rust的生命周期是不能手动指定的，
// 需要编译器根据传入的参数进行推断。当编译器在某条语句上不能根据参数进行推断时，
// 他会继续往下执行并推断生命周期参数。编译器会持续根据语句上下文推断出生命周期参数，并选择最小的那个

struct Context<'a> {
    vars: Vec<&'a str>,
}

fn s1() {
    let mut v = Context { vars: vec![] };
    v.vars.push("hello");

    println!("{:?}", v.vars);
}

// 对于上述程序片段来说，当运行let mut v = Context { vars: vec![] }的时候，
// 此时'a并不能推断出来，于是编译器继续查看下一条语句，
// 当看到v.vars.push("hello")时，编译器推断出'a为'static

// fn s2() {
//     let mut v = Context { vars: vec![] };
//     v.vars.push("hello"); // 'a
//
//     {
//         let s = String::from("dd"); // 'b
//         v.vars.push(&s);
//     }
//     println!("{:?}", v.vars);
// }

// 编译器首先会在v.vars.push("hello")推断出'a为'static，然后当运行到里面的大括号的时候
// 发现v.vars.push(&s)，而s的生命周期为'b，此时编译器发现两次推断不一样，
// 于是他会选择生命周期较小的那个'b。而此时v的生命周期大于'b，编译器会报错
// 结构体实例的生命周期不能大于里面引用字段的生命周期, 如果大于的话会造成引用字段为垂悬引用

// 警惕Self的隐含生命周期参数
// 对于impl中的方法来说，self有一个隐含的生命周期参数

struct A<'a> {
    name: &'a str,
}

// impl<'a> A<'a> {
//     fn get(&self) -> &str {
//         self.name
//     }
// }

fn s3() {
    let s = String::from("hello");
    let s_ref;

    {
        let a = A { name: &s };
        s_ref = a.get();
    }
    println!("{:?}", s_ref);
}

// 对于A的get方法来说，&self有一个隐含的生命周期参数，这个生命周期就是实例化A所在的区域
// 如果返回的&str不写生命周期参数，根据生命周期省略原则，返回的参数将会和&self一样的生命周期
// 在上述示例中，返回的&str的生命周期明显大于self的生命周期。
// 但是在这里返回的str将会限制在A的实例所在的生命周期内。
// 当A的实例a脱离内部作用区域时，s_ref生命周期就结束了，也不能被引用了

// 正确的是我们显式声明我们的返回值的生命周期为'a，于是一切都正常了
// 此时 s_ref 的生命周期就扩充到了外部作用域，println就可以正常打印了

impl<'a> A<'a> {
    fn get(&self) -> &'a str {
        self.name
    }

    // 有的时候，我们希望内部的引用变量和self具有相同的生命周期
    // 此时我们需要显示声明self的生命周期参数
    fn set(&'a self, name: &'a str) -> A<'a> {
        A { name }
    }
}

fn main() {
    // s1();
    // s2();
    s3();
}
