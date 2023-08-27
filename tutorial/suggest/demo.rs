use std::fs;
use std::path::Path;

fn c1() {
    let my_vec: Vec<String> = Vec::new();
    // if my_vec.len() == 0 {
    //     println!("my_vec is empty");
    // }

    // 推荐使用 is_empty() 判断是否为空
    if my_vec.is_empty() {
        println!("my_vec is empty");
    }
}

// 其他语言比如Javascript，在某些情况下会利用哨兵值返回空值，比如如下这个例子中，
// 在找不到条件值的情况下，会返回-1以告诉你没有找到，这个值就是哨兵值，
// 那么如果我们需要利用返回结果时，则需要对于这个值做额外检查

// const arr = [1,2,3]
// arr.find(item => item === 4)

// 而Rust则具有一个更好的类型系统来应对这个问题，你不需要再依赖哨兵值了，可以直接使用Option来代替
// 不推荐
fn maybe_get_name(x: bool) -> &'static str {
    if x {
        "Bob"
    } else {
        ""
    }
}

// 使用Option的写法(推荐): 利用Option，可以让我们更好的去判断是否为空值，而不需要知道或定义一个哨兵值了
fn maybe_get_name1(x: bool) -> Option<&'static str> {
    if x {
        Some("Bob")
    } else {
        None
    }
}

fn c2() {
    let name = maybe_get_name(true);
    println!("first name is {}", name);
    let name = maybe_get_name(false);
    println!("second name is {}", name);

    let name = maybe_get_name1(true);
    if let Some(v) = name {
        println!("first name is {}", v);
    }
    let name = maybe_get_name1(false);
    if let Some(v) = name {
        println!("second name is {}", v);
    }
}

fn print_file_content(path: &str) {
    let content = fs::read_to_string(path).unwrap();
    println!("{}", content);
}

// 如果我们只是这样定义函数参数，那么我们必须要传入一个字符串slice作为文件名称，
// 而不能够传入其他的同样可以指定到文件的参数，例如Path::new("README.md")。所以更好的做法是

fn print_file_content1(path: impl AsRef<Path>) {
    let content = fs::read_to_string(path).unwrap();
    println!("{}", content);
}

fn c3() {
    print_file_content("test.txt");

    print_file_content1("test.txt");
    let path = Path::new("test.txt");
    print_file_content1(path);
}

fn main() {
    // c1();
    // c2();
    c3();
}
