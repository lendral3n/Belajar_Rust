pub fn functions() {
    // Function
    fn function() {
        println!("Function");
    }

    // Function with parameters
    fn function_with_parameters(param1: i32, param2: i32) {
        println!("Function with parameters - param1: {}, param2: {}", param1, param2);
    }

    // Function with return type
    fn function_with_return_type() -> i32 {
        println!("Function with return type");
        return 1;
    }

    // Function with parameters and return type
    fn function_with_parameters_and_return_type(param1: i32, param2: i32) -> i32 {
        println!("Function with parameters and return type - param1: {}, param2: {}", param1, param2);
        return 1;
    }

    // Function with parameters and return type
    fn function_with_parameters_and_return_type2(param1: i32, param2: i32) -> i32 {
        println!("Function with parameters and return type - param1: {}, param2: {}", param1, param2);
        return 1;
    }

    function();
    function_with_parameters(1, 2);
    let result = function_with_return_type();
    println!("Result: {}", result);
    let result2 = function_with_parameters_and_return_type(1, 2);
    println!("Result2: {}", result2);
    let result3 = function_with_parameters_and_return_type2(1, 2);
    println!("Result3: {}", result3);
}