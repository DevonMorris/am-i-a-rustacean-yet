fn main() {
    {
        let s : &str = "hello";
    }

    let mut s : String = String::from("hello");

    s.push_str(", world");

    println!("{}", s);

    {
        let x: i32 = 3;
        let y: i32 = x;
    }

    {
        let s1: String = String::from("hello");
        let s2 = s1;
        // Does not work because s1 is moved into s2 and s1 is invalidated
        // println!("{}", s1);
    }

    {
        let s1: String = String::from("hello");
        let s2 = s1.clone();
        println!("s1 = {}\ns2 = {}", s1, s2);
    }

    {
        let x: i32 = 3;
        let y: i32 = x;
        // This works because x & y implement the copy trait
        // Copy is disabled if the drop trait is implemented
        println!("y = {}\ny = {}", x, y);
    }

    {
        let s: String = String::from("hello");
        takes_ownership(s);

        // Cannot use s here since it has been "dropped"

        let x: i32 = 5;
        makes_copy(x);
    }

    {
        let s1 = gives_ownership();
        let s2 = String::from("hello");
        let s3 = takes_and_gives_back(s2);
        // s2 is invalidated here
        // println!("{}", s2);
    }
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // don't forget that this returns
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // return the same string
}
