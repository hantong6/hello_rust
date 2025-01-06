pub fn exec() {
    iter_mut();
    test_fibonacci();

}

fn iter_mut() {
    let mut values = vec![1, 2, 3];
    let mut values_iter = values.iter_mut();
    if let Some(v) = values_iter.next(){
        *v = 0;
    }
    assert_eq!(values, vec![0, 2, 3]);
}


fn test_fibonacci() {

    let mut fib = fibonacci();
    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(2));
    assert_eq!(fib.next(), Some(3));
    assert_eq!(fib.next(), Some(5));

    struct Fibonacci {
        curr: u32,
        next: u32,
    }

    // Implement `Iterator` for `Fibonacci`.
    // The `Iterator` trait only requires a method to be defined for the `next` element.
    impl Iterator for Fibonacci {
        // We can refer to this type using Self::Item
        type Item = u32;

        /* Implement next method */
        fn next(&mut self) -> Option<Self::Item>{
            let res = self.next;
            self.next = self.next + self.curr;
            self.curr = res;
            Some(res)
        }
    }

    // Returns a Fibonacci sequence generator
    fn fibonacci() -> Fibonacci {
        Fibonacci { curr: 0, next: 1 }
    }
}

fn test_shoe_in_size() {
    /* Fill in the blanks */
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|x| x.size <= shoe_size).collect()
    }
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}