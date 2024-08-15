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
    


}
