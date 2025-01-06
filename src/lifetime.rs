pub fn exec() {
    test_lifetime_multiple();
    lifetime();
    lifetime1();
    lifetime2();
    lifetime3();
    lifetime4();
    lifetime5();
    lifetime6();
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

#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn lifetime() {
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number    = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}

#[derive(Debug)]
struct NoCopyType {}

#[derive(Debug)]
struct Example<'a, 'b> {
    a: &'a u32,
    b: &'b NoCopyType
}

fn lifetime1() {
    let var_a = 35;
    let example: Example;

    //   {
    let var_b = NoCopyType {};

    /* 修复错误 */
    example = Example { a: &var_a, b: &var_b };
    //   }

    println!("(Success!) {:?}", example);
}

fn lifetime2() {
    let no_copy = NoCopyType {};
    let example = Example { a: &1, b: &no_copy };
    fix_me(&example);
    println!("Success!");
    fn fix_me<'b>(foo: &Example<'_, 'b>) -> &'b NoCopyType {
        foo.b
    }
}

fn nput(x: &i32) {
    println!("`annotated_input`: {}", x);
}

fn pass(x: &i32) -> &i32 { x }

fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}

struct Owner(i32);

impl Owner {
    // Annotate lifetimes as in a standalone function.
    fn add_one(&mut self) { self.0 += 1; }
    fn print(&self) {
        println!("`print`: {}", self.0);
    }
}

struct Person<'a> {
    age: u8,
    name: &'a str,
}

#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}
static mut CONFIG: Option<&mut Config> = None;

/* 让代码工作，但不要修改函数的签名 */
fn init() -> Option<&'static mut Config> {
    let val = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });
    Some(Box::leak(val))
}

fn lifetime3() {
    unsafe {
        CONFIG = init();
        println!("{:?}", CONFIG)
    }
}

use std::fmt::Debug;

fn print_it<T: Debug + 'static>( input: T) {
    println!( "'static value passed in is: {:?}", input );
}

fn print_it1( input: impl Debug + 'static ) {
    println!( "'static value passed in is: {:?}", input );
}

fn print_it2<T: Debug + 'static>( input: &T) {
    println!( "'static value passed in is: {:?}", input );
}

fn lifetime4() {
    // i 是有所有权的数据，并没有包含任何引用，因此它是 'static
    let i = 5;
    print_it(i);
    // 但是 &i 是一个引用，生命周期受限于作用域，因此它不是 'static
    // print_it(&i);
    // print_it1(&i);
    print_it2(&i);
}

/* 添加 HRTB 使下面代码正常运行！ */
fn call_on_ref_zero<F>(f: F) where for<'a> F: Fn(&'a i32) {
    let zero = 0;
    f(&zero);
}

/* 通过重新排序一些代码使下面代码正常运行 */
fn lifetime5() {
    let mut data = 10;
    let ref1 = &mut data;
    let ref2 = &mut *ref1;
    *ref2 += 2;
    *ref1 += 1;
    println!("{}", data);
}

/* 使下面代码正常运行 */
struct Interface<'a, 'b: 'a> {
    manager: &'a mut Manager<'b>
}

impl<'a, 'b: 'a> Interface<'a, 'b> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct Manager<'b> {
    text: &'b str
}

struct List<'b> {
    manager: Manager<'b>,
}

impl<'b> List<'b> {
    pub fn get_interface<'a>(&'a mut self) -> Interface<'a, 'b> where 'b: 'a {
        Interface {
            manager: &mut self.manager
        }
    }
}

fn lifetime6() {
    let mut list = List {
        manager: Manager {
            text: "hello"
        }
    };
    list.get_interface().noop();
    println!("Interface should be dropped here and the borrow released");
    use_list(&list);
}

fn use_list(list: &List) {
    println!("{}", list.manager.text);
}