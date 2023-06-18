mod infra;

// Your tests go here!
success_tests! {
    {
        name: false_val,
        file: "false_val.snek",
        expected: "false",
    },
    {
        name: input_compare_1,
        file: "input_compare.snek",
        input: "2",
        expected: "false",
    },
    {
        name: input_compare_2,
        file: "input_compare.snek",
        input: "10",
        expected: "true",
    },
    {
        name: basic_if,
        file: "if.snek",
        input: "",
        expected: "5",
    },
    {
        name: if_with_input1,
        file: "if_with_input.snek",
        input: "1",
        expected: "20",
    },
    {
        name: if_with_input2,
        file: "if_with_input.snek",
        input: "2",
        expected: "5",
    },
    {
        name: basic_add,
        file: "add.snek",
        expected: "13",
    },
    {
        name: add1,
        file: "add1.snek",
        expected: "73",
    },
    {
        name: binding_nested,
        file: "binding_nested.snek",
        expected: "18",
    },
    {
        name: shdowed_diff_lawers,
        file: "shdowed_diff_lawers.snek",
        expected: "6",
    },
    {
        name: basic_loop,
        file: "basic_loop.snek",
        expected: "5",
    },
}

runtime_error_tests! {
    {
        name: invalid_argument,
        file: "invalid_argument.snek",
        expected: "invalid argument",
    },
    {
        name: input_compare_3,
        file: "input_compare.snek",
        input: "true",
        expected: "invalid argument",
    },
    {
        name: add_overflow,
        file: "add_overflow.snek",
        expected: "overflow",
    },
}

static_error_tests! {
    {
        name: number_bounds_fail,
        file: "number_bounds_fail.snek",
        expected: "Invalid",
    }
}
