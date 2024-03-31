use std::ffi::CString;
use std::os::raw::c_char;

extern "C" {
    fn hello();
    fn greet(name: *const c_char);
    fn multiply(x: i32, y: i32) -> i32;
    fn print_app_info();
}

fn prompt(s: &str) -> Result<String, std::io::Error> {
    use std::io::Write;
    print!("{}", s);
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn demo_c() {
    unsafe {
        hello();
    }

    let name = prompt("What's your name? ").unwrap_or("alice".to_string());
    // let c_name = CString::new(name).unwrap();
    let c_name = CString::new(name).unwrap_or_default();
    unsafe { greet(c_name.as_ptr()) }
}

// 链接绑定的 C++ 语言库非常类似于链接绑定的 C 语言库。编译并静态链接绑定的 C++ 库时，
// 与链接绑定的 C 语言库相比有两个核心区别：一是通过构造器方法 cpp(true) 指定 C++
// 编译器；二是通过在 C++ 源文件顶部添加 extern "C" 代码段，以防止 C++ 编译器的名称篡改。

fn demo_cpp() {
    unsafe {
        println!("{}", multiply(5, 7));
    }
}

// 使用 [cc::Build::define] 自定义构建绑定的 C 语言代码
fn demo_c1() {
    unsafe {
        print_app_info();
    }
}

fn main() {
    // demo_c();
    // demo_cpp();
    demo_c1();
}
