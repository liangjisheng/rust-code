// 数组的数据类型表示方式为[Type; N]
// [Type; N]是用来描述数据类型的，所以其中的N必须在编译期间就能确认，因此N不能是一个变量
// 数组字面量使用中括号[]表示，例如[1,2,3]。还有一种特殊的表示数组字面量的方式是[val; N]
// 这有点像数组类型的描述方式[Type; N]，不过这里表示的是该数组长度为N，并且这N个元素的值都初始化为val

fn main() {
    // 自动推导类型为：[i32; 4]
    let _arr = [11, 22, 33, 44];

    let _arr1: [&str; 3] = ["junma", "jinlong", "gaoxiao"];

    // 自动推导类型为：[u8; 1024]
    // 该数组初始化为1024个u8类型的0
    // 可将之当作以0填充的1K的buf空间
    let _arr2 = [0_u8; 1024];

    // 可以迭代数组，不过不能直接for i in arr{}
    // 而是 for i in &arr{} 或者 for i in arr.iter(){}
    let arr = [11, 22, 33, 44];
    for i in &arr {
        println!("{}", i);
    }
    println!();
    for i in arr.iter() {
        println!("{}", i);
    }
    println!();
    println!("{}", arr.len());
}
