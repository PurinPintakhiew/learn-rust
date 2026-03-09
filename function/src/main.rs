fn main() {
    let result: i32 = add(8, 10);
    println!("The result of adding 8 and 10 is: {}", result);

    let result_2: i32 = subtract(10, 8);
    println!("The result of subtracting 10 from 8 is: {}", result_2);
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn subtract(a: i32, b: i32) -> i32 {
    return a - b;
}