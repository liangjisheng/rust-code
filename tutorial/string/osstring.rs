// String、&str 变量有个限制，就是只能保存 UTF-8 格式的字符串
// 根据设计文档，OsString、OsStr 作用分别类似 String 和 &str
// 但是不再做合法性检查。因此，其他编程语言传过来的字符串，他们照单全收

use std::ffi::OsStr;
use std::ffi::OsString;
use std::str::FromStr;

use std::ffi::CStr;
use std::ffi::CString;

fn osstr() {
    let a_str: &str = "Hello Str!";
    let an_os_str: &OsStr = OsStr::new(a_str);
    println!("{:?}", an_os_str);

    let a_string = String::from("Hello, String!");
    let an_os_string = OsString::from(a_string);
    println!("{:?}", an_os_string);

    let s1 = OsString::from_str("hello").unwrap();
    println!("{:?}", s1);
    let s2 = s1.to_str().unwrap();
    println!("{:?}", s2);
}

fn cstr() {
    let mut v1 = "hello".to_string().into_bytes();
    // 提供空值作为字符串结束符
    v1.push(0);

    let v2 = "hello".to_string().into_bytes();
    unsafe {
        let c_str = CStr::from_bytes_with_nul(v1.as_slice());
        if c_str.is_err() {
            println!("err {}", c_str.unwrap_err())
        } else {
            println!("{:?}", c_str.unwrap());
        }

        let c_string = CString::from_vec_unchecked(v2);
        println!("{:?}", c_string);
    }
}

fn main() {
    // osstr();
    cstr();
}
