mod function;
mod flow_control;
mod ownership;
mod borrowing;
mod array;
mod string;
mod enum1;
mod struct1;
mod pattern;
mod vec;
mod hashmap;
mod result_and_option;

fn main() {
    println!("Hello, rust!");
    function::exec();
    flow_control::exec();
    ownership::exec();
    borrowing::exec();
    array::exec();
    string::exec();
    enum1::exec();
    struct1::exec();
    pattern::exec();
    vec::exec();
    hashmap::exec();
    result_and_option::exec();
}
