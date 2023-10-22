mod m1 {
    #[derive(Debug, Default)]
    struct Point {
        x: i32,
        y: i32,
    }

    // Rhs == Self == Point
    impl PartialEq for Point {
        // impl automatically symmetric & transitive
        fn eq(&self, other: &Point) -> bool {
            self.x == other.x && self.y == other.y
        }
    }
}

mod m2 {
    // 如果一个类型的所有成员都实现了PartialEq，则它会派生实现PartialEq
    #[derive(PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[derive(PartialEq)]
    enum Suit {
        Spade,
        Heart,
        Club,
        Diamond,
    }

    // 一旦我们为自己的类型实现了PartialEq，我们就能够轻松地在类型的
    // 引用之间进行相等性比较，这要归功于 generic blanket impls

    // all of the generic blanket impls below
    // are provided by the standard library

    // this impl gives us: &Point == &Point
    // impl<A, B> PartialEq<&'_ B> for &'_ A
    //     where A: PartialEq<B> + ?Sized, B: ?Sized;

    // this impl gives us: &mut Point == &Point
    // impl<A, B> PartialEq<&'_ B> for &'_ mut A
    //     where A: PartialEq<B> + ?Sized, B: ?Sized;

    // this impl gives us: &Point == &mut Point
    // impl<A, B> PartialEq<&'_ mut B> for &'_ A
    //     where A: PartialEq<B> + ?Sized, B: ?Sized;

    // this impl gives us: &mut Point == &mut Point
    // impl<A, B> PartialEq<&'_ mut B> for &'_ mut A
    //     where A: PartialEq<B> + ?Sized, B: ?Sized;

    // 因为这个 trait 是泛型的，所以我们可以在不同的类型之间定义相等性（比较）。
    // 标准库利用这一点实现了类字符串类型之间的相互比较，比如String、&str、
    // PathBuf、&Path、OsString、&OsStr 等等
}

mod m3 {
    // 下面是一个反面实例，关于某人试图在没有满足上述规则的不同类型
    // 之间实现PartialEq用以检查完整性的例子
    #[derive(PartialEq)]
    enum Suit {
        Spade,
        Club,
        Heart,
        Diamond,
    }

    #[derive(PartialEq)]
    enum Rank {
        Ace,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Jack,
        Queen,
        King,
    }

    #[derive(PartialEq)]
    struct Card {
        suit: Suit,
        rank: Rank,
    }

    // check equality of Card's suit
    impl PartialEq<Suit> for Card {
        fn eq(&self, other: &Suit) -> bool {
            self.suit == *other
        }
    }

    // check equality of Card's rank
    impl PartialEq<Rank> for Card {
        fn eq(&self, other: &Rank) -> bool {
            self.rank == *other
        }
    }

    pub fn t3() {
        let AceOfSpades = Card {
            suit: Suit::Spade,
            rank: Rank::Ace,
        };
        assert!(AceOfSpades == Suit::Spade);
        assert!(AceOfSpades == Rank::Ace);
    }
}

// Eq是一个标记 trait，并且是PartialEq<Self>的一个 subtrait
// trait Eq: PartialEq<Self> {}
// 如果我们为一个类型实现了Eq，在PartialEq所要求的对称性和可传递性之上，
// 我们还保证了反射性（reflexivity），也就是对于任意的a，都有a == a。
// 从这种意义上来说，Eq对PartialEq进行了细化，因为它表示了一个更为严格的相等性。
// 如果一个类型的所有成员都实现了Eq，那么Eq的实现可以派生到这个类型。

// 浮点型实现了PartialEq但是没有实现Eq，因为NaN != NaN。几乎所有其他的实现了
// PartialEq的类型都实现了Eq，除非它们包含浮点类型。

// 一旦一个类型实现了PartialEq和Debug，我们可以就可以在assert_eq!宏中使用它。
// 我们还可以比较实现了PartialEq类型的集合
mod m4 {
    #[derive(PartialEq, Debug)]
    pub struct Point {
        x: i32,
        y: i32,
    }

    pub fn example_assert(p1: Point, p2: Point) {
        assert_eq!(p1, p2);
    }

    pub fn example_compare_collections<T: PartialEq>(vec1: Vec<T>, vec2: Vec<T>) {
        // if T: PartialEq this now works!
        if vec1 == vec2 {
            // some code
        } else {
            // other code
        }
    }
}

fn main() {
    m3::t3();
}
