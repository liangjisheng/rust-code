// 表达式计算后有返回值，例如3+4是一个表达式，它返回计算结果7
// 与表达式对应的概念是语句，语句没有返回值或者不关心其返回值

// 可以在表达式结尾加上分号;来将表达式转换为【语句】
// 编译器发现表达式后有分号结尾时，在编译期间会自动修改代码，
// 它会在分号的后面加上一个小括号()。单独的小括号是一个特殊的值，表示什么也不做

fn e1() {
    3 + 4;
    // 实际等价于 3+4;()
}

// 带有分号表示这是一行Rust代码，Rust会先执行3+4得到7，然后忽略或丢弃该表达式的返回值7
// 再然后执行下一行代码，即一个单独的小括号，小括号表示什么也不做，直接跳过。
// 所以，代码3+4;从原本的表达式转变成了不关心返回值的【语句】

// 用于声明或定义的代码都是语句。例如let声明变量、fn定义函数、struct声明结构体等

// if 结构也是一个表达式，所以它有返回值，可以将if的返回值赋值给变量，
// 而它的返回值来自于它的大括号：当大括号最后执行的一条代码不加分号结尾时
// 该代码的计算结果就是if结构的返回值

fn main() {
    e1();

    let x = if true {
        println!("true");
        33 // 分支的最后一条代码计算结果赋值给x，不能分号结尾
    } else {
        println!("false");
        44 // 分支的最后一条代码计算结果赋值给x，不能分号结尾
    }; // 这个结尾分号表示let语句的结尾分号

    println!("x {}", x);
}
