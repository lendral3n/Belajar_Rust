pub fn vector() {
    // Vector
    let mut vector = Vec::new();
    vector.push(1);
    vector.push(2);
    vector.push(3);
    vector.push(4);
    println!("Vector: {:?}", vector);

    // Vector with capacity
    let mut vector_with_capacity = Vec::with_capacity(5);
    vector_with_capacity.push(1);
    vector_with_capacity.push(2);
    vector_with_capacity.push(3);
    vector_with_capacity.push(4);
    println!("Vector with capacity: {:?}", vector_with_capacity);

    // Vector with length
    let mut vector_with_length = Vec::with_capacity(5);
    vector_with_length.resize(5, 0);
    println!("Vector with length: {:?}", vector_with_length);

    // Vector with length and capacity
    let mut vector_with_length_and_capacity = Vec::with_capacity(5);
    vector_with_length_and_capacity.resize(5, 0);
    println!("Vector with length and capacity: {:?}", vector_with_length_and_capacity);
}