// Result 内部本质又是一个枚举，内部分别是 Ok 和 Err，是 Ok 时则代表正常返回值，Err 则代表异常

// 使用 ? 后，你不需要挨个判断并返回，任何一个 ? 返回 Err 了函数都会直接返回 Err
// 因为 ? 的行为是遇到 Err 直接返回 Err，众所周知，返回的行为必须在函数上，所以使用 ? 时，
// 相关的代码必须在一个函数/方法里，而函数也应该返回 Result 类型，
// 如果在 main() 中直接使用则同样需要把 main() 改为 main() -> Result
// ? 不仅可以用于 Result 也可以用于 Option

fn r1() {
    let sr: Result<u32, &str> = Ok(123); //or Err("You are wrong");

    match sr {
        Ok(v) => println!("sr value: {}", v),
        Err(e) => println!("sr is error {}", e),
    }

    // 直接拿值, 正常则拿值，异常则 panic
    let val: u32 = sr.unwrap();
    println!("{}", val);
}

fn r2() {
    let val = 12;
    let x: Result<&i32, i32> = Ok(&val);
    assert_eq!(x, Ok(&12));
    let cloned = x.cloned();
    assert_eq!(cloned, Ok(12));
}

// 返回一个结果，类型是 u32 或字串（&'static str）
fn is_it_fifty(num: u32) -> Result<u32, &'static str> {
    let error = "It didn’t work";
    if num == 50 {
        Ok(num)
    } else {
        Err(error)
    }
}

fn r3() {
    let my_num = 50;
    match is_it_fifty(my_num) {
        Ok(_v) => println!("Good! my_num is 50"),
        Err(_e) => println!("Error. my_num is {:?}", my_num),
    }
}

#[derive(Debug)]
enum Version {
    Version1,
    Version2,
}

fn parse_version(header: &[u8]) -> Result<Version, &'static str> {
    match header.get(0) {
        None => Err("invalid header length"),
        Some(&1) => Ok(Version::Version1),
        Some(&2) => Ok(Version::Version2),
        Some(_) => Err("invalid version"),
    }
}

fn r4() {
    let version = parse_version(&[1, 2, 3, 4]);
    match version {
        Ok(v) => println!("working with version: {:?}", v),
        Err(e) => println!("error parsing header: {:?}", e),
    }
}

fn r5() {
    let good_result: Result<i32, i32> = Ok(10);
    let bad_result: Result<i32, i32> = Err(10);

    // The `is_ok` and `is_err` methods do what they say.
    assert!(good_result.is_ok() && !good_result.is_err());
    assert!(bad_result.is_err() && !bad_result.is_ok());

    // `map` consumes the `Result` and produces another.
    let good_result: Result<i32, i32> = good_result.map(|i| i + 1);
    let bad_result: Result<i32, i32> = bad_result.map(|i| i - 1);
    assert!(bad_result.is_err());

    // Use `and_then` to continue the computation.
    let good_result: Result<bool, i32> = good_result.and_then(|i| Ok(i == 11));

    // Use `or_else` to handle the error.
    let bad_result: Result<i32, i32> = bad_result.or_else(|i| Ok(i + 20));
    assert!(bad_result.is_ok());

    // Consume the result and return the contents with `unwrap`.
    let final_awesome_result = good_result.unwrap();
}

fn main() {
    // r1();
    // r2();
    // r3();
    // r4();
    r5();
}
