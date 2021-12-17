use std::io;

fn main() {
    let x: i32 = 1;
    let y: u32 = 1;

    let tup: (i32, f64, u8) = (1000, 1.0, 1);
    // access elements of a tuple
    let z = tup.0;

    // This is equivalent to C++ void
    // () is the void type, also can be the void value
    // since the type only has one value the type and value are kind of
    // synonymous
    let void_value: () = ();

    // Array, on the stack, not a vector
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5]; // [3, 3, 3, 3, 3]
    // Indexing as usual
    let first = a[0];
    let second = a[1];

    // Some invalid runtime access of an array
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index not a number");

    let element = a[index];

    println!("The value of the element at index {} is {}", index, element);
}
