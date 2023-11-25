fn main() {
    println!("Hello, world!");

    let random_sum = add_numbers(1, 2);
    println!("{random_sum}");
}

pub fn add_numbers(num1: i32, num2: i32) -> i32 {
    return num1 + num2;
}
