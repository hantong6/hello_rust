pub fn exec() {
    check();
    mut_struct();
    reuse_struct();
    debug();
    ref_struct();
}

struct Color(i32, i32, i32);
fn check() {
    let v = Color(0, 127,255);
    check_color(v);
    fn check_color(p: Color) {
        let Color(x, y, z) = p;
        assert_eq!(x, 0);
        assert_eq!(p.1, 127);
        assert_eq!(z, 255);
    }
}

struct Person {
    name: String,
    age: u8,
}

fn mut_struct() {
    let age = 18;
    let mut p = Person {
        name: String::from("sunface"),
        age,
    };
    p.age = 30;
    p.name = String::from("sunfei");
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn reuse_struct() {
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };
    let u2 = set_email(u1);
    fn set_email(u: User) -> User {
        User {
            email: String::from("contact@im.dev"),
            ..u
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn debug() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // 打印 debug 信息到标准错误输出 stderr,并将 `30 * scale` 的值赋给 `width`
        height: 50,
    };

    dbg!(&rect1); // 打印 debug 信息到标准错误输出 stderr
    println!("{:?}", rect1); // 打印 debug 信息到标准输出 stdout
}

#[derive(Debug)]
struct File {
    name: String,
    data: String,
}

fn ref_struct() {
    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string()
    };
    let ref name = f.name;
    println!("{}, {}, {:?}",f.name, f.data, f);
}