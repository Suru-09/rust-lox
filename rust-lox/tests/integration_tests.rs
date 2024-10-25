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

mod if_tests {
    use crate::common::common::run_test;
    use crate::generate_integration_test;
    pub const IF_PREFIX: &str = "tests/resources/integration_tests/if";

    generate_integration_test!(
        test_class_in_else,
        &format!("{}{}", IF_PREFIX, "/class_in_else.lox")
    );

    generate_integration_test!(
        test_class_in_then,
        &format!("{}{}", IF_PREFIX, "/class_in_then.lox")
    );

    generate_integration_test!(
        test_dangling_else,
        &format!("{}{}", IF_PREFIX, "/dangling_else.lox")
    );

    generate_integration_test!(
        test_else,
        &format!("{}{}", IF_PREFIX, "/else.lox")
    );

    generate_integration_test!(
        test_fun_in_else,
        &format!("{}{}", IF_PREFIX, "/fun_in_else.lox")
    );

    generate_integration_test!(
        test_fun_in_then,
        &format!("{}{}", IF_PREFIX, "/fun_in_then.lox")
    );

    generate_integration_test!(
        test_if,
        &format!("{}{}", IF_PREFIX, "/if.lox")
    );

    generate_integration_test!(
        test_truth,
        &format!("{}{}", IF_PREFIX, "/truth.lox")
    );

    generate_integration_test!(
        test_var_in_else,
        &format!("{}{}", IF_PREFIX, "/var_in_else.lox")
    );

    generate_integration_test!(
        test_var_in_then,
        &format!("{}{}", IF_PREFIX, "/var_in_then.lox")
    );
}

mod logical_operator_tests {
    use crate::common::common::run_test;
    use crate::generate_integration_test;
    pub const LOPERATOR_PREFIX: &str = "tests/resources/integration_tests/logical_operator";
    
    generate_integration_test!(
        test_and_truth,
        &format!("{}{}", LOPERATOR_PREFIX, "/and_truth.lox")
    );
    
    generate_integration_test!(
        test_and,
        &format!("{}{}", LOPERATOR_PREFIX, "/and.lox")
    );

    generate_integration_test!(
        test_or_truth,
        &format!("{}{}", LOPERATOR_PREFIX, "/or_truth.lox")
    );

    generate_integration_test!(
        test_or,
        &format!("{}{}", LOPERATOR_PREFIX, "/or.lox")
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

mod nil_tests {
    use crate::common::common::run_test;
    use crate::generate_integration_test;
    pub const NIL_PREFIX: &str = "tests/resources/integration_tests/nil";

    generate_integration_test!(
        test_nil,
        &format!("{}{}", NIL_PREFIX, "/literal.lox")
    );
}

mod number_tests {
    use crate::common::common::run_test;
    use crate::generate_integration_test;
    pub const NUMBER_PREFIX: &str = "tests/resources/integration_tests/number";


    // TODO: commented tested are related to the field property.
    // generate_integration_test!(
    //     test_decimal_point_at_eof,
    //     &format!("{}{}", NUMBER_PREFIX, "/decimal_point_at_eof.lox")
    // );

    // generate_integration_test!(
    //     test_leading_dot,
    //     &format!("{}{}", NUMBER_PREFIX, "/leading_dot.lox")
    // );

    generate_integration_test!(
        test_literals,
        &format!("{}{}", NUMBER_PREFIX, "/literals.lox")
    );

    generate_integration_test!(
        test_nan_equality,
        &format!("{}{}", NUMBER_PREFIX, "/nan_equality.lox")
    );

    // generate_integration_test!(
    //     test_trailing_dot,
    //     &format!("{}{}", NUMBER_PREFIX, "/trailing_dot.lox")
    // );
}

mod operator_tests {
    use crate::common::common::run_test;
    use crate::generate_integration_test;
    pub const OPERATOR_PREFIX: &str = "tests/resources/integration_tests/operator";

    generate_integration_test!(
        test_add_bool_nil,
        &format!("{}{}", OPERATOR_PREFIX, "/add_bool_nil.lox")
    );

    generate_integration_test!(
        test_add_bool_num,
        &format!("{}{}", OPERATOR_PREFIX, "/add_bool_num.lox")
    );

    generate_integration_test!(
        test_add_bool_string,
        &format!("{}{}", OPERATOR_PREFIX, "/add_bool_string.lox")
    );

    generate_integration_test!(
        test_add_nil_nil,
        &format!("{}{}", OPERATOR_PREFIX, "/add_nil_nil.lox")
    );

    generate_integration_test!(
        test_add_num_nil,
        &format!("{}{}", OPERATOR_PREFIX, "/add_num_nil.lox")
    );

    generate_integration_test!(
        test_add_string_nil,
        &format!("{}{}", OPERATOR_PREFIX, "/add_string_nil.lox")
    );

    generate_integration_test!(
        test_add,
        &format!("{}{}", OPERATOR_PREFIX, "/add.lox")
    );

    generate_integration_test!(
        test_comparison,
        &format!("{}{}", OPERATOR_PREFIX, "/comparison.lox")
    );

    generate_integration_test!(
        test_divide_nonnum_num,
        &format!("{}{}", OPERATOR_PREFIX, "/divide_nonnum_num.lox")
    );

    generate_integration_test!(
        test_divide_num_nonnum,
        &format!("{}{}", OPERATOR_PREFIX, "/divide_num_nonnum.lox")
    );

    generate_integration_test!(
        test_divide,
        &format!("{}{}", OPERATOR_PREFIX, "/divide.lox")
    );


    // TODO: add when finishing classes.
    // generate_integration_test!(
    //     test_equals_class,
    //     &format!("{}{}", OPERATOR_PREFIX, "/equals_class.lox")
    // );

    // TODO: when method exists.
    // generate_integration_test!(
    //     test_equals_method,
    //     &format!("{}{}", OPERATOR_PREFIX, "/equals_method.lox")
    // );

    generate_integration_test!(
        test_equals,
        &format!("{}{}", OPERATOR_PREFIX, "/equals.lox")
    );

    generate_integration_test!(
        test_greater_nonnum_num,
        &format!("{}{}", OPERATOR_PREFIX, "/greater_nonnum_num.lox")
    );

    generate_integration_test!(
        test_greater_num_nonnum,
        &format!("{}{}", OPERATOR_PREFIX, "/greater_num_nonnum.lox")
    );

    generate_integration_test!(
        test_greater_or_equal_nonum_num,
        &format!("{}{}", OPERATOR_PREFIX, "/greater_or_equal_nonnum_num.lox")
    );

    generate_integration_test!(
        test_greater_or_equal_num_nonum,
        &format!("{}{}", OPERATOR_PREFIX, "/greater_or_equal_num_nonnum.lox")
    );

    generate_integration_test!(
        test_less_nonnum_num,
        &format!("{}{}", OPERATOR_PREFIX, "/less_nonnum_num.lox")
    );

    generate_integration_test!(
        test_less_num_nonnum,
        &format!("{}{}", OPERATOR_PREFIX, "/less_num_nonnum.lox")
    );

    generate_integration_test!(
        test_less_or_equal_nonnum_num,
        &format!("{}{}", OPERATOR_PREFIX, "/less_or_equal_nonnum_num.lox")
    );

    generate_integration_test!(
        test_less_or_equal_num_nonnum,
        &format!("{}{}", OPERATOR_PREFIX, "/less_or_equal_num_nonnum.lox")
    );

    generate_integration_test!(
        test_multiply_nonnum_num,
        &format!("{}{}", OPERATOR_PREFIX, "/multiply_nonnum_num.lox")
    );

    generate_integration_test!(
        test_multiply_num_nonnum,
        &format!("{}{}", OPERATOR_PREFIX, "/multiply_num_nonnum.lox")
    );

    generate_integration_test!(
        test_multiply,
        &format!("{}{}", OPERATOR_PREFIX, "/multiply.lox")
    );

    generate_integration_test!(
        test_negate_nonnum,
        &format!("{}{}", OPERATOR_PREFIX, "/negate_nonnum.lox")
    );

    generate_integration_test!(
        test_negate,
        &format!("{}{}", OPERATOR_PREFIX, "/negate.lox")
    );


    //TODO: Enable when there is full support for classes.
    // generate_integration_test!(
    //     test_not_class,
    //     &format!("{}{}", OPERATOR_PREFIX, "/not_class.lox")
    // );

    generate_integration_test!(
        test_not_equals,
        &format!("{}{}", OPERATOR_PREFIX, "/not_equals.lox")
    );

    generate_integration_test!(
        test_not,
        &format!("{}{}", OPERATOR_PREFIX, "/not.lox")
    );

    generate_integration_test!(
        test_subtract_nonnum_num,
        &format!("{}{}", OPERATOR_PREFIX, "/subtract_nonnum_num.lox")
    );

    generate_integration_test!(
        test_subtract_num_nonnum,
        &format!("{}{}", OPERATOR_PREFIX, "/subtract_num_nonnum.lox")
    );

    generate_integration_test!(
        test_subtract,
        &format!("{}{}", OPERATOR_PREFIX, "/subtract.lox")
    );
}

mod print_tests {
    use crate::common::common::run_test;
    use crate::generate_integration_test;
    pub const PRINT_PREFIX: &str = "tests/resources/integration_tests/print";

    generate_integration_test!(
        test_missing_argument,
        &format!("{}{}", PRINT_PREFIX, "/missing_argument.lox")
    );
}

mod regression_tests {
    use crate::common::common::run_test;
    use crate::generate_integration_test;
    pub const REGRESSION_PREFIX: &str = "tests/resources/integration_tests/regression";

    generate_integration_test!(
        test_40,
        &format!("{}{}", REGRESSION_PREFIX, "/40.lox")
    );


    // TODO: enable when inheritance is available.
    // generate_integration_test!(
    //     test_394,
    //     &format!("{}{}", REGRESSION_PREFIX, "/394.lox")
    // );
}

mod root_folder_tests {
    pub const TESTS_PREFIX: &str = "tests/resources/integration_tests";
    use crate::common::common::run_test;
    use crate::generate_integration_test;
    
    generate_integration_test!(
        test_empty_file,
        &format!("{}{}", TESTS_PREFIX, "/empty_file.lox")
    );
    
    generate_integration_test!(
        test_precedence,
        &format!("{}{}", TESTS_PREFIX, "/precedence.lox")
    );
}

mod return_tests {
    use crate::common::common::run_test;
    use crate::generate_integration_test;
    pub const RETURN_PREFIX: &str = "tests/resources/integration_tests/return";

    generate_integration_test!(
        test_after_else,
        &format!("{}{}", RETURN_PREFIX, "/after_else.lox")
    );

    generate_integration_test!(
        test_after_if,
        &format!("{}{}", RETURN_PREFIX, "/after_if.lox")
    );

    generate_integration_test!(
        test_after_while,
        &format!("{}{}", RETURN_PREFIX, "/after_while.lox")
    );

    generate_integration_test!(
        test_at_top_level,
        &format!("{}{}", RETURN_PREFIX, "/at_top_level.lox")
    );

    generate_integration_test!(
        test_in_function,
        &format!("{}{}", RETURN_PREFIX, "/in_function.lox")
    );

    // TODO: enable when methods are available.
    // generate_integration_test!(
    //     test_in_method,
    //     &format!("{}{}", RETURN_PREFIX, "/in_method.lox")
    // );

    generate_integration_test!(
        test_return_nil_if_no_value,
        &format!("{}{}", RETURN_PREFIX, "/return_nil_if_no_value.lox")
    );
}

mod string_tests {
    use crate::common::common::run_test;
    use crate::generate_integration_test;
    pub const STRING_PREFIX: &str = "tests/resources/integration_tests/string";

    generate_integration_test!(
        test_error_after_multiline,
        &format!("{}{}", STRING_PREFIX, "/error_after_multiline.lox")
    );

    // TODO: add support for UNICODE.
    // generate_integration_test!(
    //     test_literals,
    //     &format!("{}{}", STRING_PREFIX, "/literals.lox")
    // );

    generate_integration_test!(
        test_multiline,
        &format!("{}{}", STRING_PREFIX, "/multiline.lox")
    );

    // TODO: support for SCAN errors in mod.rs
    // generate_integration_test!(
    //     test_unterminated,
    //     &format!("{}{}", STRING_PREFIX, "/unterminated.lox")
    // );
}

mod variable_tests {
    use crate::common::common::run_test;
    use crate::generate_integration_test;
    pub const VARIABLE_PREFIX: &str = "tests/resources/integration_tests/variable";


    // TODO: impprove resolver.
    // generate_integration_test!(
    //     test_collide_with_parameter,
    //     &format!("{}{}", VARIABLE_PREFIX, "/collide_with_parameter.lox")
    // );

    // generate_integration_test!(
    //     test_duplicate_local,
    //     &format!("{}{}", VARIABLE_PREFIX, "/duplicate_local.lox")
    // );

    // generate_integration_test!(
    //     test_duplicate_parameter,
    //     &format!("{}{}", VARIABLE_PREFIX, "/duplicate_parameter.lox")
    // );

    generate_integration_test!(
        test_early_bound,
        &format!("{}{}", VARIABLE_PREFIX, "/early_bound.lox")
    );

    generate_integration_test!(
        test_in_middle_of_block,
        &format!("{}{}", VARIABLE_PREFIX, "/in_middle_of_block.lox")
    );

    generate_integration_test!(
        test_in_nested_block,
        &format!("{}{}", VARIABLE_PREFIX, "/in_nested_block.lox")
    );
}

mod while_tests {
    use crate::common::common::run_test;
    use crate::generate_integration_test;
    pub const WHILE_PREFIX: &str = "tests/resources/integration_tests/while";

    generate_integration_test!(
        test_class_in_body,
        &format!("{}{}", WHILE_PREFIX, "/class_in_body.lox")
    );

    generate_integration_test!(
        test_closure_in_body,
        &format!("{}{}", WHILE_PREFIX, "/closure_in_body.lox")
    );

    generate_integration_test!(
        test_fun_in_body,
        &format!("{}{}", WHILE_PREFIX, "/fun_in_body.lox")
    );

    generate_integration_test!(
        test_return_closure,
        &format!("{}{}", WHILE_PREFIX, "/return_closure.lox")
    );
    generate_integration_test!(
        test_return_inside,
        &format!("{}{}", WHILE_PREFIX, "/return_inside.lox")
    );

    generate_integration_test!(
        test_syntax,
        &format!("{}{}", WHILE_PREFIX, "/syntax.lox")
    );

    generate_integration_test!(
        test_var_in_body,
        &format!("{}{}", WHILE_PREFIX, "/var_in_body.lox")
    );

}


