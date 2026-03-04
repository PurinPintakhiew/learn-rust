fn main() {
    let x = 10;
    let y = 7.5;
    
    let sum = x + y as i32;
    
    let str = String::from("Hallo, world");
    let str2 = "Hallo, world".to_string();
    let str3 = "Hallo, world";
    let str4 = format!("str: {}, str2: {}, str3: {}", str, str2, str3);
    
    println!("Sum : {}", sum);
    println!("{}", str4);
}
