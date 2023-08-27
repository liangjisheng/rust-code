// 创建向量有几种方式
// Vec::new()创建空的vec
// Vec::with_capacity()创建空的vec，并将其容量设置为指定的数量
// vec![]宏创建并初始化vec(中括号可以换为小括号或大括号)
// vec![v;n]创建并初始化vec，共n个元素，每个元素都初始化为v

fn main() {
    let mut v1 = Vec::new();
    // 追加元素时，将根据所追加的元素推导v1的数据类型Vec<i32>
    v1.push(1); // push()向vec尾部追加元素
    v1.push(2);
    v1.push(3);
    v1.push(4);
    assert_eq!(v1, [1, 2, 3, 4]); // vec可以直接和数组进行比较

    // v2的类型推导为：Vec<i32>
    let v2 = vec![1, 2, 3, 4];
    assert_eq!(v2, [1, 2, 3, 4]);

    let v3 = vec![3; 4]; // 等价于vec![3,3,3,3]
    assert_eq!(v3, [3, 3, 3, 3]);

    // 创建容量为10的空vec
    let mut v4 = Vec::with_capacity(10);
    v4.push(33);

    let v = vec![11, 22, 33, 44];
    let n: usize = 3;
    println!("{},{}", v[0], v[n]);

    // 如果不想要在越界访问vec时panic中断程序，可使用
    // get()来获取指定索引处的元素引用或范围内元素的引用，如果索引越界，返回None。
    // get_mut()来获取元素的可变引用或范围内元素的可变引用，如果索引越界，返回None

    let v = [11, 22, 33, 44];
    // 取得index=3处元素，成功，于是unwrap()提取得到44
    let n = v.get(3).unwrap();
    println!("{}", n);

    // 取得index=4处元素，失败，于是panic报错
    // let nn = v.get(4).unwrap();

    let v = vec![11, 22, 33, 44];
    //迭代后, v 发生了移动, 不能再使用
    for i in v {
        println!("{}", i);
    }
    // error
    // println!("{:?}", v);
}
