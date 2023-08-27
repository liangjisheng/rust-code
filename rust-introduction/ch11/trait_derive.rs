// 通过让某个类型去实现某个Trait，使得该类型具备该Trait的功能，是组合(composite)的方式。
// 经常和组合放在一起讨论的是继承(inheritance)。
// 继承通常用来描述属于同种性质的父子关系(is a)，而组合用来描述具有某功能(has a)

// Rust除了支持组合，还支持继承。但Rust只支持Trait之间的继承，比如Trait A继承Trait B
// 实现继承的方式很简单，在定义Trait A时使用冒号加上Trait B即可
// 如果Trait A继承Trait B，当类型C想要实现Trait A时，将要求同时也要去实现B

trait B {
    fn func_in_b(&self);
}

// Trait A 继承 Trait B
trait A: B {
    fn func_in_a(&self);
}

struct C {}

// C实现Trait A
impl A for C {
    fn func_in_a(&self) {
        println!("impl: func_in_a");
    }
}

// C还要实现Trait B
impl B for C {
    fn func_in_b(&self) {
        println!("impl: func_in_b");
    }
}

fn main() {
    let c = C {};
    c.func_in_a();
    c.func_in_b();
}
