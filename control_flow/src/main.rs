fn main() {
    let num = 3;

    // No parens around condition required
    if num > 0 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // No implicit conversions to boolean
    // if num {
    //     println!("The number was non-zero");
    // }

    // Obvious else if is a thing
    if num > 0 {
        println!("a");
    } else if num > 10 {
        println!("b");
    } else {
        println!("c");
    }

    // Remember that in these {} the last statement returns which is why no ;
    // but the types must still be compatible
    let condition: bool = true;
    let x = if condition { 1 } else { 6 };
    // Oh no, incompatible types, which means at compile time can't deduce type of
    // x
    // let x = if condition { 1 } else { "six" };

    // This is a loop that runs forever
    // loop {
    //     println!("anotha one");
    // }

    // use break or continue to control these types of loops
    let mut idx = 0;
    loop {
        if idx > 9 {
            break;
        }
        println!("anotha one");
        idx += 1;
    }

    // you can assign output of loops just like all expressions
    // break is kind of like end loop and return combined
    // but not return from function just in the context of the loop
    idx = 0;
    let x = loop {
        idx += 1;
        if idx > 9 {
            break idx;
        }
    };
    println!("x is: {}", x);

    // obviously while loops are a thing too
    idx = 0;
    while idx < 10 {
        idx += 1;
        println!("Counting up: {}", idx);
    }

    // range based for loops
    let a = [1,2,3,4,5];
    for x in a {
        println!("Element is {}", x);
    }

    // Easier with ranges
    // will be cool to learn more about ranges
    for x in (0..10).rev() {
        println!("T-minus {}", x);
    }
    println!("BLASTOFF");
}
