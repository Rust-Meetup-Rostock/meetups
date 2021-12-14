pub fn main() {
    let hello_world = "hello_world";

    borrow_value(hello_world);
    multi_borrow_value(hello_world);
    return_argument_borrow(hello_world);

    let result = return_static_borrow();
    let result = return_borrow_with_lifetime();
}

pub fn borrow_value(value: &str) {
    println!("{}", value);
}

pub fn multi_borrow_value(value: &str) {
    let var_a = value;
    let var_b = value;
    let var_c = value;

    println!("{}, {}, {}", var_a, var_b, var_c);
}

pub fn return_argument_borrow(value: &str) -> &str {
    let var_a = value;
    let var_b = var_a;
    let var_c = var_b;

    var_c
}

pub fn return_static_borrow() -> &'static str {
    let hello_world = "hello_world";

    hello_world
}

pub fn return_borrow_with_lifetime<'named>() -> &'named str {
    let hello_world = "hello_world";

    hello_world
}
