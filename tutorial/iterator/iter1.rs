// https://juejin.cn/post/7205508171523817532

// 迭代器与 for 循环最主要的差别就在于：是否通过索引来访问集合。严格来说，Rust 中的 for
// 循环是编译器提供的语法糖，最终还是对迭代器中的元素进行遍历。在 Rust 中，实现了
// IntoIterator trait 的类型都可以自动把类型集合转换为迭代器，然后通过 for 语法糖进行访问

use std::collections::HashMap;

fn i1() {
    let arr = [1, 2, 3];
    for v in arr {
        print!("{} ", v);
    }
    println!();

    // 可以使用 IntoIterator trait 的 into_iter 方法显式地将数组转换成迭代器
    let arr = [1, 2, 3];
    // let mut arr_iter = IntoIterator::into_iter(arr);
    for v in IntoIterator::into_iter(arr) {
        print!("{} ", v);
    }
    println!();
    println!("{:?}", arr);

    let arr = [1, 2, 3];
    // let mut arr_iter = arr.into_iter();
    let mut arr_iter = IntoIterator::into_iter(arr);
    assert_eq!(arr_iter.next(), Some(1));
    assert_eq!(arr_iter.next(), Some(2));
    assert_eq!(arr_iter.next(), Some(3));
    assert_eq!(arr_iter.next(), None);

    // 数组不是迭代器(没有实现Iterator)，但是数组实现了IntoIterator，Rust 通过 for 语法糖
    // 自动把实现了该特征的数组类型转换为迭代器，最终让我们可以直接对一个数组进行迭代

    let x = &[1, 2, 4];
    let mut iterator = x.iter();
    assert_eq!(iterator.next(), Some(&1));
    assert_eq!(iterator.next(), Some(&2));
    assert_eq!(iterator.next(), Some(&4));
    assert_eq!(iterator.next(), None);

    // 遍历是按照迭代器中元素的排列顺序依次进行的，同时手动迭代必须将迭代器声明为 mut 可变
    // 因为调用 next 会改变迭代器其中的状态数据，使用 for 循环时则无需标注，因为for循环自动完成
    // next 方法对迭代器的遍历是消耗性的，每次消耗它一个元素，最终迭代器中将没有任何元素，只能返回 None
}

fn i2() {
    // 标准库的一揽子实现机制为迭代器Iterator本身自动实现了 IntoIterator trait
    /*
    impl<I: Iterator> IntoIterator for I {
      type Item = I::Item;
      type IntoIter = I;

      #[inline]
      fn into_iter(self) -> I {
        self
      }
    }
    */
    // 所以 vec.into_iter() 与 vec.into_iter().into_iter().into_iter() 效果相同的

    let values = vec![1, 2, 3];
    for v in values.into_iter().into_iter().into_iter() {
        print!("{} ", v);
    }
    println!();
}

// into_iter，iter，iter_mut 产生迭代器的区别
// into_iter() 返回T，&T或 &mut T 类型的Iterator，依赖于环境；
// iter() 迭代引用(&T)：调用next方法返回的类型是Some(&T)；
// iter_mut() 迭代可变引用(&mut T)：调用next方法返回的类型是Some(&mut T)
// Rust命名规律：into_ 之类的，都是拿走所有权，_mut 之类的都是可变引用，剩下的就是不可变引用

fn i3() {
    let values = vec![1, 2, 3];

    // 所有权转移至迭代器中，values 将不能再使用
    for v in values.into_iter() {
        print!("{} ", v)
    }
    println!();

    // 下面的代码将报错，因为 values 的所有权在上面 `for` 循环中已经被转移走
    // println!("{:?}", values);

    let values = vec![1, 2, 3];
    let _values_iter = values.iter();

    // 不会报错，因为 values_iter 只是借用了 values 中的元素
    println!("{:?}", values);

    let mut values = vec![1, 2, 3];
    // 对 values 中的元素进行可变借用
    let mut values_iter_mut = values.iter_mut();

    // 取出第一个元素，并修改为0
    if let Some(v) = values_iter_mut.next() {
        *v = 0;
    }

    // 输出[0, 2, 3]
    println!("{:?}", values);
}

// Iterator 是迭代器 trait，只有实现了它才能称为迭代器，才能调用 next 方法
// IntoIterator 强调的是某一个类型如果实现了该 trait，那么该类型数据可以通过
// into_iter() 、iter() 或 iter_mut() 方法将其变成一个迭代器
// Vec<T> 方法	返回类型
// .iter()	    Iterator<Item = &T>
// .iter_mut()	Iterator<Item = &mut T>
// .into_iter()	Iterator<Item = T>

// 消费性适配器：某些方法(比如collect、 sum等)在其内部会自动调用 next方法
// 并会拿走迭代器的所有权，消耗掉迭代器上的元素
// sum 方法，它会拿走迭代器的所有权，并反复调用 next
// 来遍历迭代器并对里面的元素进行求和，并在迭代完成时返回总和

fn i4() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);

    // v1_iter 是借用了 v1，因此 v1 可以照常使用
    println!("{:?}", v1);

    // 以下代码会报错，因为 `sum` 拿到了迭代器 `v1_iter` 的所有权
    // println!("{:?}", v1_iter);

    let v1: Vec<i32> = vec![1, 2, 3];
    // 警告，因为map方法返回一个迭代器，map 迭代器是惰性的，不产生任何效果
    let _ = v1.iter().map(|x| x + 1);
    // 将v1中的数依次加1然后生成一个新的Vec，其中collect()方法就是一个消费者适配器
    // Vec<_> 表明将迭代器中的元素收集成 Vec 类型，具体元素类型通过类型推导获得
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    // zip是一个迭代器适配器，其作用就是将两个迭代器的内容压缩到一起，形成
    // Iterator<Item=(ValueFromA, ValueFromB)>这样的新的迭代器，然后通过
    // collect方法将迭代器中的（K, V）形式的值收集成HashMap<K, V>
    let names = ["tim", "tony"];
    let ages = [11, 33];
    let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();
}

// 实现自己的迭代器非常简单，但是 Iterator trait 中，只需要自己实现 next 一个方法
// 因为其他方法都有默认实现，并且这些默认实现的方法其实都是基于 next 方法实现的

struct MyType {
    items: Vec<String>,
}

impl MyType {
    fn iter(&self) -> impl Iterator<Item = &String> {
        MyTypeIterator {
            index: 0,
            items: &self.items,
        }
    }
}

struct MyTypeIterator<'a> {
    index: usize,
    items: &'a Vec<String>,
}

impl<'a> Iterator for MyTypeIterator<'a> {
    type Item = &'a String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.items.len() {
            None
        } else {
            let item = &self.items[self.index];
            self.index += 1;
            Some(item)
        }
    }
}

fn i5() {
    let my_type: MyType = MyType {
        items: vec!["alice".to_string(), "bob".to_string()],
    };

    for v in my_type.iter() {
        println!("{}", v);
    }

    // 比如enumerate 是 Iterator trait 上的方法，该方法产生一个新的迭代器
    // 其每个元素都是元素（索引，值）。enumerate 是迭代器适配器，可以使用
    // 消费者迭代器或 for 循环对新产生的迭代器进行处理
    let v = vec![1u64, 2, 3, 4, 5, 6];
    for (i, v) in v.iter().enumerate() {
        println!("第 {} 个值是 {}", i, v)
    }
}

fn main() {
    // i1();
    // i2();
    // i3();
    // i4();
    i5();
}
