// VecDeque是双端队列，同时具有栈和队列特征的数据结构

use std::collections::VecDeque;

fn vd1() {
    let mut v: VecDeque<u32> = VecDeque::new();
    v.push_front(1);
    v.push_back(2);
    v[1] = 0;

    // pop_front 删除并返回队列的头部元素，并返回该元素，如果列表为空，则返回None
    // pop_back 删除并返回队列的尾部元素，并返回该元素，如果列表为空，则返回None
    assert_eq!(v.pop_front(), Some(1));
    assert_eq!(v.pop_back(), Some(0));
    assert_eq!(v.pop_back(), None);

    // remove 删除指定索引元素，然后后面的元素向前补齐，如果索引越界会导致程序错误
    let mut v: VecDeque<u32> = VecDeque::new();
    v.push_back(0);
    v.push_back(1);
    v.remove(0); // Some(1)
    v.remove(1); // None

    // get 通过索引访问元素，返回值是Option<&T>，如果元素存在则返回Some(&T)，否则返回None
    let mut v: VecDeque<u32> = VecDeque::new();
    v.push_back(0);
    v.push_back(1);
    v.get(1); // Some(1)
    v.get(3); // None
}

fn main() {
    vd1();
}
