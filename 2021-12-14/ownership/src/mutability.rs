use crate::CustomStruct;

pub fn immutable() -> String {
    let hello_world = String::from("hello_");
    //hello_world.push_str("world");

    hello_world
}

pub fn mutable() -> String {
    let mut hello_world = String::from("hello");
    hello_world.push('_');
    hello_world.push_str("world");
    hello_world.push_str("!");

    hello_world
}

pub fn mutable_arguments() -> String {
    let mut foo = String::from("foo");
    //mutate(foo);
    //mutate(foo);
    //mutate(foo);

    foo
}

pub fn mutate(mut value: String) -> String {
    value.push('_');
    value.push_str("foo");
    value.push_str("_bar");

    value
}

pub fn mutable_borrowed_arguments() -> CustomStruct {
    let mut foo = CustomStruct::new(42, "42");

    do_nothing_with_borrowed(&foo);
    do_nothing_with_borrowed(&foo);
    do_nothing_with_borrowed(&foo);
    mutate_borrowed(&mut foo);
    mutate_borrowed(&mut foo);
    mutate_borrowed(&mut foo);

    foo
}

pub fn do_nothing_with_borrowed(value: &CustomStruct) {
    println!("{:#?}", value);
}

pub fn mutate_borrowed(value: &mut CustomStruct) {
    value.isize_value = 43;
    value.string_value = "45".to_string();
}

pub fn multi_mutate_borrowed(value: &mut CustomStruct) {
    let mut foo = String::from("hello_world");

    let mutable_foo_borrow = &mut foo;
    //let reference_b = &mut foo;

    //println!("{}, {}", mutable_foo_borrow, reference_b);
    println!("{}", mutable_foo_borrow);
}
