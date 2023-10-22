// trait Index<Idx: ?Sized> {
//     type Output: ?Sized;
//     fn index(&self, index: Idx) -> &Self::Output;
// }
//
// trait IndexMut<Idx>: Index<Idx> where Idx: ?Sized {
//     fn index_mut(&mut self, index: Idx) -> &mut Self::Output;
// }

// 我们可以将 [] 索引到带有 T 值的 Index<T, Output = U> 类型，索引操作将返回 &U 值
// 为了语法方便，编译器会自动在索引操作返回值的前面插入一个解引用操作符 *

fn f1() {
    // Vec<i32> impls Index<usize, Output = i32> so
    // indexing Vec<i32> should produce &i32s and yet...
    let vec = vec![1, 2, 3, 4, 5];
    // let num_ref: &i32 = vec[0]; // ❌ expected &i32 found i32

    // above line actually desugars to
    // let num_ref: &i32 = *vec[0]; // ❌ expected &i32 found i32

    // both of these alternatives work
    let num: i32 = vec[0]; // ✅
    let num_ref = &vec[0]; // ✅
}

// 为了展示我们自己如何实现Index，下面是一个有趣的示例，这个例子展示了我们
// 如何使用一个新类型和 Index trait 在 Vec 上实现环绕索引和负索引
use std::ops::Index;

struct WrappingIndex<T>(Vec<T>);

impl<T> Index<usize> for WrappingIndex<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        &self.0[index % self.0.len()]
    }
}

impl<T> Index<i128> for WrappingIndex<T> {
    type Output = T;
    fn index(&self, index: i128) -> &T {
        let self_len = self.0.len() as i128;
        let idx = (((index % self_len) + self_len) % self_len) as usize;
        &self.0[idx]
    }
}

fn indexes() {
    let wrapping_vec = WrappingIndex(vec![1, 2, 3]);
    assert_eq!(1, wrapping_vec[0_usize]);
    assert_eq!(2, wrapping_vec[1_usize]);
    assert_eq!(3, wrapping_vec[2_usize]);
}

fn wrapping_indexes() {
    let wrapping_vec = WrappingIndex(vec![1, 2, 3]);
    assert_eq!(1, wrapping_vec[3_usize]);
    assert_eq!(2, wrapping_vec[4_usize]);
    assert_eq!(3, wrapping_vec[5_usize]);
}

fn neg_indexes() {
    let wrapping_vec = WrappingIndex(vec![1, 2, 3]);
    assert_eq!(1, wrapping_vec[-3_i128]);
    assert_eq!(2, wrapping_vec[-2_i128]);
    assert_eq!(3, wrapping_vec[-1_i128]);
}

fn wrapping_neg_indexes() {
    let wrapping_vec = WrappingIndex(vec![1, 2, 3]);
    assert_eq!(1, wrapping_vec[-6_i128]);
    assert_eq!(2, wrapping_vec[-5_i128]);
    assert_eq!(3, wrapping_vec[-4_i128]);
}

// 这里没有要求Idx类型是数值类型或者是一个Range，它也可以是一个枚举！
// 下面是一个使用篮球位置在一支球队里检索球员的例子

enum BasketballPosition {
    PointGuard,
    ShootingGuard,
    Center,
    PowerForward,
    SmallForward,
}

struct BasketballPlayer {
    name: &'static str,
    position: BasketballPosition,
}

struct BasketballTeam {
    point_guard: BasketballPlayer,
    shooting_guard: BasketballPlayer,
    center: BasketballPlayer,
    power_forward: BasketballPlayer,
    small_forward: BasketballPlayer,
}

impl Index<BasketballPosition> for BasketballTeam {
    type Output = BasketballPlayer;
    fn index(&self, position: BasketballPosition) -> &BasketballPlayer {
        match position {
            BasketballPosition::PointGuard => &self.point_guard,
            BasketballPosition::ShootingGuard => &self.shooting_guard,
            BasketballPosition::Center => &self.center,
            BasketballPosition::PowerForward => &self.power_forward,
            BasketballPosition::SmallForward => &self.small_forward,
        }
    }
}

fn basketball() {
    let team = BasketballTeam {
        point_guard: BasketballPlayer {
            name: "PointGuard",
            position: BasketballPosition::PointGuard,
        },
        shooting_guard: BasketballPlayer {
            name: "ShootingGuard",
            position: BasketballPosition::ShootingGuard,
        },
        center: BasketballPlayer {
            name: "Center",
            position: BasketballPosition::Center,
        },
        power_forward: BasketballPlayer {
            name: "PowerForward",
            position: BasketballPosition::PowerForward,
        },
        small_forward: BasketballPlayer {
            name: "SmallForward",
            position: BasketballPosition::SmallForward,
        },
    };

    let point: &BasketballPlayer = &team[BasketballPosition::PointGuard];
    println!("{}", point.name);
}

fn main() {
    // indexes();
    // wrapping_indexes();
    // neg_indexes();
    // wrapping_neg_indexes();

    basketball();
}
