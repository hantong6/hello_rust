pub fn exec() {
    vec1();
    extend();
    from_into();
    mut_vec();
    vec_slice();
    vec_capacity();
    vec_enum();
    trait_object();
    test_stack();
}

fn vec1() {
    let arr: [u8; 3] = [1, 2, 3];
    let v = Vec::from(arr);
    is_vec(&v);
    let v = vec![1, 2, 3];
    is_vec(&v);
    // vec!(..) and vec![..] are same macros, so
    let v = vec!(1, 2, 3);
    is_vec(&v);
    // in code below, v is Vec<[u8; 3]> , not Vec<u8>
    // USE `for` to rewrite the below code
    let mut v1 = Vec::new();
    for i in &v {
        v1.push(*i)
    }
    is_vec(&v1);
    assert_eq!(format!("{:?}",v), format!("{:?}",v1));
    println!("Success!");
    fn is_vec(v: &Vec<u8>) {}
}

fn extend() {
    let mut v1 = Vec::from([1, 2, 4]);
    v1.pop();
    v1.push(3);
    let mut v2 = Vec::new();
    v2.extend([1,2,3]);
    assert_eq!(v1, v2);
    println!("Success!")
}

fn from_into() {
    // array -> Vec
    // impl From<[T; N]> for Vec
    let arr = [1, 2, 3];
    let v1 = Vec::from(arr);
    let v2: Vec<i32> = arr.into();
    assert_eq!(v1, v2);
    // String -> Vec
    // impl From<String> for Vec
    let s = "hello".to_string();
    let v1: Vec<u8> = s.into();
    let s = "hello".to_string();
    let v2 = s.into_bytes();
    assert_eq!(v1, v2);
    // impl<'_> From<&'_ str> for Vec
    let s = "hello";
    let v3 = Vec::from(s);
    assert_eq!(v2, v3);
    // 迭代器 Iterators 可以通过 collect 变成 Vec
    let v4: Vec<i32> = [0; 10].into_iter().collect();
    assert_eq!(v4, vec![0; 10]);
    println!("Success!")
}

fn mut_vec() {
    let mut v = Vec::from([1, 2, 3]);
    for i in 0..3 {
        println!("{:?}", v[i])
    }
    for i in 0..5 {
        if let Some(x) = v.get(i) {
            v[i] = x + 1;
        } else {
            v.push(i + 2);
        }
    }
    assert_eq!(v, vec![2, 3, 4, 5, 6]);
    println!("Success!")
}

fn vec_slice() {
    let mut v = vec![1, 2, 3];
    let slice1 = &v[..];
    // 越界访问将导致 panic.
    // 修改时必须使用 `v.len`
    let slice2 = &v[0..3];
    assert_eq!(slice1, slice2);
    // 切片是只读的
    // 注意：切片和 `&Vec` 是不同的类型，后者仅仅是 `Vec` 的引用，并可以通过解引用直接获取 `Vec`
    let vec_ref: &mut Vec<i32> = &mut v;
    (*vec_ref).push(4);
    let slice3 = &mut v[0..4];
    assert_eq!(slice3, &[1, 2, 3, 4]);
    println!("Success!")
}

fn vec_capacity() {
    let mut vec = Vec::with_capacity(10);
    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 10);
    for i in 0..10 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10);
    assert_eq!(vec.capacity(), 10);
    vec.push(11);
    assert_eq!(vec.len(), 11);
    assert!(vec.capacity() >= 11);
    let mut vec = Vec::with_capacity(100);
    for i in 0..100 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 100);
    assert_eq!(vec.capacity(), 100);
    println!("Success!")
}

#[derive(Debug, PartialEq)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn vec_enum() {
    // 填空
    let v : Vec<IpAddr>= vec![IpAddr::V4("127.0.0.1".to_string()), IpAddr::V6("::1".to_string())];
    // 枚举的比较需要派生 PartialEq 特征
    assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
    assert_eq!(v[1], IpAddr::V6("::1".to_string()));
    println!("Success!")
}

trait IpAddr1 {
    fn display(&self);
}
struct V4(String);
impl IpAddr1 for V4 {
    fn display(&self) {
        println!("ipv4: {:?}",self.0)
    }
}
struct V6(String);
impl IpAddr1 for V6 {
    fn display(&self) {
        println!("ipv6: {:?}",self.0)
    }
}
fn trait_object() {
    // 填空
    let v: Vec<Box<dyn IpAddr1>>= vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];
    for ip in v {
        ip.display();
    }
}

struct Stack<T> {
    data: Vec<T>
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack{
            data: Vec::new()
        }
    }

    fn push(&mut self, e: T) {
        self.data.push(e);
    }

    fn pop(&mut self) -> T {
        self.data.pop().unwrap()
    }

    fn peek(&self) -> &T {
        &self.data[self.data.len() - 1]
    }

    fn len(&self) -> usize {
        self.data.len()
    }

}

fn test_stack() {
    let mut stack = Stack::new();
    stack.push(1);
    println!("{}", stack.peek());
    println!("{}", stack.pop());
    assert_eq!(stack.len(), 0)
}



