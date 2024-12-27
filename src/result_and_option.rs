use std::fmt::{write, Display, Formatter};

pub fn exec() {
    println!("{:#?}", call(8, -2));
}

fn call(a: i32, b: i32) -> Result<f64, String> {
    let r = divide(a, b);
    let s = sqrt(r.unwrap());
    match s {
        Ok(f) => Ok(f),
        Err(e) => Err(e.to_string())
    }
}

fn divide(a: i32, b: i32) -> Option<f64> {
    if b != 0 {
        Some(a as f64 / b as f64)
    } else {
        None
    }
}

pub enum MathError {
    DivisionByZero,
    NegativeSquareRoot
}

impl Display for MathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result<> {
        match self {
            MathError::DivisionByZero => write!(f, "DivisionByZero"),
            MathError::NegativeSquareRoot => write!(f, "NegativeSquareRoot"),
        }
    }
}

fn sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}