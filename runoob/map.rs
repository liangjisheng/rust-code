use std::collections::HashMap;

// 如果已经存在相同的键，会直接覆盖对应的值

fn main() {
    let mut map = HashMap::new();

    map.insert("color", "red");
    map.insert("size", "10 m^2");

    // 确认当前不存在某个键时才执行的插入动作
    map.entry("color").or_insert("red");

    println!("{}", map.get("color").unwrap());

    // 迭代
    for p in map.iter() {
        println!("{:?}", p);
    }

    // 已经确定有某个键的情况下如果想直接修改对应的值
    if let Some(x) = map.get_mut(&"color") {
        *x = "b";
    }
    for p in map.iter() {
        println!("{:?}", p);
    }
}
