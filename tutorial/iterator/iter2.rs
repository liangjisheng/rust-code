// pub trait Iterator {
//     type Item;
//
//     fn next(&mut self) -> Option<Self::Item>;
//
//  多种内置实现方法, skip, map, reduce, collect
//  和Java中的Stream内置方法非常类似.
// }

#[derive(Debug, Clone)]
struct Animal {
    name: String,
    age: u32,
    kind: String,
    i: i32,
}

// 可以在Animal上实现Iterator trait，使其可以通过for循环进行迭代
impl Iterator for Animal {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let next_attribute = match self.i {
            0 => Some(self.name.clone()),
            1 => Some(self.age.to_string()),
            2 => Some(self.kind.clone()),
            _ => None,
        };
        self.i += 1;
        next_attribute
    }
}

fn i1() {
    let mut animal = Animal {
        name: "Tom".to_string(),
        age: 15,
        kind: "cat".to_string(),
        i: 0,
    };
    println!("Name: {}", animal.next().unwrap());
    println!("Age: {}", animal.next().unwrap());
    println!("Kind: {}", animal.next().unwrap());
}

fn print_all_attributes(animals: Vec<Animal>) {
    for mut animal in animals {
        println!("Name: {}", animal.next().unwrap());
        println!("Age: {}", animal.next().unwrap());
        println!("Kind: {}", animal.next().unwrap());
    }
}

fn i2() {
    let animals = vec![Animal {
        name: "Tom".to_string(),
        age: 15,
        kind: "cat".to_string(),
        i: 0,
    }];
    print_all_attributes(animals);
}

fn map_demo() {
    let animals = vec![
        Animal {
            name: "Tom".to_string(),
            age: 15,
            kind: "cat".to_string(),
            i: 0,
        },
        Animal {
            name: "Jerry".to_string(),
            age: 7,
            kind: "mouse".to_string(),
            i: 0,
        },
    ];
    let list: Vec<String> = animals.into_iter().map(|ani| ani.name.clone()).collect();
    println!("{:?}", list)
}

fn filter_demo() {
    let animals = vec![Animal {
        name: "Tom".to_string(),
        age: 15,
        kind: "cat".to_string(),
        i: 0,
    }];
    let filtered_animals: Vec<Animal> = animals
        .into_iter()
        .filter(|animal| animal.age >= 3)
        .collect();
    println!("{:?}", filtered_animals)
}
//  输出结果：
//  [Animal { name: "Tom", age: 15, kind: "cat", i: 0 }]

// enumerate 方法会将一个迭代器中的元素和它们的索引配对，并返回一个新的迭代器
fn enumerate_demo() {
    let animals = vec![
        Animal {
            name: "Tom".to_string(),
            age: 15,
            kind: "cat".to_string(),
            i: 0,
        },
        Animal {
            name: "Jerry".to_string(),
            age: 7,
            kind: "mouse".to_string(),
            i: 0,
        },
    ];
    for (i, animal) in animals.iter().enumerate() {
        println!("{}: {:?}", i, animal);
    }
}
// 输出：
// 0: Animal { name: "Tom", age: 15, kind: "cat", i: 0 }
// 1: Animal { name: "Jerry", age: 7, kind: "mouse", i: 0 }

// flat_map方法是Iterator trait 中比较少见的方法之一，它可以用于将嵌套的迭代器展开为单个迭代器
fn flat_map_demo() {
    let cat = Animal {
        name: "Tom".to_string(),
        age: 15,
        kind: "cat".to_string(),
        i: 0,
    };
    let mouse = Animal {
        name: "Jerry".to_string(),
        age: 7,
        kind: "mouse".to_string(),
        i: 0,
    };
    let animals = vec![vec![cat], vec![mouse]];

    let list: Vec<Animal> = animals.iter().flat_map(|x| x.iter().cloned()).collect();
    println!("{:?}", list)
}

// 如果我们需要同时遍历两个向量，我们可以使用zip方法进行配对
fn zip_demo() {
    let names = vec!["Tom", "Jerry", "Bob"];
    let ages = vec![3, 4, 5];

    for (name, age) in names.iter().zip(ages.iter()) {
        println!("{} is {} years old.", name, age);
    }
}

// fold 方法在Rust中也十分重要，它可以接受一个初始值和一个闭包，遍历迭代器中的每一个元素，
// 并将它们合并成单个值
fn fold_demo() {
    let cat = Animal {
        name: "Tom".to_string(),
        age: 15,
        kind: "cat".to_string(),
        i: 0,
    };
    let mouse = Animal {
        name: "Jerry".to_string(),
        age: 7,
        kind: "mouse".to_string(),
        i: 0,
    };
    let animals = vec![cat, mouse];

    let sum = animals.iter().fold(0, |t, ani| t + ani.age);
    println!("{}", sum)
}
// 输出 22

fn main() {
    // i1();
    // i2();
    // map_demo();
    // filter_demo();
    // enumerate_demo();
    // flat_map_demo();
    // zip_demo();
    fold_demo();
}
