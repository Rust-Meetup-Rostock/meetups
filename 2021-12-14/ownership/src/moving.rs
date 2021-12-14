use crate::CustomStruct;

pub fn get_value_simple() -> String {
    String::from("hello_world")
}

pub fn get_value() -> String {
    let variable_a = String::from("hello_world");

    variable_a
}

pub fn get_moved_value() -> String {
    let variable_a = String::from("hello_world");
    let variable_b = variable_a;

    variable_b
}

pub fn do_move_chaining() {
    let variable_a = String::from("hello_world");
    let variable_b = variable_a;
    let variable_c = variable_b;
    let variable_d = variable_c;
    let _variable_e = variable_d;
}

pub fn foo() {
    let foo = get_invalid_moved_value_1();

    println!("{}", foo);
}

pub fn get_invalid_moved_value_1() -> String {
    let variable_a = String::from("hello_world");
    //let _variable_b = variable_a;
    //let _variable_c = variable_a;

    variable_a
}

pub fn do_partially_move() -> CustomStruct {
    let mut result = CustomStruct::new(42, "42");
    let moved_isize = result.isize_value;
    //let moved_string = result.string_value;
    //let moved_string = result.string_value;

    //result.isize_value = 43;
    //result.string_value = "43".to_string();

    result
}

pub fn copy_example() {
    let value = 42;
    let value_str = String::from("42");

    do_something(value_str.clone());
    do_something(value_str);
    //do_something(value);
}

pub fn do_something(_value: String) {}

pub fn do_invalid_move() {
    let variable_a = String::from("hello_world");
    let variable_b = variable_a;
    //let variable_c = variable_a;
}
