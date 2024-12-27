use std::mem;
use crate::enum1::List::{Cons, Nil};

pub fn exec() {
    size();
    convert_as();
    init();
    destruct();
    enum_array();
    option_match();
    list_define();
}

enum EnumA {
    A(u8, u8),
    B,
    C{}
}

enum EnumB {
    A = 255
}

enum EnumC {
    A = 255,
    B
}

fn size() {
    assert_eq!(size_of::<EnumA>(), 3);
    assert_eq!(size_of::<EnumB>(), 0);
    assert_eq!(size_of::<EnumC>(), 2);
}

enum Number {
    Zero,
    One,
    Two,
}

enum Number1 {
    Zero = 0,
    One,
    Two,
}

enum Number2 {
    Zero = 0,
    One = 1,
    Two = 2,
}

fn convert_as() {
    assert_eq!(Number::One as u8, Number1::One as u8);
    assert_eq!(Number1::One as u8, Number2::One as u8);
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn init() {
    let msg1 = Message::Move{x: 1, y: 2_}; // 使用x = 1, y = 2 来初始化
    let msg2 = Message::Write("hello, world!".to_string()); // 使用 "hello, world!" 来初始化
}

fn destruct() {
    let msg = Message::Move{x: 1, y: 2};

    if let Message::Move{x: a, y: b} = msg {
        assert_ne!(a, b);
    } else {
        panic!("不要让这行代码运行！");
    }
}

fn enum_array() {
    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];
    for msg in msgs {
        show_message(msg)
    }
    fn show_message(msg: Message) {
        println!("{:?}", msg);
    }
}

fn option_match() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    if let Some(n) = six {
        println!("{}", n);
        return;
    }
    panic!("不要让这行代码运行！");
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
}

enum List {
    // Cons: 链表中包含有值的节点，节点是元组类型，第一个元素是节点的值，第二个元素是指向下一个节点的指针
    Cons(u32, Box<List>),
    // Nil: 链表中的最后一个节点，用于说明链表的结束
    Nil,
}

// 为枚举实现一些方法
impl List {
    // 创建空的链表
    fn new() -> List {
        // 因为没有节点，所以直接返回 Nil 节点
        // 枚举成员 Nil 的类型是 List
        Nil
    }

    // 在老的链表前面新增一个节点，并返回新的链表
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    // 返回链表的长度
    fn len(&self) -> u32 {
        match *self {
            // 这里我们不能拿走 tail 的所有权，因此需要获取它的引用
            Cons(_, ref tail) => 1 + tail.len(),
            // 空链表的长度为 0
            Nil => 0
        }
    }

    // 返回链表的字符串表现形式，用于打印输出
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // 递归生成字符串
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                "Nil".to_string()
            },
        }
    }
}

fn list_define() {
    // 创建一个新的链表(也是空的)
    let mut list = List::new();
    // 添加一些元素
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    // 打印列表的当前状态
    println!("链表的长度是: {}", list.len());
    println!("{}", list.stringify());
}


