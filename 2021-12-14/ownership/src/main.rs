mod borrowing;
mod moving;
mod mutability;
mod shadowing;

fn main() {
    let hello_world_str = "hello_world";
    let hello_world = "hello_world".to_string();

    println!("{}", hello_world_str);
}

#[derive(Debug, Clone)]
pub struct CustomStruct {
    pub isize_value: isize,
    pub string_value: String,
}

impl CustomStruct {
    pub fn new(isize_value: isize, string_value: &str) -> Self {
        Self {
            isize_value,
            string_value: string_value.to_string(),
        }
    }
}
