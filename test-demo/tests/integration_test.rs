use test_demo;

mod common;

#[test]
fn it_adds_two() {
    assert_eq!(5, test_demo::add(2, 3));
}

#[test]
fn test1() {
    common::setup();
}
