use std::ops::{Add, Mul, Sub};

pub fn exec() {
    test_multiply();
    test_add();
    test_dyn_trait();
    test_dyn_return();
}

// 实现 fn multiply 方法
fn test_multiply() {
    assert_eq!(6, multiply(2u8, 3u8));
    assert_eq!(5.0, multiply(1.0, 5.0));

    println!("Success!");

    fn multiply<T: Mul<Output=T>>(a: T, b: T) -> T {
        a * b
    }
}

fn test_add() {
    // 你需要为 FooBar 派生一些特征来让代码工作
    assert_eq!(Foo + Bar, FooBar);
    assert_eq!(Foo - Bar, BarFoo);
    println!("Success!");

    struct Foo;
    struct Bar;

    #[derive(Debug, PartialEq)]
    struct FooBar;

    #[derive(Debug, PartialEq)]
    struct BarFoo;

    impl Add<Bar> for Foo {
        type Output = FooBar;

        fn add(self, _rhs: Bar) -> FooBar {
            FooBar
        }
    }

    impl Sub<Bar> for Foo {
        type Output = BarFoo;

        fn sub(self, _rhs: Bar) -> BarFoo {
            BarFoo
        }
    }
}

// 实现 `fn summary`
// 修复错误且不要移除任何代码行
fn test_dyn_trait() {
    let post = Post {
        title: "Popular Rust".to_string(),
        author: "Sunface".to_string(),
        content: "Rust is awesome!".to_string(),
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "Weibo seems to be worse than Tweet".to_string(),
    };

    summary(&post);
    summary(&weibo);

    println!("{:?}", post);
    println!("{:?}", weibo);

    fn summary(target: &dyn Summary) {
        target.summarize();
    }

    trait Summary {
        fn summarize(&self) -> String;
    }

    #[derive(Debug)]
    struct Post {
        title: String,
        author: String,
        content: String,
    }

    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("The author of post {} is {}", self.title, self.author)
        }
    }

    #[derive(Debug)]
    struct Weibo {
        username: String,
        content: String,
    }

    impl Summary for Weibo {
        fn summarize(&self) -> String {
            format!("{} published a weibo {}", self.username, self.content)
        }
    }

}

fn test_dyn_return() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());

    struct Sheep {}
    struct Cow {}

    trait Animal {
        fn noise(&self) -> String;
    }

    impl Animal for Sheep {
        fn noise(&self) -> String {
            "baaaaah!".to_string()
        }
    }

    impl Animal for Cow {
        fn noise(&self) -> String {
            "moooooo!".to_string()
        }
    }

    // 返回一个类型，该类型实现了 Animal 特征，但是我们并不能在编译期获知具体返回了哪个类型
    // 修复这里的错误，你可以使用虚假的随机，也可以使用特征对象
    fn random_animal(random_number: f64) -> Box<dyn Animal> {
        if random_number < 0.5 {
            Box::new(Sheep {})
        } else {
            Box::new(Cow {})
        }
    }
}

