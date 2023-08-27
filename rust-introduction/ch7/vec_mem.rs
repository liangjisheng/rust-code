// Vec所存储的数据部分在堆内存中，同时在栈空间中存放了该vec的胖指针。胖指针包括三部分元数据：
// 指向堆的指针(一个机器字长)
// 当前vec元素数量(即长度，usize，一个机器字长)
// vec的容量(即当前vec最多可存放多少元素，usize，一个机器字长)

fn v1() {
    let mut v1 = vec![11, 22, 33];
    // len: 3, cap: 3
    println!("len: {}, cap: {}", v1.len(), v1.capacity());

    // push()向vec中插入一个元素，将导致扩容
    // 扩容将导致重新分配vec的内存
    v1.push(44);
    // len: 4, cap: 6
    println!("len: {}, cap: {}", v1.len(), v1.capacity());
}

// 频繁扩容或者当元素数量较多且需要扩容时，大量的内存拷贝会降低程序的性能
// 因此，如果可以的话，可以采取如下方式：
// 在创建vec的时候使用Vec::with_capacity()指定一个足够大的容量值，以此来尽量减少可能的内存拷贝。
// 通过reserve()方法来调整已存在的vec容量，使之至少有指定的空闲容量数，以此来尽量减少可能的内存拷贝

fn v2() {
    // 创建一个容量为3的空vec
    let mut v1 = Vec::with_capacity(3);
    v1.push(11);
    v1.push(22);
    v1.push(33);
    // len: 3, cap: 3
    println!("len: {}, cap: {}", v1.len(), v1.capacity());

    // 调整v1，使其至少要有10个空闲位置
    v1.reserve(10);
    // len: 3, cap: 13
    println!("len: {}, cap: {}", v1.len(), v1.capacity());

    // 当空闲容量足够时，reserve()什么也不做
    v1.reserve(5);
    println!("len: {}, cap: {}", v1.len(), v1.capacity());
}

fn main() {
    // v1();
    v2();
}
