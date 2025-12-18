use tests::add_two;
mod common;
use common::setup;
#[test]
fn it_adds_two() {
    setup();
    let result = add_two(2);
    assert_eq!(result, 4);
}
