fn main() {
    let mut s1 = String::from("hello");

    change(&mut s1);

    let len = calculate_len(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // Cannot have more than one mutable reference to memory at any given time
    // let r1 = &mut s1;
    // let r2 = &mut s1;
    // println!("{}, {}", r1, r2);

    // Cannot have a mutable when an immutable reference exists
    // let r1 = &s1;
    // let r2 = &s1;
    // let r3 = &mut s1;
    // println!("{}, {}, {}", r1, r2, r3);

    // Impossible to have dangling references
    // let ref_to_nothing = dangle();
}

fn calculate_len(s: &String) -> usize {
    s.len()
}

// Does not work because it is not a mutable reference
// fn change(s: &String) -> () {
//     s.push_str(", world");
// }

// Does not work because it is not a mutable reference
fn change(s: &mut String) -> () {
    s.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
