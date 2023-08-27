fn main() {
    // 通过iter()调用得到的迭代器，其中迭代的是nums数组中元素的的引用（借用）
    // 可以看到，闭包中的参数x前面带有引用&符号
    let nums = vec![1, 2, 3, 4];

    let borrowed_values = nums.iter().for_each(|&x| {
        println!("{}", x); //x为i32类型
    });

    // 通过into_iter()调用得到的迭代器，其中迭代的是nums数组中元素本身（占据所有权）。
    // 调用nums.into_iter()之后，nums变量的所有权会被转移走，无法再次使用nums
    let nums = vec![1, 2, 3, 4];
    // 迭代器中迭代的是nums数组中元素本身（占据所有权）
    let owned_values = nums.into_iter().for_each(|x| {
        println!("{}", x);
    });
    // 下面这行代码编译不过：borrow of moved value: `nums`，即nums所有权已被转移走
    // println!("{:?}", nums);

    let v = vec![1, 2, 3];
    let mut iter = v.into_iter();

    assert_eq!(Some(1), iter.next());
    assert_eq!(Some(2), iter.next());
    assert_eq!(Some(3), iter.next());
    assert_eq!(None, iter.next());
}
