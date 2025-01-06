use std::cmp::max;

macro_rules! repeat {
    ($c:expr, $n:expr) => {{
        let mut s = String::with_capacity($n);
        for _ in 0..$n {
            s.push($c);
        }
        s
    }};
}

macro_rules! sum {
    ( $( $x:expr ),* ) => {
        {
            let mut result = 0;
            $(
                result += $x;
            )*
            result
        }
    };
}

macro_rules! max {
    ( $( $x:expr ),* ) => {
        {
            let mut result = 0;
            $(
                result = max(result, $x);
            )*
            result
        }
    };
}

pub fn exec() {
    assert_eq!(repeat!('a', 3), "aaa");
    assert_eq!(sum!(1, 2, 3), 6);
    assert_eq!(max!(1, 2, 3), 3);
}





