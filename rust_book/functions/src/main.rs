fn main() {
    println!("Hello, world!");

    another_function();
    printi32(20);
    print_labeled_measurement(5, 'h');

    // Statements don't return values
    // let x = (let y = 6);

    // This is an expression
    // Note: it is important that the line x + 1 does not have a ;
    // this means it will "return" the result of x + 1
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    let x = 5;
    printi32(x);
    printi32(anotha_one(x));
    printi32(anotha_one(five()));
}

// We can be really explicit that this doesn't return out of here
fn another_function() -> () {
    println!("Another Function.");
}

fn printi32(x: i32) -> () {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) -> () {
    println!("The measurement is: {}{}", value, unit_label);
}

// You can also say "return 5" but it's not idiomatic
fn five() -> i32 {
    5
}

// Note again, no semicolon bc we need to return
fn anotha_one(x: i32) -> i32 {
    x + 1
}
