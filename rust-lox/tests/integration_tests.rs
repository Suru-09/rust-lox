pub mod common;

mod assignment_tests {
    use crate::common::common::run_test;
    use crate::generate_integration_test;
    pub const ASSIGNMENT_PREFIX: &str = "tests/resources/integration_tests/assignment";

    generate_integration_test!(
        test_associativity,
        &format!("{}{}", ASSIGNMENT_PREFIX, "/associativity.lox")
    );

    generate_integration_test!(
        test_global,
        &format!("{}{}", ASSIGNMENT_PREFIX, "/global.lox")
    );

    generate_integration_test!(
        test_grouping,
        &format!("{}{}", ASSIGNMENT_PREFIX, "/grouping.lox")
    );

    generate_integration_test!(
        test_infix_operator,
        &format!("{}{}", ASSIGNMENT_PREFIX, "/infix_operator.lox")
    );

    generate_integration_test!(
        test_local,
        &format!("{}{}", ASSIGNMENT_PREFIX, "/local.lox")
    );

    generate_integration_test!(
        test_prefix_operator,
        &format!("{}{}", ASSIGNMENT_PREFIX, "/prefix_operator.lox")
    );

    generate_integration_test!(
        test_syntax,
        &format!("{}{}", ASSIGNMENT_PREFIX, "/syntax.lox")
    );

    generate_integration_test!(
        test_to_this,
        &format!("{}{}", ASSIGNMENT_PREFIX, "/to_this.lox")
    );

    generate_integration_test!(
        test_undefined,
        &format!("{}{}", ASSIGNMENT_PREFIX, "/undefined.lox")
    );
}
