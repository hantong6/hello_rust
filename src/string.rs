pub fn exec() {
    ref_box();
    mut_str();
    replace_str();
    add_str();
    convert_str();
    escape();
    str_index();
    str_char();
    slice();
    slice1();
    str_capacity();
    str_mem();
}

fn ref_box() {
    let s: Box<str> = "hello, world".into();
    greetings(&s);
    fn greetings(s: &str) {
        println!("{}",s)
    }
}

fn mut_str() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";
    println!("{}", s)
}

fn replace_str() {
    let s = String::from("I like dogs");
    let s1 = s.replace("dogs", "cats");
    assert_eq!(s1, "I like cats")
}

fn add_str() {
    let s1 = String::from("hello,");
    let s2 = "world!";
    let s3 = s1 + s2;
    assert_eq!(s3,"hello,world!");
    println!("{}",s3);
}

fn convert_str() {
    let s = "hello, world";
    let s = String::from(s);
    greetings(s);
    fn greetings(s: String) {
        println!("{}",s)
    }
}

fn escape() {
    let raw_str = "Escapes don't work here: \x3F \u{211D}";
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);
    let  delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);
    let long_delimiter = r###"Hello, "##""###;
    assert_eq!(long_delimiter, "Hello, \"##\"")
}

fn str_index() {
    let s1 = String::from("hi,中国");
    let h = &s1[0..1]; // 修改当前行来修复错误，提示: `h` 字符在 UTF-8 格式中只需要 1 个字节来表示
    assert_eq!(h, "h");
    let h1 = &s1[3..6];// 修改当前行来修复错误，提示: `中` 字符在 UTF-8 格式中需要 3 个字节来表示
    assert_eq!(h1, "中");
}

fn str_char() {
    for c in "你好，世界".chars() {
        println!("{}", c)
    }
}

fn slice() {
    let mut s = String::from("hello, world");
    let slice1: &str = &s; // 使用两种方法
    assert_eq!(slice1, "hello, world");
    let slice2 = &s[0..5];
    assert_eq!(slice2, "hello");
    let mut slice3: &mut String = &mut s;
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");
    println!("Success!")
}
fn slice1() {
    let s = String::from("hello, 世界");
    let slice1 = &s[0..1]; //提示: `h` 在 UTF-8 编码中只占用 1 个字节
    assert_eq!(slice1, "h");
    let slice2 = &s[7..10];// 提示: `世` 在 UTF-8 编码中占用 3 个字节
    assert_eq!(slice2, "世");
    // 迭代 s 中的所有字符
    for (i, c) in s.chars().enumerate() {
        if i == 7 {
            assert_eq!(c, '世')
        }
    }
    println!("Success!")
}

fn from_utf8() {
    let mut s = String::new();
    s.push_str("hello");
    let v = vec![104, 101, 108, 108, 111];
    let s1 = String::from_utf8(v).unwrap();
    assert_eq!(s, s1);
    println!("Success!")
}

fn str_capacity() {
    let mut s = String::with_capacity(25);
    println!("{}", s.capacity());
    for _ in 0..2 {
        s.push_str("hello");
        println!("{}", s.capacity());
    }
    println!("Success!")
}

use std::mem;

fn str_mem() {
    let story = String::from("Rust By Practice");
    // 阻止 String 的数据被自动 drop
    let mut story = mem::ManuallyDrop::new(story);
    let ptr = story.as_mut_ptr();
    let len = story.len();
    let capacity = story.capacity();
    assert_eq!(16, len);
    // 我们可以基于 ptr 指针、长度和容量来重新构建 String.
    // 这种操作必须标记为 unsafe，因为我们需要自己来确保这里的操作是安全的
    let s = unsafe { String::from_raw_parts(ptr, len, capacity) };
    assert_eq!(*story, s);
    println!("Success!")
}
