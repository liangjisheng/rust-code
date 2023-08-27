// https://juejin.cn/post/7207724065058258981

trait Greeting {
    fn greeting(&self) -> &str;
}

struct Cat;

impl Greeting for Cat {
    fn greeting(&self) -> &str {
        "Meow!"
    }
}

struct Dog;

impl Greeting for Dog {
    fn greeting(&self) -> &str {
        "Woof!"
    }
}

// 在上述代码中，定义了一个 trait Greeting，两个 struct 实现了它，根据函数调用方式，主要两种使用方式：
// 基于泛型的静态派发
// 基于 trait object 的动态派发

// 比较重要的一点是 trait object 属于 Dynamically Sized Types（DST）
// 在编译期无法确定大小，只能通过指针来间接访问，常见的形式有 Box<dyn trait>, &dyn trait

// 基于泛型的静态派发
fn print_greeting_static<G: Greeting>(g: G) {
    println!("{}", g.greeting());
}

// 在 Rust 1.26 版本中，引入了一种新的 trait 使用方式，即：impl trait，
// 可以用在两个地方：函数参数与返回值。该方式主要是简化复杂 trait 的使用，
// 算是泛型的特例版，因为在使用 impl trait 的地方，也是静态派发，而且作为
// 函数返回值时，数据类型只能有一种，这一点要尤为注意!

fn print_greeting_impl(g: impl Greeting) {
    println!("{}", g.greeting());
}

// 下面代码会编译报错 mismatched types
// fn return_greeting_impl(i: i32) -> impl Greeting {
//     if i > 10 {
//         return Cat;
//     }
//     Dog
// }

// 基于 trait object 的动态派发
fn print_greeting_dynamic(g: Box<dyn Greeting>) {
    println!("{}", g.greeting());
}

// 在 Rust 中，泛型的实现采用的是单态化（monomorphization），会针对不同类型的调用者
// 在编译时生成不同版本的函数，所以泛型也被称为类型参数。好处是没有虚函数调用的开销，缺点是最终的二进制文件膨胀

fn t1() {
    print_greeting_static(Cat);
    print_greeting_static(Dog);

    print_greeting_impl(Cat);
    print_greeting_impl(Dog);

    print_greeting_dynamic(Box::new(Cat));
    print_greeting_dynamic(Box::new(Dog));
}

// Rust 提供了类型「惰性绑定」的机制，即关联类型（associated type）
// 这样就能在实现 trait 时再来确定类型，一个常见的例子是标准库中的
// Iterator, next 的返回值为 Self::Item

// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

/// 一个只输出偶数的示例
struct EvenNumbers {
    count: usize,
    limit: usize,
}

impl Iterator for EvenNumbers {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count > self.limit {
            return None;
        }
        let ret = self.count * 2;
        self.count += 1;
        Some(ret)
    }
}

fn t2() {
    let nums = EvenNumbers { count: 1, limit: 5 };
    for n in nums {
        print!("{} ", n);
    }
    println!();
}

// 关联类型的使用和泛型相似，Iterator 也可使用泛型来定义
// pub trait Iterator<T> {
//     fn next(&mut self) -> Option<T>;
// }

// 它们的区别主要在于：
// 一个特定类型（比如上文中的 Cat）可以多次实现泛型 trait。比如对于 From，
// 可以有 impl From<&str> for Cat 也可以有 impl From<String> for Cat
// 但是对于关联类型的 trait，只能实现一次。比如对于 FromStr，只能有
// impl FromStr for Cat ，类似的 trait 还有 Iterator Deref

fn main() {
    // t1();
    t2();
}

// 在 Rust 中，并不是所有的 trait 都可用作 trait object，需要满足一定的条件，
// 称之为 object safety 属性。主要有以下几点：

// 函数返回类型不能是 Self（即当前类型）。这主要因为把一个对象转为 trait object 后
// 原始类型信息就丢失了，所以这里的 Self 也就无法确定了。
// 函数中不允许有泛型参数。主要原因在于单态化时会生成大量的函数，很容易导致 trait 内的方法膨胀
