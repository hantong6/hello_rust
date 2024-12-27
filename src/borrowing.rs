pub fn exec() {
    test_lifetime();
    print_p();
    de_ref();
    mut_ref();
    mut_let();
    ref_let();
    double_ref();
    double_ref_mut_1();
}

fn test_lifetime() {
    let large = longest("a", "ab");
    println!("larger one is {}", large);
    fn longest<'a> (x: &'a str, y:&'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}

fn print_p() {
    let x = 5;
    let p = &x;
    println!("x 的内存地址是 {:p}", p); // output: 0x16fa3ac84
}

fn de_ref() {
    let x = 5;
    let y = &x;
    assert_eq!(5, *y);
}

fn mut_ref() {
    let mut s = String::from("hello, ");
    push_str(&mut s);
    fn push_str(s: &mut String) {
        s.push_str("world")
    }
}

fn mut_let() {
    let mut s = String::from("hello, ");
    let mut p = s;
    p.push_str("world");
}

fn ref_let() {
    let c = '中';
    let r1 = &c;
    let ref r2 = c;
    assert_eq!(*r1, *r2);
    assert_eq!(get_addr(r1),get_addr(r2));
    fn get_addr(r: &char) -> String {
        format!("{:p}", r)
    }
}

fn double_ref() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);
}

fn double_ref_mut() {
    let mut s = String::from("hello, ");
    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");
    // println!("{}",r1);
}

fn double_ref_mut_1() {
    let mut s = String::from("hello, ");
    let r1 = &mut s;
    println!("{}", r1);
    let r2 = &mut s;
    println!("{}", r2);
}