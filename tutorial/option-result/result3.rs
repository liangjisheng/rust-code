#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NonPositiveLogarithm,
    NegativeSquareRoot,
}

type MathResult = Result<f64, MathError>;

// 除法
fn div(x: f64, y: f64) -> MathResult {
    if y == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(x / y)
    }
}

// 平方根
fn sqrt(x: f64) -> MathResult {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

// 取对数
fn ln(x: f64) -> MathResult {
    if x <= 0.0 {
        Err(MathError::NonPositiveLogarithm)
    } else {
        Ok(x.ln())
    }
}

// sqrt(ln(x / y)) 测试
fn test_ops(x: f64, y: f64) -> MathResult {
    match div(x, y) {
        Err(e) => Err(e),
        Ok(ratio) => match ln(ratio) {
            Err(e) => Err(e),
            Ok(ln) => match sqrt(ln) {
                Err(e) => Err(e),
                Ok(sqrt) => Ok(sqrt),
            },
        },
    }
}

fn test_ops_result(x: f64, y: f64) {
    match test_ops(x, y) {
        Err(e) => {
            println!("test({}, {}) fail: {:?}", x, y, e);
        }
        Ok(val) => {
            println!("sqrt(ln({} / {})) = {}", x, y, val);
        }
    }
}

fn test_ops_simple(x: f64, y: f64) -> MathResult {
    let ratio = div(x, y)?;
    let ln = ln(ratio)?;
    let sqrt = sqrt(ln)?;

    Ok(sqrt)
}

// 使用 ? 解包，简直清爽的不要不要的，甚至你想搞一点写成
fn test_ops_simple1(x: f64, y: f64) -> MathResult {
    Ok(sqrt(ln(div(x, y)?)?)?)
}

fn test_ops_result_simple(x: f64, y: f64) {
    match test_ops_simple(x, y) {
        Err(e) => {
            println!("test({}, {}) fail: {:?}", x, y, e);
        }
        Ok(val) => {
            println!("sqrt(ln({} / {})) = {}", x, y, val);
        }
    }
}

fn main() {
    test_ops_result(1.0, 0.0);
    test_ops_result(-1.0, 2.0);
    test_ops_result(1.0, 2.0);
    test_ops_result(5.0, 2.0);
    println!("");

    test_ops_result_simple(1.0, 0.0);
    test_ops_result_simple(-1.0, 2.0);
    test_ops_result_simple(1.0, 2.0);
    test_ops_result_simple(5.0, 2.0);
}
