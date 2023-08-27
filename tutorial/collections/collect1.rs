// collect() 可以接受任何可迭代的内容，并将其转换为相关的集合
// 使用collect() 的最基本模式是将一个集合转换为另一个集合。你拿一个集合，在上面调用iter，做一堆转换，最后是collect()

use std::collections::VecDeque;

fn main() {
    let a = [1, 2, 3];
    // 注意 Vec<i32>
    let doubled: Vec<i32> = a.iter().map(|&x| x * 2).collect();
    assert_eq!(vec![2, 4, 6], doubled);

    let a = [1, 2, 3];
    let doubled: VecDeque<i32> = a.iter().map(|&x| x * 2).collect();
    assert_eq!(2, doubled[0]);
    assert_eq!(4, doubled[1]);
    assert_eq!(6, doubled[2]);

    let a = [1, 2, 3];
    let doubled = a.iter().map(|x| x * 2).collect::<Vec<i32>>();
    assert_eq!(vec![2, 4, 6], doubled);

    // collect() 只关心您收集到的内容，您仍然可以使用部分类型提示 _
    let a = [1, 2, 3];
    let doubled = a.iter().map(|x| x * 2).collect::<Vec<_>>();
    assert_eq!(vec![2, 4, 6], doubled);

    let chars = ['g', 'd', 'k', 'k', 'n'];
    let hello: String = chars
        .iter()
        .map(|&x| x as u8)
        .map(|x| (x + 1) as char)
        .collect();
    assert_eq!("hello", hello);

    // 如果您有 Result<T, E> 的列表，您可以使用 collect() 查看是否有任何失败
    let results = [Ok(1), Err("nope"), Ok(3), Err("bad")];
    let result: Result<Vec<_>, &str> = results.iter().cloned().collect();
    // gives us the first error
    assert_eq!(Err("nope"), result);

    let results = [Ok(1), Ok(3)];
    let result: Result<Vec<_>, &str> = results.iter().cloned().collect();
    // gives us the list of answers
    assert_eq!(Ok(vec![1, 3]), result);
}
