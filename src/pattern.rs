use std::env::join_paths;
use serde_json::Value;

pub fn exec() {
    match1();
    set_value();
    match2();
    match3();
    match4();
    match5();
    match6();
    match7();
    json_parse();
    match_number(3);
    match_struct();
    match_struct_1();
    match_guard();
    ignore();
    match_ref();
}

enum Direction {
    East,
    West,
    North,
    South,
}

fn match1() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North  => { // 在这里匹配 South 或 North
            println!("South or North");
        },
        _ => println!("West"),
    };
}

fn set_value() {
    let boolean = true;
    let binary = match boolean {
        true => 1,
        false => 0
    };
    assert_eq!(binary, 1);
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn match2() {
    let msgs = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];
    for msg in msgs {
        show_message(msg)
    }
    fn show_message(msg: Message) {
        match msg {
            Message::Move{x: a, y: b} => { // 这里匹配 Message::Move
                assert_eq!(a, 1);
                assert_eq!(b, 3);
            },
            Message::ChangeColor(_, g, b) => {
                assert_eq!(g, 255);
                assert_eq!(b, 00);
            }
            __ => println!("no data in these variants")
        }
    }
}

fn match3() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];
    for ab in alphabets {
        assert!(matches!(ab, 'a'..='z' | 'A'..='Z' | '0'..='9'))
    }
}

enum MyEnum {
    Foo,
    Bar
}

fn match4() {
    let mut count = 0;
    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    for e in v {
        if let MyEnum::Foo = e { // 修复错误，只能修改本行代码
            count += 1;
        }
    }
    assert_eq!(count, 2);
}

enum Foo {
    Bar(u8)
}

fn match5() {
    let a = Foo::Bar(1);
    if let Foo::Bar(i) = a {
        println!("foobar 持有的值是: {}", i);
    }
}

enum Foo1 {
    Bar,
    Baz,
    Qux(u32)
}

fn match6() {
    let a = Foo1::Qux(10);
    match a {
        Foo1::Bar => println!("match foo::bar"),
        Foo1::Baz => println!("match foo::baz"),
        _ => println!("match others")
    }
}

fn match7() {
    let age = Some(30);
    if let Some(age) = age { // 创建一个新的变量，该变量与之前的 `age` 变量同名
        assert_eq!(age, 30);
    } // 新的 `age` 变量在这里超出作用域
    match age {
        // `match` 也能实现变量遮蔽
        Some(age) =>  println!("age 是一个新的变量，它的值是 {}",age),
        _ => ()
    }
}

fn json_parse() {
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let v: Value = serde_json::from_str(data).unwrap();
    dbg!(v);
}

fn match_number(n: i32) {
    match n {
        // 匹配一个单独的值
        1 => println!("One!"),
        // 使用 `|` 填空，不要使用 `..` 或 `..=`
        2 | 3 | 4 | 5 => println!("match 2 -> 5"),
        // 匹配一个闭区间的数值序列
        6..=10 => {
            println!("match 6 -> 10")
        },
        _ => {
            println!("match 11 -> +infinite")
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn match_struct() {
    // 填空，让 p 匹配第二个分支
    let p = Point { x: 0, y: 10 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // 第二个分支
        Point { x: 0..=5, y: a@(10 | 20 | 30) } => println!("On the y axis at {}", a),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}


// 修复错误
enum Message1 {
    Hello { id: i32 },
}

fn match_struct_1() {
    let msg = Message1::Hello { id: 5 };
    match msg {
        Message1::Hello {
            id: id@3..=7,
        } => println!("id 值的范围在 [3, 7] 之间: {}", id),
        Message1::Hello { id: newid@(10 | 11 | 12) } => {
            println!("id 值的范围在 [10, 12] 之间: {}", newid)
        }
        Message1::Hello { id } => println!("Found some other id: {}", id),
    }
}

fn match_guard() {
    let num = Some(4);
    let split = 5;
    match num {
        Some(x) if x < split => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }
}

fn ignore() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers {
        (first, .. , last) => {
            assert_eq!(first, 2);
            assert_eq!(last, 2048);
        }
    }
}

fn match_ref() {
    let mut v = String::from("hello,");
    let r = &mut v;
    match r {
        value => value.push_str(" world!")
    }
}

