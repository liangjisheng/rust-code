// 在具有 OOP 风格的数据继承的编程语言中，一个方法中的self的值总是等于调用这个方法的类型，
// 但是在 Rust 中，self的值永远等于实现这个方法的类型

struct Human {
    profession: &'static str,
    health_points: u32,
}

impl Human {
    // self will always be a Human here, even if we call it on a Soldier
    fn state_profession(&self) {
        println!("I'm a {}!", self.profession);
    }
}

enum Weapon {
    Spear,
    Axe,
    Sword,
}

struct Soldier {
    profession: &'static str,
    human: Human,
    weapon: Weapon,
}

fn example() {
    let soldier = &Soldier {
        profession: "spearman",
        human: Human {
            profession: "servant",
            health_points: 0,
        },
        weapon: Weapon::Spear,
    };

    assert_eq!("servant", soldier.human.profession);
    assert_eq!("spearman", soldier.profession);
    soldier.human.state_profession(); // prints "I'm a servant!"

    // soldier.state_profession(); // still prints "I'm a servant!"
}

use std::ops::Deref;

#[derive(Debug)]
struct SortedVec<T: Ord>(Vec<T>);

impl<T: Ord> SortedVec<T> {
    fn new(mut vec: Vec<T>) -> Self {
        vec.sort();
        SortedVec(vec)
    }
    fn push(&mut self, t: T) {
        self.0.push(t);
        self.0.sort();
    }
}

impl<T: Ord> Deref for SortedVec<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Vec<T> {
        &self.0
    }
}

fn example1() {
    let mut sorted: SortedVec<i32> = SortedVec::new(vec![2, 8, 6, 3]);
    sorted.push(1);
    println!("sorted {:?}", sorted);

    // 我们未曾给SortedVec实现Clone，所以当我们调用.clone()方法时，编译器使用
    // 解引用强制转换把它解析为Vec上的方法调用，所以它会返回一个Vec而不是一个SortedVec
    let mut sorted_clone: Vec<i32> = sorted.clone();
    // sorted_clone no longer sorted
    sorted_clone.push(4);
    println!("sorted_clone {:?}", sorted_clone);
}

fn main() {
    // example();
    example1();
}
