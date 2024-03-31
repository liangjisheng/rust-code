// 最后一个实例使用可变的 [HashMap] 来计算众数，[fold] 和 [entry] API 用来
// 从集合中收集每个不同整数的计数。[HashMap] 中最常见的值可以用 [max_by_key] 取得

use std::collections::HashMap;

fn main() {
    let data = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];

    let frequencies = data.iter().fold(HashMap::new(), |mut freqs, value| {
        *freqs.entry(value).or_insert(0) += 1;
        freqs
    });

    let mode = frequencies
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value, _)| *value);

    println!("Mode of the data is {:?}", mode);
}
