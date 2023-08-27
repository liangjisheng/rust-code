use std::collections::HashMap;

fn h1() {
    let mut state_codes = HashMap::new();

    // insert() 方法用于插入或更新一个键值对到哈希表中。
    // 如果键已经存在，则更新为新的简直对，并则返回旧的值。
    // 如果键不存在则执行插入操作并返回 None
    state_codes.insert("name", "alice");
    state_codes.insert("site", "https://alice.com");
    println!("{:?}", state_codes);
    // pretty 格式输出
    println!("{:#?}", state_codes);
    println!("size of map is {}", state_codes.len());

    // get() 方法用于根据键从哈希表中获取相应的值。
    // 如果值不存在，也就是哈希表不包含参数的键则返回 None。
    // 如果值存在，则返回值的一个引用
    match state_codes.get(&"name") {
        Some(value) => {
            println!("Value for key name is {}", value);
        }
        None => {
            println!("nothing found");
        }
    }

    // iter() 方法会返回哈希表中 键值对的引用 组成的无序迭代器
    // 迭代器元素的类型为 (&'a K, &'a V)
    for (key, val) in state_codes.iter() {
        println!("key: {} val: {}", key, val);
    }

    if state_codes.contains_key(&"name") {
        println!("found key");
    }

    state_codes.remove(&"site");
    println!("length of the hashmap after remove() {}", state_codes.len());
    println!("{:?}", state_codes);
}

fn h2() {
    // insert插入/更新键值对，如果键不存在，则执行插入操作，并返回None；如果键存在，则执行更新操作，并返回旧值
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("lisi", 86); // None
    map.insert("lisi", 97); // Some(86)

    // 使用entry和or_insert方法检查键是否有对应值，没有对应值就插入键值对，有对应值则不执行操作。
    // entry方法以键为参数，返回值是一个枚举类型Entry。
    // Entry类型的or_insert方法以值为参数，在键有对应值时不执行任何操作；在键没有对应值时将将只对插入HashMap
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.entry("alice").or_insert(97); // 插入成功
    map.entry("alice").or_insert(98); // 没有插入和更新
    map.insert("lisi", 98);
    println!("{}", map["alice"]); // 97
    println!("{:?}", map);
    map.get("alice"); // Some(97)
    map.get("bob"); // None

    for (_, val) in map.iter_mut() {
        *val += 2;
    }
    println!("{:?}", map);

    // remove删除并返回指定的键的值，如果键不存在则返回None
    map.remove("lisi"); // Some(98)
    map.remove("bob"); // None

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn h3() {
    // 对于实现了 Copy trait 的类型(如i32)，值会被复制到HashMap中。
    // 对于拥有所有权的值(如String)，值将被移动，所有权会转移给HashMap。
    // 如果将值的引用插入到HashMap，值本身不会移动。但是开发者需要保证在HashMap有效的期间内，被引用的值必须保持有效
    let field_name = String::from("I am key");
    let field_value = String::from("I am value");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 这里 field_name 和 field_value 不再有效
    // println!("{}, {}", field_name, field_value);

    //  ********** 例子2 **********

    let field_name = String::from("I am key");
    let field_value = String::from("I am value");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);

    // field_name和field_value仍然可以使用
    println!("{}, {}", field_name, field_value);
}

fn main() {
    // h1();
    // h2();
    h3();
}
