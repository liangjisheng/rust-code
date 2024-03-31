// rand_distr crate 实现了诸多概率分布类型，如均匀分布、正态分布（Normal distribution）
// 柯西分布（Cauchy distribution）等。rand_distr crate 是 rand::distributions 模块
// 的一个超级集合

use rand::thread_rng;
use rand_distr::{Distribution, Normal, NormalError};

fn main() -> Result<(), NormalError> {
    let mut rng = thread_rng();
    // 正态分布
    let normal = Normal::new(2.0, 9.0)?;
    // Distribution::sample 方法创建一个迭代器，用来生成泛型 T 的随机值，其使用 rng 作为随机来源
    let v = normal.sample(&mut rng);

    println!("  正态分布： {}", v);

    Ok(())
}
