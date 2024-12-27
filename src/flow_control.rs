pub fn exec() {
    if_else();
    if_else_1();
    for_1();
    for_2();
    iter();
    while_1();
    break_1();
    continue_1();
    loop_1();
    loop_2();
}

fn if_else() {
    let n = 5;
    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }
}

fn if_else_1() {
    let n = 5;
    let big_n =
        if n < 10 && n > -10 {
            println!(" 数字太小，先增加 10 倍再说");
            10 * n
        } else {
            println!("数字太大，我们得让它减半");
            n / 2
        };
    println!("{} -> {}", n, big_n);
}

fn for_1() {
    for n in 1..100 {
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }
}

fn for_2() {
    let names = [String::from("liming"),String::from("hanmeimei")];
    for name in &names {}
    println!("{:?}", names);
    let numbers = [1, 2, 3];
    for n in numbers {}
    println!("{:?}", numbers);
}

fn iter() {
    let a = [4,3,2,1];
    for (i,v) in a.iter().enumerate() {
        println!("第{}个元素是{}",i+1,v);
    }
}

fn while_1() {
    let mut n = 1;
    while n <= 10 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n+=1;
    }
    println!("n 的值是 {}, 循环结束",n);
}

fn break_1() {
    let mut n = 0;
    for i in 0..=100 {
        if n == 66 {
            break;
        }
        n += 1;
    }
    assert_eq!(n, 66);
}

fn continue_1() {
    let mut n = 0;
    for i in 0..=100 {
        if n != 66 {
            n+=1;
            continue;
        }
        break;
    }
    assert_eq!(n, 66);
}


fn loop_1() {
    let mut count = 0u32;
    println!("Let's count until infinity!");
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("OK, that's enough");
            break;
        }
    }
    assert_eq!(count, 5);
}

fn loop_2() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);
}

fn loop_3() {
    let mut count = 0;
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                break 'inner1;
            }
            count += 2;
        }
        count += 5;
        'inner2: loop {
            if count >= 30 {
                break 'outer;
            }
            continue 'outer;
        }
    }
    assert!(count == 30)
}
