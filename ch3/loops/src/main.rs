fn main() {
    // LOOP EXAMPLE
    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is: {}", result);

    // WHILE LOOP EXAMPLE
    // let mut number = 3;

    // while number != 0 {
    //     println!("{}", number);
    //     number -= 1;
    // }

    // println!("LIFTOFF!");

    // FOR LOOP EXAMPLE
    // let a = [10, 20, 30, 40, 50];

    // for element in a.iter() {
    //     println!("The value of current element is: {}", element);
    // }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
