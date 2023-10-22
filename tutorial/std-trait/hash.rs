// 为一个类型实现Eq和Hash的最大好处是，它让我们能够把类型作为 key 存储在HashMap和HashSet中

use std::collections::HashSet;

// now our type can be stored
// in HashSets and HashMaps!
#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn example_hashset() {
    let mut points = HashSet::new();
    points.insert(Point { x: 0, y: 0 });
}

fn main() {
    example_hashset();
}
