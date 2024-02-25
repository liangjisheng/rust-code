struct Interface<'a> {
    manager: &'a mut Manager<'a>,
}

impl<'a> Interface<'a> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct Manager<'a> {
    text: &'a str,
}

struct List<'a> {
    manager: Manager<'a>,
}

impl<'a> List<'a> {
    pub fn get_interface(&'a mut self) -> Interface {
        Interface {
            manager: &mut self.manager,
        }
    }
}

fn main() {
    let mut list = List {
        manager: Manager { text: "hello" },
    };

    list.get_interface().noop();

    println!("Interface should be dropped here and the borrow released");

    // 下面的调用会失败，因为同时有不可变/可变借用
    // 但是Interface在之前调用完成后就应该被释放了
    use_list(&list);
}

fn use_list(list: &List) {
    println!("{}", list.manager.text);
}

// 这段代码看上去并不复杂，实际上难度挺高的，首先在直觉上，list.get_interface() 借用的可变引用，
// 按理来说应该在这行代码结束后，就归还了，但是为什么还能持续到 use_list(&list) 后面呢？

// 这是因为我们在 get_interface 方法中声明的 lifetime 有问题，该方法的参数的生命周期是 'a，而
// List 的生命周期也是 'a，说明该方法至少活得跟 List 一样久，再回到 main 函数中，list 可以活到
// main 函数的结束，因此 list.get_interface() 借用的可变引用也会活到 main 函数的结束，在此期间，
// 自然无法再进行借用了。

// 要解决这个问题，我们需要为 get_interface 方法的参数给予一个不同于 List<'a> 的生命周期 'b，最终代码如下：
// complex_demo1.rs
