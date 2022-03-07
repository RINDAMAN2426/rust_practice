use chapter11_Writing_Automated_Tests as adder;

mod common;

#[test]
fn it_adds_two() {
  common::setup();
  assert_eq!(4, adder::add_two(2));
}