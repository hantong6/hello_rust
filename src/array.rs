pub fn exec() {
    array_mul();
    mem_size();
}

fn array_mul() {
    let origin = [1, 2, 3, 4];
    let mut left = [1; 4];
    let mut right = [1; 4];
    for i in 0..origin.len() {
        if i == 0 {
            left[i] = 1;
        } else if i == 1  {
            left[i] = origin[i - 1];
        } else {
            left[i] = left[i - 1] * origin[i - 1];
        }
    }
    for i in (0..origin.len()).rev() {
        if i == origin.len() - 1 {
            right[i] = 1;
        } else if i == origin.len() - 2  {
            right[i] = origin[i + 1];
        } else {
            right[i] = right[i + 1] * origin[i + 1];
        }
    }
    let mut result = [1; 4];
    for i in 0..origin.len() {
        result[i] = left[i] * right[i];
    }
    println!("{:?}", result);
    assert_eq!(result, [24, 12, 8 ,6]);
}

fn mem_size() {
    // 很多时候，我们可以忽略数组的部分类型，也可以忽略全部类型，让编译器帮助我们推导
    let arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];
    // 数组分配在栈上， `std::mem::size_of_val` 函数会返回整个数组占用的内存空间
    // 数组中的每个 char 元素占用 4 字节的内存空间，因为在 Rust 中， char 是 Unicode 字符
    assert_eq!(std::mem::size_of_val(&arr), 12);
}


