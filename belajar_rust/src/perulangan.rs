pub fn perulangan() {
    // For loop: 4 iterasi
    for i in 0..4 {
        println!("for loop - i = {}", i);
    }

    // While loop: 3 iterasi
    let mut j = 0;
    while j < 3 {
        println!("while loop - j = {}", j);
        j += 1;
    }

    // Infinite loop (loop): 3 iterasi
    let mut k = 0;
    loop {
        println!("loop - k = {}", k);
        k += 1;
        if k == 3 {
            break;
        }
    }
}