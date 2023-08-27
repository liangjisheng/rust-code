use std::collections::HashSet;

fn main() {
    // insert() 用于插入一个值到集合中，如果集合中已经存在指定的值，则返回 false，否则返回 true
    let mut languages = HashSet::new();
    languages.insert("Python");
    languages.insert("Rust");
    languages.insert("Ruby");
    languages.insert("PHP");
    languages.insert("C++");
    languages.insert("Go");

    languages.insert("Rust"); // 插入失败但不会引发异常

    println!("{:#?}", languages);
    println!("{:?}", languages);
    println!("size of the set is {}", languages.len());

    // iter() 方法用于返回集合中所有元素组成的无序迭代器
    for language in languages.iter() {
        println!("{}", language);
    }

    // get() 方法用于获取集合中指定值的一个引用
    match languages.get(&"Rust") {
        Some(value) => {
            println!("found {}", value);
        }
        None => {
            println!("not found");
        }
    }

    if languages.contains(&"Rust") {
        println!("found language");
    }

    languages.remove(&"PHP");
    println!("length of the Hashset after remove() : {}", languages.len());
}
