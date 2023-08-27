macro_rules! ok_or_return{
// match something(q,r,t,6,7,8) etc
// compiler extracts function name and arguments. It injects the values in respective varibles.
    ($a:ident($($b:tt)*))=>{
       {
        match $a($($b)*) {
            Ok(value)=>value,
            Err(err)=>{
                return Err(err);
            }
        }
        }
    };
}

fn some_work(i: i64, j: i64) -> Result<(i64, i64), String> {
    if i + j > 2 {
        Ok((i, j))
    } else {
        Err("error".to_owned())
    }
}

macro_rules! vec2 {
  // $()中表示模式，想想一下正则中的分组功能。
  // 其中expr表示匹配表达式，$item是将匹配的表达式进行命名。
  // 括号后面的逗号意味着可能有多个参数，
  // *号表示前面的所有字符（包括$()和,）出现0到多次（这里和一般的正则不同哦）
  ( $( $item:expr ),* ) => {
    {
      let mut temp_vec = Vec::new();
      $(
        temp_vec.push($item);
      )* // 匹配了多少次参数，就编译出多少次这个语句
      temp_vec
    }
  }
}

fn main() -> Result<(), String> {
    ok_or_return!(some_work(1, 4));
    // ok_or_return!(some_work(1, 0));

    let v2 = vec2![1, 2, 3];
    println!("{:?}", v2);
    // [1, 2, 3]

    Ok(())
}
