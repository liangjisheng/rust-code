use num::bigint::Sign::{Minus, NoSign, Plus};
use num::BigInt;
use num::Num;
use num::Zero;

fn main() {
    test_from_bytes_be();
    test_to_bytes_be();
    test_from_bytes_le();
    test_to_bytes_le();
}

fn test_from_bytes_be() {
    fn check(s: &str, result: &str) {
        assert_eq!(
            BigInt::from_bytes_be(Plus, s.as_bytes()),
            BigInt::parse_bytes(result.as_bytes(), 10).unwrap()
        );
    }
    check("A", "65");
    check("AA", "16705");
    check("AB", "16706");
    check("Hello world!", "22405534230753963835153736737");
    assert_eq!(BigInt::from_bytes_be(Plus, &[]), BigInt::zero());
    assert_eq!(BigInt::from_bytes_be(Minus, &[]), BigInt::zero());
}

fn test_to_bytes_be() {
    fn check(s: &str, result: &str) {
        let b = BigInt::parse_bytes(result.as_bytes(), 10).unwrap();
        let (sign, v) = b.to_bytes_be();
        assert_eq!((Plus, s.as_bytes()), (sign, &*v));
    }
    check("A", "65");
    check("AA", "16705");
    check("AB", "16706");
    check("Hello world!", "22405534230753963835153736737");
    let b: BigInt = Zero::zero();
    assert_eq!(b.to_bytes_be(), (NoSign, vec![0]));

    // Test with leading/trailing zero bytes and a full BigDigit of value 0
    let b = BigInt::from_str_radix("00010000000000000200", 16).unwrap();
    assert_eq!(b.to_bytes_be(), (Plus, vec![1, 0, 0, 0, 0, 0, 0, 2, 0]));
}

fn test_from_bytes_le() {
    fn check(s: &str, result: &str) {
        assert_eq!(
            BigInt::from_bytes_le(Plus, s.as_bytes()),
            BigInt::parse_bytes(result.as_bytes(), 10).unwrap()
        );
    }
    check("A", "65");
    check("AA", "16705");
    check("BA", "16706");
    check("!dlrow olleH", "22405534230753963835153736737");
    assert_eq!(BigInt::from_bytes_le(Plus, &[]), BigInt::zero());
    assert_eq!(BigInt::from_bytes_le(Minus, &[]), BigInt::zero());
}

fn test_to_bytes_le() {
    fn check(s: &str, result: &str) {
        let b = BigInt::parse_bytes(result.as_bytes(), 10).unwrap();
        let (sign, v) = b.to_bytes_le();
        assert_eq!((Plus, s.as_bytes()), (sign, &*v));
    }
    check("A", "65");
    check("AA", "16705");
    check("BA", "16706");
    check("!dlrow olleH", "22405534230753963835153736737");
    let b: BigInt = Zero::zero();
    assert_eq!(b.to_bytes_le(), (NoSign, vec![0]));

    // Test with leading/trailing zero bytes and a full BigDigit of value 0
    let b = BigInt::from_str_radix("00010000000000000200", 16).unwrap();
    assert_eq!(b.to_bytes_le(), (Plus, vec![0, 2, 0, 0, 0, 0, 0, 0, 1]));
}
