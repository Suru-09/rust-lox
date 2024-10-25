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

mod block_tests {
    use crate::common::common::run_test;
    use crate::generate_integration_test;
    pub const BLOCK_PREFIX: &str = "tests/resources/integration_tests/block";

    generate_integration_test!(
        test_empty,
        &format!("{}{}", BLOCK_PREFIX, "/empty.lox")
    );

    generate_integration_test!(
        test_scope,
        &format!("{}{}", BLOCK_PREFIX, "/scope.lox")
    );
}

mod bool_tests {
    use crate::common::common::run_test;
    use crate::generate_integration_test;
    pub const BOOL_PREFIX: &str = "tests/resources/integration_tests/bool";

    generate_integration_test!(
        test_equality,
        &format!("{}{}", BOOL_PREFIX, "/equality.lox")
    );

    generate_integration_test!(
        test_not,
        &format!("{}{}", BOOL_PREFIX, "/not.lox")
    );
}

mod comment_tests {
    use crate::common::common::run_test;
    use crate::generate_integration_test;
    pub const COMMENT_PREFIX: &str = "tests/resources/integration_tests/comments";

    generate_integration_test!(
        test_line_at_eof,
        &format!("{}{}", COMMENT_PREFIX, "/line_at_eof.lox")
    );

    generate_integration_test!(
        test_only_line_comment_and_line,
        &format!("{}{}", COMMENT_PREFIX, "/only_line_comment_and_line.lox")
    );

    generate_integration_test!(
        test_only_line_comment,
        &format!("{}{}", COMMENT_PREFIX, "/only_line_comment.lox")
    );

    // TODO: allow unicode comments
    // generate_integration_test!(
    //     test_unicode,
    //     &format!("{}{}", COMMENT_PREFIX, "/unicode.lox")
    // );
}

mod expressions_test {
    use crate::common::common::run_test;
    use crate::generate_integration_test;
    pub const EXPRESSIONS_PREFIX: &str = "tests/resources/integration_tests/expressions";

    generate_integration_test!(
        test_evaluate,
        &format!("{}{}", EXPRESSIONS_PREFIX, "/evaluate.lox")
    );
}