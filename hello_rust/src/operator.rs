pub fn operator(){
    // operator aritmatika
    let (num1, num2) = (1, 2);
    println!("num1 + num2 = {}", num1 + num2);
    println!("num1 - num2 = {}", num1 - num2);
    println!("num1 * num2 = {}", num1 * num2);
    println!("num1 / num2 = {}", num1 / num2);
    println!("num1 % num2 = {}", num1 % num2);

    // operator perbandingan
    let num1 = 1;
    let num2 = 2;
    println!("num1 == num2 = {}", num1 == num2);
    println!("num1 != num2 = {}", num1 != num2);
    println!("num1 > num2 = {}", num1 > num2);
    println!("num1 < num2 = {}", num1 < num2);
    println!("num1 >= num2 = {}", num1 >= num2);
    println!("num1 <= num2 = {}", num1 <= num2);

    // operator negasi
    let (value_left, value_right) = (12, -12);
    let res_one = -value_left == value_right;
    let res_two = !(value_left == value_right);
    println!("{res_one} {res_two}");
    // output => true true

    //  operator logika / bool
    let (bool_left, bool_right) = (false, true);
    println!("AND result \t: {}", bool_left && bool_right);
    println!("OR result \t: {}", bool_left || bool_right);

    // operator bitwise
    let num1 = 1;
    let num2 = 2;
    println!("num1 & num2 = {}", num1 & num2);
    println!("num1 | num2 = {}", num1 | num2);
    println!("num1 ^ num2 = {}", num1 ^ num2);
    println!("num1 << num2 = {}", num1 << num2);
    println!("num1 >> num2 = {}", num1 >> num2);
}