pub fn exec() {
    let (x, y) = (1, 2);
    let s = sum(x, y);
    assert_eq!(s, 3);
    print();
    match_bool();
    // never_return();
}
fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn print() -> () {
    println!("hello, function");
}


fn match_bool() {
    let b = false;
    let _v = match b {
        true => 1,
        false => {
            println!("Success!");
            0
            // panic!("we have no value for `false`, but we can panic")
        }
    };
    println!("Exercise Failed if printing out this line!");
}

fn never_return() -> ! {
    // loop {};
    panic!("never return!")
}
