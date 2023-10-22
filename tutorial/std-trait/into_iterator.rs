// trait IntoIterator
// where
//     <Self::IntoIter as Iterator>::Item = Self::Item,
// {
//     type Item;
//     type IntoIter: Iterator;
//     fn into_iter(self) -> Self::IntoIter;
// }

// 正如其名，IntoIterator类型可以转化为迭代器。当一个类型在一个for-in
// 循环里被使用的时候，该类型的into_iter方法会被调用

// vec = Vec<T>
// for v in vec {} // v = T

// above line desugared
// for v in vec.into_iter() {}

// 不仅Vec实现了IntoIterator，如果我们想在不可变引用或可变引用上迭代，&Vec和&mut Vec同样也是如此

// vec = Vec<T>
// for v in &vec {} // v = &T

// above example desugared
// for v in (&vec).into_iter() {}

// vec = Vec<T>
// for v in &mut vec {} // v = &mut T

// above example desugared
// for v in (&mut vec).into_iter() {}

// trait FromIterator<A> {
//     fn from_iter<T>(iter: T) -> Self
//     where
//         T: IntoIterator<Item = A>;
// }

// 正如其名，FromIterator类型可以从一个迭代器创建而来。FromIterator最常用于Iterator上的collect方法调用

// fn collect<B>(self) -> B
// where
//     B: FromIterator<Self::Item>;

// 搜集（collect）一个Iterator<Item = char> 到 String

fn filter_letters(string: &str) -> String {
    string.chars().filter(|c| c.is_alphabetic()).collect()
}

// 标准库中所有的集合都实现了IntoIterator和FromIterator，从而使它们之间的转换更为简单

use std::collections::{BTreeSet, HashMap, HashSet, LinkedList};

// String -> HashSet<char>
fn unique_chars(string: &str) -> HashSet<char> {
    string.chars().collect()
}

// Vec<T> -> BTreeSet<T>
fn ordered_unique_items<T: Ord>(vec: Vec<T>) -> BTreeSet<T> {
    vec.into_iter().collect()
}

// HashMap<K, V> -> LinkedList<(K, V)>
fn entry_list<K, V>(map: HashMap<K, V>) -> LinkedList<(K, V)> {
    map.into_iter().collect()
}

// and countless more possible examples

fn main() {}
