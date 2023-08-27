fn main() {
    struct Person;

    let a = Person;
    let b = &a;
    // 对引用clone()时，将拷贝引用类型本身，而不是去拷贝引用所指向的数据本身，所以变量c的类型是&Person
    let c = b.clone(); // c的类型是&Person

    #[derive(Clone)]
    struct Person1;

    let a = Person1;
    let b = &a;
    let c = b.clone(); // c的类型是Person1，而不是&Person1

    // 前后两个示例的区别，仅在于引用所指向的类型Person有没有实现Clone
    // 没有实现Clone时，引用类型的clone()将等价于Copy，但 clippy 工具的错误提示说明这很可能不是我们想要的克隆效果
    // 实现了Clone时，引用类型的clone()将克隆并得到引用所指向的类型

    // 同一种类型的同一个方法，调用时却产生两种效果，之所以会有这样的区别，是因为
    // (1) 方法调用的符号.会自动解引用
    // (2) 方法调用前会先查找方法，查找方法时有优先级，找得到即停。
    // 由于解引用的前和后是两种类型(解引用前是引用类型，解引用后是引用指向的类型)，
    // 如果这两种类型都实现了同一个方法(比如clone())，
    // Rust编译器将按照方法查找规则来决定调用哪个类型上的方法
    // 参考(https://rustc-dev-guide.rust-lang.org/method-lookup.html?highlight=lookup#method-lookup)
}
