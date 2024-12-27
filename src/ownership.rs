pub fn exec() {
    print();
    print_1();
    copy_print();
    move_mut();
    box_mut();
    move_ref();
    move_ref_1();
}

fn print() {
    fn take_ownership(s: String) -> String {
        s
    }
    let s1 = String::from("Hello");
    let s2 = take_ownership(s1.clone());

    println!("{}", s1);
    println!("{}", s2);
}

fn print_1() {
    let s = give_ownership();
    println!("{}", s);
    fn give_ownership() -> String {
        let s = String::from("hello, world");
        let _s = s.into_bytes();
        String::from_utf8(_s).expect("convert error")
    }
}

fn copy_print() {
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}

fn move_mut() {
    let s = String::from("hello, ");
    let mut s1 = s;
    s1.push_str("world")
}

fn box_mut() {
    let x = Box::new(5);
    let mut y = Box::new(3);
    *y = 4;
    assert_eq!(*x, 5);
}

fn move_ref() {
    let t = (String::from("hello"), String::from("world"));
    let _s = t.0;
    println!("{:?}", t.1);
}

fn move_ref_1() {
    let t = (String::from("hello"), String::from("world"));
    let (ref s1, ref s2) = t;
    println!("{:?}, {:?}, {:?}", s1, s2, t);
}
