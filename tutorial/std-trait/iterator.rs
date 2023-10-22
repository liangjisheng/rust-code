// Iterator<Item = T> 类型可以被迭代并产生T类型。没有IteratorMut trait
// 每个Iterator实现可以指定它返回的是不可变引用、可变引用还是拥有通过Item关联类型的值
// .iter() 返回 Iterator<Item = &T>
// .iter_mut() 返回 Iterator<Item = &mut T>
// .into_iter() 返回 Iterator<Item = T>

// 大多数类型没有它们自己的迭代器，这对于初级 Rustaceans 来说，并不明显，
// 但中级 Rustaceans 认为这是理所当然的。如果一个类型是可迭代的，
// 我们几乎总是实现自定义的迭代器类型来迭代它，而不是让它自己迭代

mod m1 {
    struct MyType {
        items: Vec<String>,
    }

    impl MyType {
        fn iter(&self) -> impl Iterator<Item = &String> {
            MyTypeIterator {
                index: 0,
                items: &self.items,
            }
        }
    }

    struct MyTypeIterator<'a> {
        index: usize,
        items: &'a Vec<String>,
    }

    impl<'a> Iterator for MyTypeIterator<'a> {
        type Item = &'a String;
        fn next(&mut self) -> Option<Self::Item> {
            if self.index >= self.items.len() {
                None
            } else {
                let item = &self.items[self.index];
                self.index += 1;
                Some(item)
            }
        }
    }

    pub fn t1() {
        let mt = MyType {
            items: vec!["alice".to_string(), "bob".to_string()],
        };
        for v in mt.iter() {
            println!("{}", v);
        }
    }
}

// 上面的例子展示了如何从头开始实现一个迭代器，但在这种情况下，常用的解决方案是直接延用Vec的iter方法
mod m2 {
    struct MyType {
        items: Vec<String>,
    }

    impl MyType {
        fn iter(&self) -> impl Iterator<Item = &String> {
            self.items.iter()
        }
    }

    pub fn t2() {
        let mt = MyType {
            items: vec!["mike".to_string(), "john".to_string()],
        };
        for v in mt.iter() {
            println!("{}", v);
        }
    }
}

fn main() {
    // m1::t1();
    m2::t2();
}
