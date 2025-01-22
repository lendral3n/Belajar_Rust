use std::io;

pub struct MyStruct {
    number: i32,
    float: f32,
    boolean: bool,
    character: char,
    string: &'static str,
    array: [i32; 4],
    tuple: (i32, i32, i32, i32),
}

pub fn tipedata() {
    // Integer
    let number = 1;
    let number2: i32 = 2;
    println!("Integer: {}, {}", number, number2);

    // Float
    let float = 1.0;
    let float2: f32 = 2.0;
    println!("Float: {}, {}", float, float2);

    // Boolean
    let boolean = true;
    let boolean2: bool = false;
    println!("Boolean: {}, {}", boolean, boolean2);

    // Character
    let character = 'A';
    let character2: char = 'B';
    println!("Character: {}, {}", character, character2);

    // String
    let string = "Hello";
    let string2: &str = "World";
    println!("String: {} {}", string, string2);

    // Array
    let array = [1, 2, 3, 4];
    let array2: [i32; 4] = [5, 6, 7, 8];
    println!("Array 1: {:?}, Array 2: {:?}", array, array2);

    // Tuple
    let tuple = (1, 2, 3, 4);
    let tuple2: (i32, i32, i32, i32) = (5, 6, 7, 8);
    println!("Tuple 1: {:?}, Tuple 2: {:?}", tuple, tuple2);

    // Struct
    let my_struct = MyStruct {
        number: 1,
        float: 2.0,
        boolean: true,
        character: 'A',
        string: "Hello",
        array: [1, 2, 3, 4],
        tuple: (1, 2, 3, 4),
    };
    println!(
        "Struct: {{ number: {}, float: {}, boolean: {}, character: {}, string: {}, array: {:?}, tuple: {:?} }}",
        my_struct.number, my_struct.float, my_struct.boolean, my_struct.character, my_struct.string, my_struct.array, my_struct.tuple
    );

    // Option
    let some_option = Some(1);
    let none_option: Option<i32> = None;
    println!("Option Some: {:?}, Option None: {:?}", some_option, none_option);

    // Result
    let ok_result: Result<i32, &str> = Ok(1);
    let err_result: Result<i32, &str> = Err("Error occurred");
    println!("Result Ok: {:?}, Result Err: {:?}", ok_result, err_result);

    // Collection
    let collection = vec![1, 2, 3, 4];
    println!("Collection: {:?}", collection);

    // Error Handling
    let io_error = io::Error::new(io::ErrorKind::Other, "Custom Error");
    println!("IO Error: {:?}", io_error);
}