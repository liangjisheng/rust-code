#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(bad_add(1, 2), 2);
    }
}

fn add(a: i8, b: i8) -> i8 {
    return a + b;
}

fn bad_add(a: i8, b: i8) -> i8 {
    return a * b;
}
