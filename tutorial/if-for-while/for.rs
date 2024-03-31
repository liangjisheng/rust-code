fn for1() {
    let arr = ['a', 'b', 'c'];
    for element in arr.iter() {
        println!("element: {}", element);
    }
    println!("end");

    for element in arr {
        println!("the value is: {element}");
    }
    println!("end");

    for number in 1..4 {
        println!("number：{}", number);
    }

    //反转范围
    for number in (1..4).rev() {
        println!("number：{}", number);
    }
    println!("end");
}

// 注意，使用 for 时我们往往使用集合的引用形式，除非你不想在后面的代码中继续使用该集合
// （比如我们这里使用了 container 的引用）。如果不使用引用的话，所有权会被转移（move）
// 到 for 语句块中，后面就无法再使用这个集合了)：

// for item in &container {
//     ...
// }

// 对于实现了 copy 特征的数组(例如 [i32; 10] )而言， for item in arr 并不会把 arr
// 的所有权转移，而是直接对其进行了拷贝，因此循环之后仍然可以使用 arr

// 如果想在循环中，修改该元素，可以使用 mut 关键字：

// for item in &mut collection {
//   ...
// }

// 使用方法	                    等价使用方式	                                    所有权
// for item in collection	    for item in IntoIterator::into_iter(collection)	转移所有权
// for item in &collection	    for item in collection.iter()	                不可变借用
// for item in &mut collection	for item in collection.iter_mut()	            可变借用

fn for2() {
    for i in 1..11 {
        if i == 5 {
            continue;
        }
        print!("{} ", i);
    }
    println!();

    // 在循环中获取元素的索引
    let a = [4, 3, 2, 1];
    // `.iter()` 方法把 `a` 数组变成一个迭代器
    for (i, v) in a.iter().enumerate() {
        println!("第 {} 个元素是 {}", i + 1, v);
    }
}

fn for3() {
    let mut v = vec![1, 2, 3];

    // 在 for 循环中,v.len只会在循环伊始之时进行求值，之后就一直使用该值
    for i in 0..v.len() {
        v.push(i);
        println!("{:?}", v);
    }
}

fn main() {
    // for1();
    // for2();
    for3();
}
