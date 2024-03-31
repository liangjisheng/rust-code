// 第一个实例是通过对数据引用生成一个迭代器，然后计算平均数（所有测量值的总和除以测量值的计数）
// 并使用 [sum] 和 [len] 函数分别确定值的总和及值的计数

fn main() {
    let data = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];

    let sum = data.iter().sum::<i32>() as f32;
    let count = data.len();

    let mean = match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    };

    println!("Mean of the data is {:?}", mean);
}
