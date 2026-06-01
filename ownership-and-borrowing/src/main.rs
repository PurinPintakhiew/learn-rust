fn main() {
    let mut treasure: String = String::from("coins");

    let user1: &String = &treasure;
    let user2: &String = &treasure;

    println!("User 1 has: {}", user1);
    println!("User 2 has: {}", user2);

    let treasure_user: &mut String = &mut treasure;

    treasure_user.push_str(" and tokens");
    println!("Treasure now has: {}", treasure_user);
}
