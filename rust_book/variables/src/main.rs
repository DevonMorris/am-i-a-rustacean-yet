const HOURS_TO_SECONDS: u32 = 60 * 60;
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    println!("The value of HOURS_TO_SECONDS is: {}", HOURS_TO_SECONDS);

    {
        let x = 7;
        println!("The value of x in inner scope is: {}", x);
    }

    // This makes me uncomfortable tbh
    let spaces = "    ";
    let spaces = spaces.len();
    println!("Number of bytes in spaces: {}", spaces);
}
