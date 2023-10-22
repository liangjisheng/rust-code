// trait Borrow<Borrowed>
//     where
//         Borrowed: ?Sized,
// {
//     fn borrow(&self) -> &Borrowed;
// }

// trait BorrowMut<Borrowed>: Borrow<Borrowed>
//     where
//         Borrowed: ?Sized,
// {
//     fn borrow_mut(&mut self) -> &mut Borrowed;
// }

// 这些 trait 被发明用于解决非常具体的问题，即使用 &str 类型的值在
// HashSet、HashMap、BTreeSet和BTreeMap中查找String类型的 key

// 我们可以把Borrow<T>和BorrowMut<T>看作更严格的AsRef<T>和AsMut<T>，
// 它们返回的引用&T与Self有等价性的Eq、Hash和Ord实现

use std::borrow::Borrow;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

fn get_hash<T: Hash>(t: T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

fn asref_example<Owned, Ref>(owned1: Owned, owned2: Owned)
where
    Owned: Eq + Ord + Hash + AsRef<Ref>,
    Ref: Eq + Ord + Hash,
{
    let ref1: &Ref = owned1.as_ref();
    let ref2: &Ref = owned2.as_ref();

    // refs aren't required to be equal if owned types are equal
    assert_eq!(owned1 == owned2, ref1 == ref2); // ✅

    let owned1_hash = get_hash(&owned1);
    let owned2_hash = get_hash(&owned2);
    let ref1_hash = get_hash(&ref1);
    let ref2_hash = get_hash(&ref2);

    // ref hashes aren't required to be equal if owned type hashes are equal
    assert_eq!(owned1_hash == owned2_hash, ref1_hash == ref2_hash); // ✅

    // ref comparisons aren't required to match owned type comparisons
    assert_eq!(owned1.cmp(&owned2), ref1.cmp(&ref2)); // ✅
}

fn borrow_example<Owned, Borrowed>(owned1: Owned, owned2: Owned)
where
    Owned: Eq + Ord + Hash + Borrow<Borrowed>,
    Borrowed: Eq + Ord + Hash,
{
    let borrow1: &Borrowed = owned1.borrow();
    let borrow2: &Borrowed = owned2.borrow();

    // borrows are required to be equal if owned types are equal
    assert_eq!(owned1 == owned2, borrow1 == borrow2); // ✅

    let owned1_hash = get_hash(&owned1);
    let owned2_hash = get_hash(&owned2);
    let borrow1_hash = get_hash(&borrow1);
    let borrow2_hash = get_hash(&borrow2);

    // borrow hashes are required to be equal if owned type hashes are equal
    assert_eq!(owned1_hash == owned2_hash, borrow1_hash == borrow2_hash); // ✅

    // borrow comparisons are required to match owned type comparisons
    assert_eq!(owned1.cmp(&owned2), borrow1.cmp(&borrow2)); // ✅
}

#[derive(Hash, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl AsRef<Point> for Point {
    fn as_ref(&self) -> &Point {
        self
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Point { x, y }
    }
}

fn main() {
    let s1 = Point::new(1, 2);
    let s2 = Point::from((3, 4));

    asref_example(s1.clone(), s2.clone());
    borrow_example(s1, s2);
}

// 意识到这些 trait 以及它们为什么存在是有益的，因为它有助于搞清楚
// HashSet、HashMap、BTreeSet以及BTreeMap的某些方法，但是我们
// 很少需要为我们的类型实现这些 trait，因为我们很少需要创建一对儿类型，
// 其中一个是另一个的借用版本。如果我们有某个类型T，&T在 99.99%的
// 情况下可以完成工作，并且因为 generic blanket impl，T:Borrow<T>
// 已经为所有的类型T实现了，所以我们不需要手动地实现它并且我们不需要创建
// 一个U以用来T:Borrow<U>
