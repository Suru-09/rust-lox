pub mod common;

use crate::common::common::{parse_tests, INTEGRATION_TESTS_PATH};

#[test]
fn test_run_tests() {
    println!("{:?}", parse_tests(INTEGRATION_TESTS_PATH));
    assert!(true);
}
