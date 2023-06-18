mod infra;

// Your tests go here!
success_tests! {
    // ========================================================= //
    // TEST ADDITION                                             //
    // ========================================================= //
    {
        name: test_basic_add1,
        file: "test_basic_add1.snek",
        expected: "15",
    },
    {
        name: test_basic_add2,
        file: "test_basic_add2.snek",
        expected: "539",
    },
    {
        name: test_basic_add3,
        file: "test_basic_add3.snek",
        expected: "152",
    },
    {
        name: test_basic_add4,
        file: "test_basic_add4.snek",
        expected: "134",
    },
    {
        name: test_basic_add5,
        file: "test_basic_add5.snek",
        expected: "3",
    },
    {
        name: test_basic_add6,
        file: "test_basic_add6.snek",
        expected: "0",
    },
    // ========================================================= //
    // TEST MULTIPLICATION                                       //
    // ========================================================= //
    {
        name: test_basic_mult1,
        file: "test_basic_mult1.snek",
        expected: "30",
    },
    {
        name: test_basic_mult2,
        file: "test_basic_mult2.snek",
        expected: "-81000",
    },
    // ========================================================= //
    // TEST SUBTRACTION                                          //
    // ========================================================= //
    {
        name: test_basic_sub1,
        file: "test_basic_sub1.snek",
        expected: "3",
    },
    {
        name: test_basic_sub2,
        file: "test_basic_sub2.snek",
        expected: "2",
    },
    // ========================================================= //
    // TEST MIXED BINARY OPERATIONS                              //
    // ========================================================= //
    {
        name: test_binary_mixed1,
        file: "test_binary_mixed1.snek",
        expected: "-490",
    },
    {
        name: test_binops_1,
        file: "test_binops_1.snek",
        expected: "50",
    },
    {
        name: test_binops_2,
        file: "test_binops_2.snek",
        expected: "120124545156",
    },
    // ========================================================= //
    // TEST BLOCKS                                               //
    // ========================================================= //
    {
        name: test_block1,
        file: "test_block1.snek",
        expected: "19",
    },
    {
        name: test_block2,
        file: "test_block2.snek",
        expected: "40",
    },
    // ========================================================= //
    // TEST BOOLEANS                                             //
    // ========================================================= //
    {
        name: test_bool1,
        file: "test_bool1.snek",
        expected: "true",
    },
    {
        name: test_bool2,
        file: "test_bool2.snek",
        expected: "false",
    },
    {
        name: test_bool3,
        file: "test_bool3.snek",
        expected: "false",
    },
    {
        name: test_bool4,
        file: "test_bool4.snek",
        expected: "true",
    },
    {
        name: test_bool5,
        file: "test_bool5.snek",
        expected: "true",
    },
    {
        name: test_bool6,
        file: "test_bool6.snek",
        expected: "true",
    },
    {
        name: test_bool7,
        file: "test_bool7.snek",
        expected: "false",
    },
    // ========================================================= //
    // TEST IF-EXPRESSIONS                                       //
    // ========================================================= //
    {
        name: test_if1,
        file: "test_if1.snek",
        expected: "5",
    },
    {
        name: test_if2,
        file: "test_if2.snek",
        expected: "10",
    },
    {
        name: test_if3,
        file: "test_if3.snek",
        expected: "100",
    },
    {
        name: test_if4,
        file: "test_if4.snek",
        expected: "25",
    },
    {
        name: test_if5,
        file: "test_if5.snek",
        expected: "25",
    },
    {
        name: test_if_num_0,
        file: "test_if6.snek",
        input: "0",
        expected: "true",
    },
    {
        name: test_if_num_2,
        file: "test_if6.snek",
        input: "2",
        expected: "true",
    },
    {
        name: test_if_num_neg2,
        file: "test_if6.snek",
        input: "-2",
        expected: "true",
    },
    {
        name: test_if_true,
        file: "test_if6.snek",
        input: "true",
        expected: "true",
    },
    {
        name: test_if_false,
        file: "test_if6.snek",
        input: "false",
        expected: "false",
    },
    // ========================================================= //
    // TEST INPUTS                                               //
    // ========================================================= //
    {
        name: test_input1,
        file: "test_input1.snek",
        input: "true",
        expected: "true",
    },
    {
        name: num_squared_32,
        file: "num_squared.snek",
        input: "32",
        expected: "1024",
    },
    {
        name: num_squared_neg32,
        file: "num_squared.snek",
        input: "-32",
        expected: "1024",
    },
    // ========================================================= //
    // TEST LET EXPRESSIONS                                      //
    // ========================================================= //
    {
        name: test_let1,
        file: "test_let1.snek",
        expected: "6",
    },
    {
        name: test_let2,
        file: "test_let2.snek",
        expected: "6",
    },
    {
        name: test_let3,
        file: "test_let3.snek",
        expected: "5",
    },
    {
        name: test_let4,
        file: "test_let4.snek",
        expected: "111",
    },
    {
        name: test_let5,
        file: "test_let5.snek",
        expected: "-90",
    },
    {
        name: test_let6,
        file: "test_let6.snek",
        expected: "50",
    },
    {
        name: test_shadow_let1,
        file: "test_shadow_let1.snek",
        expected: "50",
    },
    {
        name: test_shadow_let2,
        file: "test_shadow_let2.snek",
        expected: "95",
    },
    // ========================================================= //
    // TEST LOOPS                                                //
    // ========================================================= //
    {
        name: test_loop1,
        file: "test_loop1.snek",
        expected: "26",
    },
    {
        name: test_loop2,
        file: "test_loop2.snek",
        expected: "19",
    },
    {
        name: test_loop3,
        file: "test_loop3.snek",
        expected: "-6",
    },
    {
        name: test_loop_factorial_4,
        file: "factorial.snek",
        input: "4",
        expected: "24",
    },
    {
        name: test_loop_factorial_9,
        file: "factorial.snek",
        input: "9",
        expected: "362880",
    },
    {
        name: test_loop_factorial_0,
        file: "factorial.snek",
        input: "0",
        expected: "1",
    },
    {
        name: test_loop_factorial_2,
        file: "factorial.snek",
        input: "2",
        expected: "2",
    },
    {
        name: test_loop_factorial_5,
        file: "factorial.snek",
        input: "5",
        expected: "120",
    },
    {
        name: test_loop_factorial_20,
        file: "factorial.snek",
        input: "20",
        expected: "2432902008176640000",
    },
    {
        name: test_loop_fibonacci_0,
        file: "fibonacci_iterative.snek",
        input: "0",
        expected: "0",
    },
    {
        name: test_loop_fibonacci_1,
        file: "fibonacci_iterative.snek",
        input: "1",
        expected: "1",
    },
    {
        name: test_loop_fibonacci_16,
        file: "fibonacci_iterative.snek",
        input: "16",
        expected: "987",
    },
    {
        name: test_loop_fibonacci_19,
        file: "fibonacci_iterative.snek",
        input: "19",
        expected: "4181",
    },
    {
        name: test_loop_fibonacci_11,
        file: "fibonacci_iterative.snek",
        input: "11",
        expected: "89",
    },
    {
        name: test_count_primes_10,
        file: "count_primes_less_than_eq.snek",
        input: "10",
        expected: "4",
    },
    {
        name: test_count_primes_928,
        file: "count_primes_less_than_eq.snek",
        input: "928",
        expected: "157",
    },
    {
        name: test_count_primes_9280,
        file: "count_primes_less_than_eq.snek",
        input: "9280",
        expected: "1148",
    },
    {
        name: test_count_primes_84,
        file: "count_primes_less_than_eq.snek",
        input: "84",
        expected: "23",
    },
    {
        name: test_count_primes_131,
        file: "count_primes_less_than_eq.snek",
        input: "131",
        expected: "32",
    },
    {
        name: test_count_primes_12345,
        file: "count_primes_less_than_eq.snek",
        input: "12345",
        expected: "1474",
    },
    {
        name: test_count_primes_846,
        file: "count_primes_less_than_eq.snek",
        input: "846",
        expected: "146",
    },
    {
        name: test_is_even_32,
        file: "is_even.snek",
        input: "32",
        expected: "true",
    },
    {
        name: test_is_even_999,
        file: "is_even.snek",
        input: "999",
        expected: "false",
    },
    {
        name: test_is_even_neg32,
        file: "is_even.snek",
        input: "-32",
        expected: "true",
    },
    {
        name: test_is_even_neg999,
        file: "is_even.snek",
        input: "-999",
        expected: "false",
    },
    {
        name: test_is_even_0,
        file: "is_even.snek",
        input: "0",
        expected: "true",
    },
    {
        name: test_perfect_number_28,
        file: "perfect_number.snek",
        input: "28",
        expected: "true",
    },
    {
        name: test_perfect_number_5,
        file: "perfect_number.snek",
        input: "5",
        expected: "false",
    },
    {
        name: test_perfect_number_56,
        file: "perfect_number.snek",
        input: "56",
        expected: "false",
    },
    {
        name: test_perfect_number_8128,
        file: "perfect_number.snek",
        input: "8128",
        expected: "true",
    },
    {
        name: test_perfect_number_496,
        file: "perfect_number.snek",
        input: "496",
        expected: "true",
    },
    {
        name: test_perfect_number_6,
        file: "perfect_number.snek",
        input: "6",
        expected: "true",
    },
    {
        name: test_perfect_number_62,
        file: "perfect_number.snek",
        input: "62",
        expected: "false",
    },
    {
        name: test_sum_of_digit_1,
        file: "sum_of_digits.snek",
        input: "1",
        expected: "1",
    },
    {
        name: test_sum_of_digit_neg4,
        file: "sum_of_digits.snek",
        input: "-4",
        expected: "-4",
    },
    {
        name: test_sum_of_digit_1555,
        file: "sum_of_digits.snek",
        input: "1555",
        expected: "16",
    },
    {
        name: test_sum_of_digit_461168,
        file: "sum_of_digits.snek",
        input: "461168",
        expected: "26",
    },
    {
        name: test_sum_of_digit_neg461168,
        file: "sum_of_digits.snek",
        input: "-461168",
        expected: "-26",
    },
    {
        name: test_sum_of_digit_31415926,
        file: "sum_of_digits.snek",
        input: "31415926",
        expected: "31",
    },
    {
        name: test_sum_of_digit_neg31415926,
        file: "sum_of_digits.snek",
        input: "-31415926",
        expected: "-31",
    },
    {
        name: test_collatz_123,
        file: "collatz.snek",
        input: "123",
        expected: "47",
    },
    {
        name: test_collatz_1234,
        file: "collatz.snek",
        input: "1234",
        expected: "133",
    },
    {
        name: test_collatz_27,
        file: "collatz.snek",
        input: "27",
        expected: "112",
    },
    {
        name: test_collatz_12,
        file: "collatz.snek",
        input: "12",
        expected: "10",
    },
    {
        name: test_collatz_19,
        file: "collatz.snek",
        input: "19",
        expected: "21",
    },
    {
        name: test_collatz_999,
        file: "collatz.snek",
        input: "999",
        expected: "50",
    },
    {
        name: test_reverse_digits_123456,
        file: "reverse_digits.snek",
        input: "123456",
        expected: "654321",
    },
    {
        name: test_reverse_digits_131,
        file: "reverse_digits.snek",
        input: "131",
        expected: "131",
    },
    {
        name: test_reverse_digits_81267,
        file: "reverse_digits.snek",
        input: "81267",
        expected: "76218",
    },
    {
        name: test_reverse_digits_neg293,
        file: "reverse_digits.snek",
        input: "-293",
        expected: "-392",
    },
    {
        name: test_reverse_digits_neg725610,
        file: "reverse_digits.snek",
        input: "-725610",
        expected: "-16527",
    },
    {
        name: test_reverse_digits_neg50000,
        file: "reverse_digits.snek",
        input: "-50000",
        expected: "-5",
    },
    {
        name: test_reverse_digits_neg82000,
        file: "reverse_digits.snek",
        input: "82000",
        expected: "28",
    },
    // ========================================================= //
    // TEST SET                                                  //
    // ========================================================= //
    {
        name: test_set1,
        file: "test_set1.snek",
        expected: "5",
    },
    {
        name: test_set2,
        file: "test_set2.snek",
        expected: "17",
    },
    {
        name: test_set3,
        file: "test_set3.snek",
        expected: "6",
    },
    {
        name: test_set4,
        file: "test_set4.snek",
        expected: "92",
    },
    // Tests using set! with shadowed variables
    {
        name: test_set5,
        file: "test_set5.snek",
        expected: "111",
    },
    // ========================================================= //
    // TEST UNARY                                                //
    // ========================================================= //
    {
        name: test_unary1,
        file: "test_unary1.snek",
        expected: "2",
    },
    {
        name: test_unary2,
        file: "test_unary2.snek",
        expected: "96",
    },
    {
        name: test_unary3,
        file: "test_unary3.snek",
        expected: "-124",
    },
    {
        name: test_unary4,
        file: "test_unary4.snek",
        expected: "-1000",
    },
    {
        name: test_unary5,
        file: "test_unary5.snek",
        expected: "true",
    },
    {
        name: test_unary6,
        file: "test_unary6.snek",
        expected: "true",
    },
    {
        name: test_unary7,
        file: "test_unary7.snek",
        expected: "false",
    },
    {
        name: test_unary8,
        file: "test_unary8.snek",
        expected: "false",
    },
    {
        name: test_unary9,
        file: "test_unary9.snek",
        expected: "false",
    },
    {
        name: test_unary10,
        file: "test_unary10.snek",
        expected: "false",
    },
    {
        name: test_unary11,
        file: "test_unary11.snek",
        expected: "true",
    },
    {
        name: test_unary12,
        file: "test_unary12.snek",
        expected: "true",
    },
}

runtime_error_tests! {
    {
        name: test_loop_factorial_fail_input1,
        file: "factorial.snek",
        input: "true",
        expected: "invalid argument",
    },
    {
        name: test_loop_factorial_fail_input2,
        file: "factorial.snek",
        input: "false",
        expected: "invalid argument",
    },
    {
        name: test_fail_add1,
        file: "test_fail_add1.snek",
        expected: "invalid argument",
    },
    {
        name: test_fail_binops1,
        file: "test_fail_binops1.snek",
        expected: "invalid argument",
    },
    {
        name: test_fail_bool1,
        file: "test_fail_bool1.snek",
        expected: "invalid argument",
    },
    {
        name: test_fail_bool2,
        file: "test_fail_bool2.snek",
        expected: "invalid argument",
    },
    {
        name: test_fail_if1,
        file: "test_fail_if1.snek",
        expected: "invalid argument",
    },
    // ========================================================= //
    // TEST OVERFLOW                                             //
    // ========================================================= //
    {
        name: test_fail_overflow2,
        file: "test_fail_overflow2.snek",
        expected: "overflow",
    },
    {
        name: test_fail_overflow_factorial,
        file: "factorial.snek",
        input: "21",
        expected: "overflow",
    },
    // ========================================================= //
    // TEST BAD UNARY                                            //
    // ========================================================= //
    {
        name: test_fail_unary1,
        file: "test_fail_unary1.snek",
        expected: "invalid argument",
    },
    {
        name: test_fail_unary2,
        file: "test_fail_unary2.snek",
        expected: "invalid argument",
    },
}

static_error_tests! {
    {
        name: test_fail_number_bounds1,
        file: "test_fail_number_bounds1.snek",
        expected: "Invalid",
    },
    {
        name: test_fail_number_bounds2,
        file: "test_fail_number_bounds2.snek",
        expected: "Invalid",
    },
    {
        name: test_break_out_loop1,
        file: "test_fail_break1.snek",
        expected: "break",
    },
    {
        name: test_fail_let1,
        file: "test_fail_let1.snek",
        expected: "Unbound variable identifier t",
    },
    {
        name: test_fail_let2,
        file: "test_fail_let2.snek",
        expected: "Duplicate binding",
    },
    {
        name: test_fail_let3,
        file: "test_fail_let3.snek",
        expected: "Invalid",
    },
    {
        name: test_fail_unbound1,
        file: "test_fail_unbound1.snek",
        expected: "Unbound variable identifier x",
    },
    {
        name: test_fail_set1,
        file: "test_fail_set1.snek",
        expected: "Unbound variable identifier y",
    },
    {
        name: test_fail_parse_overflow1,
        file: "test_fail_overflow1.snek",
        expected: "overflow",
    },

    // ========================================================= //
    // TEST SEXPRESSION PARSE FAILS                              //
    // ========================================================= //
    {
        name: test_fail_sexp1,
        file: "test_fail_sexp1.snek",
        expected: "Invalid",
    },
    {
        name: test_fail_sexp2,
        file: "test_fail_sexp2.snek",
        expected: "Invalid",
    },
    {
        name: test_fail_sexp3,
        file: "test_fail_sexp3.snek",
        expected: "Invalid",
    },
}
