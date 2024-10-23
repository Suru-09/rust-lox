pub mod common;

use crate::common::common::{print_all_files, INTEGRATION_TESTS_PATH};

#[test]
fn test_print_all_files() {
    print_all_files(INTEGRATION_TESTS_PATH);
    assert!(true);
}
