// https://juejin.cn/post/7206732474113998903

// 闭包在Rust中的实现可以近似地理解为一个实现了FnOnce、FnMut和Fn其中一个trait的匿名结构体
// 这个匿名结构体保存捕获的环境中的变量。通过调用trait的方法来执行闭包体中的代码

// 闭包实现这三个trait的规则如下
// 所有的闭包都实现了FnOnce
// 如果闭包的方法移出了所捕获的变量的所有权，则只会实现FnOnce
// 如果闭包的方法没有移出所捕获的变量的所有权，并且对变量进行了修改，即通过可变借用使用所捕获的变量，则会实现FnMut
// 如果闭包的方法没有移出所捕获的变量的所有权，并且没有对变量进行修改，即通过不可变借用使用所捕获的变量，则会实现Fn

// 闭包实现FnOnce、FnMut和Fn中的哪个trait只与闭包如何使用所捕获的变量有关，与如何捕获变量无关
// 关键字move影响的是闭包如何捕获变量，因此，对闭包实现FnOnce、FnMut和Fn没有任何影响

// 关键字move主要用于使闭包摆脱所捕获的变量的生命周期限制，例如将闭包返回或移至其他线程时，必须使用move

// 由于在使用关键字move后，闭包捕获的变量的所有权会发生变化，
// 因此会对闭包产生另外一个影响，即闭包本身是否会实现Copy trait?
// 闭包捕获的变量为不可变引用&T或Copy语义的T时，闭包会实现Copy trait
// 闭包捕获的变量为可变引用&mut T或Move语义的T时，则闭包不会实现Copy trait

#[derive(Copy, Clone)]
struct FooCopy {
    value: i32,
}

impl FooCopy {
    fn new(value: i32) -> Self {
        Self { value }
    }

    fn get(&self) -> i32 {
        self.value
    }

    fn increase(&mut self) {
        self.value += 1;
    }
}

fn is_FnMut<F: FnMut()>(_closure: &F) {}

fn is_Copy<F: Copy>(_closure: &F) {}

fn main() {
    let mut foo_copy = FooCopy::new(0);

    let mut c_with_move = move || {
        for _ in 0..5 {
            foo_copy.increase();
        }

        println!("foo_copy in closure(with move): {}", foo_copy.get());
    };

    c_with_move(); // 5
    println!("foo_copy out of closure: {}\n", foo_copy.get()); // 0

    let mut c_without_move = || {
        for _ in 0..5 {
            foo_copy.increase();
        }

        println!("foo_copy in closure(without move): {}", foo_copy.get());
    };

    is_FnMut(&c_with_move);
    is_Copy(&c_with_move);

    is_FnMut(&c_without_move);
    //is_Copy(&c_without_move); // Error

    c_without_move(); // 5
    println!(
        "foo_copy out of closure(without move): {}\n",
        foo_copy.get()
    ); // 5

    c_with_move(); // 10
    println!("foo_copy out of closure(with move): {}\n", foo_copy.get()); // 5
}

// 例子中Copy语义的变量foo_copy在使用关键字move将其Copy至闭包c_with_move内后
// 对环境中的变量不再有影响。此时闭包的匿名结构体中保存的变量为mut FooCopy
// 在闭包中使用的increase()方法通过可变借用来进行操作，所以实现了FnMut + Copy trait

// 在不使用关键字move时，闭包c_without_move对环境中的变量 foo_copy 进行了可变借用
// 此时闭包的匿名结构体内中保存的变量为&mut FooCopy，所以会对环境中的变量进行修改
// 其同样实现了FnMut trait，但不会实现 Copy trait
