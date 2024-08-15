// This is a constant:
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    // Shadowing:
    let y = 6;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
        // The value of y in the inner scope is: 14
    }

    println!("The value of y is: {y}");
    // The value of y is: 7

    // Shadowing vs mut:
    let spaces_shad = "   ";
    let mut spaces_shad = spaces_shad.len();

    let mut spaces_mut = "   ";
    //spaces_mut = spaces_mut.len(); // err because mut 


    //**************************************************************//
    //                          Data Types                          //
    //**************************************************************//

    let guess: u32 = "43".parse().expect("Not a Numper!");

    let my_hex = 0xAA;
    println!("This is my first hex definition: {my_hex}");

    let my_binary = b'A';
    let my_second_binary = 0b1111_0000;
    println!("This is my first binary definition: {my_binary}, this my second: {my_second_binary}");

    let mut my_u8: u8 = 255;
    println!("This is my_u8: {my_u8}");

    // run with --release to dont check integer overflow
    // my_u8 = my_u8 +1;
    // println!("This is my first overflow: {my_u8}");


    let c = 'z'; // four bytes long
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("{c} , {z} , {heart_eyed_cat}");
    

    // Tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The second value of tup is : {six_point_four}");

    // Array Type
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    println!("{first} {second}");
    

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5]; // array of length 5 containing value 3

    another_function(5);
    print_labeled_measurement(5, 'h');

    // statements and expressions

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");


    //**************************************************************//
    //                         Control Flow                         //
    //**************************************************************//

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("The value of counter is: {counter}");
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The final value of counter is: {counter} result is {result}");


    // loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}