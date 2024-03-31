// 使用 [num_cpus::get] 显示当前机器中的逻辑 CPU 内核的数量。

fn main() {
    println!("Number of logical cores is {}", num_cpus::get());
}
