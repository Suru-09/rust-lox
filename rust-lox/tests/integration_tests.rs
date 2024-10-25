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

    // TODO: reintroduce when 'this' has been implemented.
    // generate_integration_test!(
    //     test_to_this,
    //     &format!("{}{}", ASSIGNMENT_PREFIX, "/to_this.lox")
    // );

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

mod closure_tests {
    use crate::common::common::run_test;
    use crate::generate_integration_test;
    pub const CLOSURE_PREFIX: &str = "tests/resources/integration_tests/closure";

    generate_integration_test!(
        test_assign_to_closure,
        &format!("{}{}", CLOSURE_PREFIX, "/assign_to_closure.lox")
    );

    generate_integration_test!(
        test_assign_to_shadowed_later,
        &format!("{}{}", CLOSURE_PREFIX, "/assign_to_shadowed_later.lox")
    );

    generate_integration_test!(
        test_close_over_function_parameter,
        &format!("{}{}", CLOSURE_PREFIX, "/close_over_function_parameter.lox")
    );

    generate_integration_test!(
        test_close_over_later_variable,
        &format!("{}{}", CLOSURE_PREFIX, "/close_over_later_variable.lox")
    );

    // TODO: enable the test when classes are fully implemented.
    // generate_integration_test!(
    //     test_close_over_method_parameter,
    //     &format!("{}{}", CLOSURE_PREFIX, "/close_over_method_parameter.lox")
    // );

    generate_integration_test!(
        test_closed_closure_in_function,
        &format!("{}{}", CLOSURE_PREFIX, "/closed_closure_in_function.lox")
    );

    generate_integration_test!(
        test_nested_closure,
        &format!("{}{}", CLOSURE_PREFIX, "/nested_closure.lox")
    );

    generate_integration_test!(
        test_open_closure_in_function,
        &format!("{}{}", CLOSURE_PREFIX, "/open_closure_in_function.lox")
    );

    generate_integration_test!(
        test_reference_closure_multiple_times,
        &format!("{}{}", CLOSURE_PREFIX, "/reference_closure_multiple_times.lox")
    );

    generate_integration_test!(
        test_reuse_closure_slot,
        &format!("{}{}", CLOSURE_PREFIX, "/reuse_closure_slot.lox")
    );

    generate_integration_test!(
        test_shadow_closure_with_local,
        &format!("{}{}", CLOSURE_PREFIX, "/shadow_closure_with_local.lox")
    );

    generate_integration_test!(
        test_unused_closure,
        &format!("{}{}", CLOSURE_PREFIX, "/unused_closure.lox")
    );

    generate_integration_test!(
        test_unused_later_closure,
        &format!("{}{}", CLOSURE_PREFIX, "/unused_later_closure.lox")
    );
}

mod for_tests {
    use crate::common::common::run_test;
    use crate::generate_integration_test;
    pub const FOR_PREFIX: &str = "tests/resources/integration_tests/for";

    generate_integration_test!(
        test_class_in_body,
        &format!("{}{}", FOR_PREFIX, "/class_in_body.lox")
    );

    generate_integration_test!(
        test_closure_in_body,
        &format!("{}{}", FOR_PREFIX, "/closure_in_body.lox")
    );

    generate_integration_test!(
        test_fun_in_body,
        &format!("{}{}", FOR_PREFIX, "/fun_in_body.lox")
    );

    generate_integration_test!(
        test_return_closure,
        &format!("{}{}", FOR_PREFIX, "/return_closure.lox")
    );

    generate_integration_test!(
        test_return_inside,
        &format!("{}{}", FOR_PREFIX, "/return_inside.lox")
    );

    generate_integration_test!(
        test_scope,
        &format!("{}{}", FOR_PREFIX, "/scope.lox")
    );

    generate_integration_test!(
        test_statement_condition,
        &format!("{}{}", FOR_PREFIX, "/statement_condition.lox")
    );

    generate_integration_test!(
        test_statement_increment,
        &format!("{}{}", FOR_PREFIX, "/statement_increment.lox")
    );

    generate_integration_test!(
        test_statement_initializer,
        &format!("{}{}", FOR_PREFIX, "/statement_initializer.lox")
    );

    generate_integration_test!(
        test_syntax,
        &format!("{}{}", FOR_PREFIX, "/syntax.lox")
    );

    generate_integration_test!(
        test_var_in_body,
        &format!("{}{}", FOR_PREFIX, "/var_in_body.lox")
    );
}

mod function_tests {
    use crate::common::common::run_test;
    use crate::generate_integration_test;
    pub const FUNCTION_PREFIX: &str = "tests/resources/integration_tests/function";

    generate_integration_test!(
        test_body_must_be_block,
        &format!("{}{}", FUNCTION_PREFIX, "/body_must_be_block.lox")
    );

    generate_integration_test!(
        test_empty_body,
        &format!("{}{}", FUNCTION_PREFIX, "/empty_body.lox")
    );

    generate_integration_test!(
        test_extra_arguments,
        &format!("{}{}", FUNCTION_PREFIX, "/extra_arguments.lox")
    );

    generate_integration_test!(
        test_local_mutual_recursion,
        &format!("{}{}", FUNCTION_PREFIX, "/local_mutual_recursion.lox")
    );

    generate_integration_test!(
        test_local_recursion,
        &format!("{}{}", FUNCTION_PREFIX, "/local_recursion.lox")
    );

    generate_integration_test!(
        test_missing_arguments,
        &format!("{}{}", FUNCTION_PREFIX, "/missing_arguments.lox")
    );

    generate_integration_test!(
        test_missing_comma_in_parameters,
        &format!("{}{}", FUNCTION_PREFIX, "/missing_comma_in_parameters.lox")
    );

    generate_integration_test!(
        test_nested_call_with_arguments,
        &format!("{}{}", FUNCTION_PREFIX, "/nested_call_with_arguments.lox")
    );

    generate_integration_test!(
        test_parameters,
        &format!("{}{}", FUNCTION_PREFIX, "/parameters.lox")
    );

    generate_integration_test!(
        test_print,
        &format!("{}{}", FUNCTION_PREFIX, "/print.lox")
    );

    generate_integration_test!(
        test_recursion,
        &format!("{}{}", FUNCTION_PREFIX, "/recursion.lox")
    );

    generate_integration_test!(
        test_too_many_arguments,
        &format!("{}{}", FUNCTION_PREFIX, "/too_many_arguments.lox")
    );

    generate_integration_test!(
        test_too_many_parameters,
        &format!("{}{}", FUNCTION_PREFIX, "/too_many_parameters.lox")
    );
}