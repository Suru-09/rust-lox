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

    generate_integration_test!(test_empty, &format!("{}{}", BLOCK_PREFIX, "/empty.lox"));

    generate_integration_test!(test_scope, &format!("{}{}", BLOCK_PREFIX, "/scope.lox"));
}

mod bool_tests {
    use crate::common::common::run_test;
    use crate::generate_integration_test;
    pub const BOOL_PREFIX: &str = "tests/resources/integration_tests/bool";

    generate_integration_test!(
        test_equality,
        &format!("{}{}", BOOL_PREFIX, "/equality.lox")
    );

    generate_integration_test!(test_not, &format!("{}{}", BOOL_PREFIX, "/not.lox"));
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

mod constructor_tests {
    use crate::common::common::run_test;
    use crate::generate_integration_test;
    pub const CONSTRUCTOR_PREFIX: &str = "tests/resources/integration_tests/constructor";

    generate_integration_test!(
        test_arguments,
        &format!("{}{}", CONSTRUCTOR_PREFIX, "/arguments.lox")
    );

    // generate_integration_test!(
    //     test_call_init_explicitly,
    //     &format!("{}{}", CONSTRUCTOR_PREFIX, "/call_init_explicitly.lox")
    // );

    generate_integration_test!(
        test_default_arguments,
        &format!("{}{}", CONSTRUCTOR_PREFIX, "/default_arguments.lox")
    );

    generate_integration_test!(
        test_default,
        &format!("{}{}", CONSTRUCTOR_PREFIX, "/default.lox")
    );

    generate_integration_test!(
        test_extra_arguments,
        &format!("{}{}", CONSTRUCTOR_PREFIX, "/extra_arguments.lox")
    );

    generate_integration_test!(
        test_init_not_method,
        &format!("{}{}", CONSTRUCTOR_PREFIX, "/init_not_method.lox")
    );

    generate_integration_test!(
        test_missing_arguments,
        &format!("{}{}", CONSTRUCTOR_PREFIX, "/missing_arguments.lox")
    );

    generate_integration_test!(
        test_return_in_nested_function,
        &format!("{}{}", CONSTRUCTOR_PREFIX, "/return_in_nested_function.lox")
    );

    generate_integration_test!(
        test_return_value,
        &format!("{}{}", CONSTRUCTOR_PREFIX, "/return_value.lox")
    );
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

    generate_integration_test!(
        test_close_over_method_parameter,
        &format!("{}{}", CLOSURE_PREFIX, "/close_over_method_parameter.lox")
    );

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
        &format!(
            "{}{}",
            CLOSURE_PREFIX, "/reference_closure_multiple_times.lox"
        )
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

    generate_integration_test!(test_scope, &format!("{}{}", FOR_PREFIX, "/scope.lox"));

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

    generate_integration_test!(test_syntax, &format!("{}{}", FOR_PREFIX, "/syntax.lox"));

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

    generate_integration_test!(test_else, &format!("{}{}", IF_PREFIX, "/else.lox"));

    generate_integration_test!(
        test_fun_in_else,
        &format!("{}{}", IF_PREFIX, "/fun_in_else.lox")
    );

    generate_integration_test!(
        test_fun_in_then,
        &format!("{}{}", IF_PREFIX, "/fun_in_then.lox")
    );

    generate_integration_test!(test_if, &format!("{}{}", IF_PREFIX, "/if.lox"));

    generate_integration_test!(test_truth, &format!("{}{}", IF_PREFIX, "/truth.lox"));

    generate_integration_test!(
        test_var_in_else,
        &format!("{}{}", IF_PREFIX, "/var_in_else.lox")
    );

    generate_integration_test!(
        test_var_in_then,
        &format!("{}{}", IF_PREFIX, "/var_in_then.lox")
    );
}

mod inheritance_tests {
    use crate::common::common::run_test;
    use crate::generate_integration_test;
    pub const INHERITANCE_PREFIX: &str = "tests/resources/integration_tests/inheritance";

    generate_integration_test!(
        test_constructor,
        &format!("{}{}", INHERITANCE_PREFIX, "/constructor.lox")
    );

    generate_integration_test!(
        test_inherit_from_function,
        &format!("{}{}", INHERITANCE_PREFIX, "/inherit_from_function.lox")
    );

    generate_integration_test!(
        test_inherit_from_nil,
        &format!("{}{}", INHERITANCE_PREFIX, "/inherit_from_nil.lox")
    );

    generate_integration_test!(
        test_inherit_from_number,
        &format!("{}{}", INHERITANCE_PREFIX, "/inherit_from_number.lox")
    );

    generate_integration_test!(
        test_inherit_methods,
        &format!("{}{}", INHERITANCE_PREFIX, "/inherit_methods.lox")
    );

    generate_integration_test!(
        test_paranthesized_superclass,
        &format!("{}{}", INHERITANCE_PREFIX, "/parenthesized_superclass.lox")
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

    generate_integration_test!(test_and, &format!("{}{}", LOPERATOR_PREFIX, "/and.lox"));

    generate_integration_test!(
        test_or_truth,
        &format!("{}{}", LOPERATOR_PREFIX, "/or_truth.lox")
    );

    generate_integration_test!(test_or, &format!("{}{}", LOPERATOR_PREFIX, "/or.lox"));
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

    generate_integration_test!(test_print, &format!("{}{}", FUNCTION_PREFIX, "/print.lox"));

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

mod method_tests {
    use crate::common::common::run_test;
    use crate::generate_integration_test;
    pub const METHOD_PREFIX: &str = "tests/resources/integration_tests/method";

    generate_integration_test!(test_arity, &format!("{}{}", METHOD_PREFIX, "/arity.lox"));

    generate_integration_test!(
        test_empty_block,
        &format!("{}{}", METHOD_PREFIX, "/empty_block.lox")
    );

    generate_integration_test!(
        test_extra_arguments,
        &format!("{}{}", METHOD_PREFIX, "/extra_arguments.lox")
    );

    generate_integration_test!(
        test_missing_arguments,
        &format!("{}{}", METHOD_PREFIX, "/missing_arguments.lox")
    );

    generate_integration_test!(
        test_not_found,
        &format!("{}{}", METHOD_PREFIX, "/not_found.lox")
    );

    generate_integration_test!(
        test_print_bound_method,
        &format!("{}{}", METHOD_PREFIX, "/print_bound_method.lox")
    );

    generate_integration_test!(
        test_refer_to_name,
        &format!("{}{}", METHOD_PREFIX, "/refer_to_name.lox")
    );

    generate_integration_test!(
        test_too_many_arguments,
        &format!("{}{}", METHOD_PREFIX, "/too_many_arguments.lox")
    );

    generate_integration_test!(
        test_too_many_parameters,
        &format!("{}{}", METHOD_PREFIX, "/too_many_parameters.lox")
    );
}

mod nil_tests {
    use crate::common::common::run_test;
    use crate::generate_integration_test;
    pub const NIL_PREFIX: &str = "tests/resources/integration_tests/nil";

    generate_integration_test!(test_nil, &format!("{}{}", NIL_PREFIX, "/literal.lox"));
}

mod number_tests {
    use crate::common::common::run_test;
    use crate::generate_integration_test;
    pub const NUMBER_PREFIX: &str = "tests/resources/integration_tests/number";

    generate_integration_test!(
        test_literals,
        &format!("{}{}", NUMBER_PREFIX, "/literals.lox")
    );

    generate_integration_test!(
        test_nan_equality,
        &format!("{}{}", NUMBER_PREFIX, "/nan_equality.lox")
    );
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

    generate_integration_test!(test_add, &format!("{}{}", OPERATOR_PREFIX, "/add.lox"));

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

    generate_integration_test!(
        test_equals_class,
        &format!("{}{}", OPERATOR_PREFIX, "/equals_class.lox")
    );

    generate_integration_test!(
        test_equals_method,
        &format!("{}{}", OPERATOR_PREFIX, "/equals_method.lox")
    );

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

    generate_integration_test!(
        test_not_class,
        &format!("{}{}", OPERATOR_PREFIX, "/not_class.lox")
    );

    generate_integration_test!(
        test_not_equals,
        &format!("{}{}", OPERATOR_PREFIX, "/not_equals.lox")
    );

    generate_integration_test!(test_not, &format!("{}{}", OPERATOR_PREFIX, "/not.lox"));

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

    generate_integration_test!(test_40, &format!("{}{}", REGRESSION_PREFIX, "/40.lox"));

    generate_integration_test!(test_394, &format!("{}{}", REGRESSION_PREFIX, "/394.lox"));
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

    generate_integration_test!(
        test_in_method,
        &format!("{}{}", RETURN_PREFIX, "/in_method.lox")
    );

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

mod super_tests {
    use crate::common::common::run_test;
    use crate::generate_integration_test;
    pub const SUPER_PREFIX: &str = "tests/resources/integration_tests/super";

    generate_integration_test!(
        test_bound_method,
        &format!("{}{}", SUPER_PREFIX, "/bound_method.lox")
    );

    generate_integration_test!(
        test_call_other_method,
        &format!("{}{}", SUPER_PREFIX, "/call_other_method.lox")
    );

    generate_integration_test!(
        test_call_same_method,
        &format!("{}{}", SUPER_PREFIX, "/call_same_method.lox")
    );

    generate_integration_test!(test_closure, &format!("{}{}", SUPER_PREFIX, "/closure.lox"));

    generate_integration_test!(
        test_constructor,
        &format!("{}{}", SUPER_PREFIX, "/constructor.lox")
    );

    generate_integration_test!(
        test_extra_arguments,
        &format!("{}{}", SUPER_PREFIX, "/extra_arguments.lox")
    );

    generate_integration_test!(
        test_indirectly_inherited,
        &format!("{}{}", SUPER_PREFIX, "/indirectly_inherited.lox")
    );

    generate_integration_test!(
        test_missing_arguments,
        &format!("{}{}", SUPER_PREFIX, "/missing_arguments.lox")
    );

    generate_integration_test!(
        test_no_superclass_bind,
        &format!("{}{}", SUPER_PREFIX, "/no_superclass_bind.lox")
    );

    generate_integration_test!(
        test_no_superclass_call,
        &format!("{}{}", SUPER_PREFIX, "/no_superclass_call.lox")
    );

    generate_integration_test!(
        test_no_superclass_method,
        &format!("{}{}", SUPER_PREFIX, "/no_superclass_method.lox")
    );

    generate_integration_test!(
        test_reassign_superclass,
        &format!("{}{}", SUPER_PREFIX, "/reassign_superclass.lox")
    );

    generate_integration_test!(
        test_super_at_top_level,
        &format!("{}{}", SUPER_PREFIX, "/super_at_top_level.lox")
    );

    generate_integration_test!(
        test_super_in_closure_in_inherited_method,
        &format!(
            "{}{}",
            SUPER_PREFIX, "/super_in_closure_in_inherited_method.lox"
        )
    );

    generate_integration_test!(
        test_super_in_inherited_method,
        &format!("{}{}", SUPER_PREFIX, "/super_in_inherited_method.lox")
    );

    generate_integration_test!(
        test_super_in_top_level_function,
        &format!("{}{}", SUPER_PREFIX, "/super_in_top_level_function.lox")
    );

    generate_integration_test!(
        test_super_without_dot,
        &format!("{}{}", SUPER_PREFIX, "/super_without_dot.lox")
    );

    generate_integration_test!(
        test_super_without_name,
        &format!("{}{}", SUPER_PREFIX, "/super_without_name.lox")
    );

    generate_integration_test!(
        test_this_in_superclass_method,
        &format!("{}{}", SUPER_PREFIX, "/this_in_superclass_method.lox")
    );
}

mod this_tests {
    use crate::common::common::run_test;
    use crate::generate_integration_test;
    pub const THIS_PREFIX: &str = "tests/resources/integration_tests/this";

    generate_integration_test!(test_closure, &format!("{}{}", THIS_PREFIX, "/closure.lox"));

    generate_integration_test!(
        test_nested_class,
        &format!("{}{}", THIS_PREFIX, "/nested_class.lox")
    );

    generate_integration_test!(
        test_nested_closure,
        &format!("{}{}", THIS_PREFIX, "/nested_closure.lox")
    );

    generate_integration_test!(
        test_this_at_top_level,
        &format!("{}{}", THIS_PREFIX, "/this_at_top_level.lox")
    );

    generate_integration_test!(
        test_this_in_method,
        &format!("{}{}", THIS_PREFIX, "/this_in_method.lox")
    );

    generate_integration_test!(
        test_this_at_top_level_function,
        &format!("{}{}", THIS_PREFIX, "/this_in_top_level_function.lox")
    );
}

mod variable_tests {
    use crate::common::common::run_test;
    use crate::generate_integration_test;
    pub const VARIABLE_PREFIX: &str = "tests/resources/integration_tests/variable";

    generate_integration_test!(
        test_collide_with_parameter,
        &format!("{}{}", VARIABLE_PREFIX, "/collide_with_parameter.lox")
    );

    generate_integration_test!(
        test_duplicate_local,
        &format!("{}{}", VARIABLE_PREFIX, "/duplicate_local.lox")
    );

    generate_integration_test!(
        test_duplicate_parameter,
        &format!("{}{}", VARIABLE_PREFIX, "/duplicate_parameter.lox")
    );

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

    generate_integration_test!(
        test_local_from_method,
        &format!("{}{}", VARIABLE_PREFIX, "/local_from_method.lox")
    );

    generate_integration_test!(
        test_redeclare_global,
        &format!("{}{}", VARIABLE_PREFIX, "/redeclare_global.lox")
    );

    generate_integration_test!(
        test_redefine_global,
        &format!("{}{}", VARIABLE_PREFIX, "/redefine_global.lox")
    );

    generate_integration_test!(
        test_scope_reuse_in_different_blocks,
        &format!(
            "{}{}",
            VARIABLE_PREFIX, "/scope_reuse_in_different_blocks.lox"
        )
    );

    generate_integration_test!(
        test_shadow_and_local,
        &format!("{}{}", VARIABLE_PREFIX, "/shadow_and_local.lox")
    );

    generate_integration_test!(
        test_shadow_global,
        &format!("{}{}", VARIABLE_PREFIX, "/shadow_global.lox")
    );

    generate_integration_test!(
        test_shadow_local,
        &format!("{}{}", VARIABLE_PREFIX, "/shadow_local.lox")
    );

    generate_integration_test!(
        test_undefined_global,
        &format!("{}{}", VARIABLE_PREFIX, "/undefined_global.lox")
    );

    generate_integration_test!(
        test_undefined_local,
        &format!("{}{}", VARIABLE_PREFIX, "/undefined_local.lox")
    );

    generate_integration_test!(
        test_uninitialized,
        &format!("{}{}", VARIABLE_PREFIX, "/uninitialized.lox")
    );

    generate_integration_test!(
        test_unreached_undefined,
        &format!("{}{}", VARIABLE_PREFIX, "/unreached_undefined.lox")
    );

    generate_integration_test!(
        test_use_false_as_var,
        &format!("{}{}", VARIABLE_PREFIX, "/use_false_as_var.lox")
    );

    generate_integration_test!(
        test_use_global_in_initializer,
        &format!("{}{}", VARIABLE_PREFIX, "/use_global_in_initializer.lox")
    );

    generate_integration_test!(
        test_use_local_in_initializer,
        &format!("{}{}", VARIABLE_PREFIX, "/use_local_in_initializer.lox")
    );

    generate_integration_test!(
        test_use_nil_as_var,
        &format!("{}{}", VARIABLE_PREFIX, "/use_nil_as_var.lox")
    );

    generate_integration_test!(
        test_use_this_as_var,
        &format!("{}{}", VARIABLE_PREFIX, "/use_this_as_var.lox")
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

    generate_integration_test!(test_syntax, &format!("{}{}", WHILE_PREFIX, "/syntax.lox"));

    generate_integration_test!(
        test_var_in_body,
        &format!("{}{}", WHILE_PREFIX, "/var_in_body.lox")
    );
}
