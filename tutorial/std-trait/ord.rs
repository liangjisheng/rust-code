// 一旦一个类型实现了Ord，我们就可以把它存储在BTreeMap和BTreeSet，还可以在 slice
// 上使用 sort()方法对其进行排序，这同样适用于其他可以解引用为 slice 的类型，比如数组、Vec和VecDeque

use std::collections::BTreeSet;

// now our type can be stored
// in BTreeSets and BTreeMaps!
#[derive(Ord, PartialOrd, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

fn example_btreeset() {
    let mut points = BTreeSet::new();
    points.insert(Point { x: 0, y: 0 });
}

// we can also .sort() Ord types in collections!
fn example_sort<T: Ord>(mut sortable: Vec<T>) -> Vec<T> {
    sortable.sort();
    sortable
}

fn main() {
    example_btreeset();

    let mut v = vec![5, 4, 3, 2, 1];
    v = example_sort(v);
    println!("{:?}", v);
}
