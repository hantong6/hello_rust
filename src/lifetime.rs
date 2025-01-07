pub fn exec() {
    test_lifetime_multiple();
}

fn test_lifetime_multiple() {
    fn insert_value<'a>(my_vec: &mut Vec<&'a i32>, value: &'a i32) {
        my_vec.push(value);
    }
    let mut my_vec: Vec<&i32> = vec![];
    let val1 = 1;
    let val2 = 2;

    let a = &mut my_vec;
    insert_value(a, &val1);
    println!("a is {:?}", a);
    let b = &mut my_vec;
    insert_value(b, &val2);
    println!("b is {:?}", b);
    println!("my_vec is {:?}", my_vec);
}