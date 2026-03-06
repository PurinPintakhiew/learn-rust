fn main() {
    // --- if-else
    let weather = "sunny"; // sunny, rainy, stormy

    if weather == "sunny" {
        println!("wash clothes.");
    } else if weather == "rainy" {
        println!("store the clothes.");
    } else if weather == "stormy" {
        println!("close the house.");
    } else {
        println!("sleep.")
    }

    // --- match
    let meal = "breakfast"; // breakfast, lunch, dinner

    match meal {
        "breakfast" => println!("bread"),
        "lunch" => println!("hamburger"),
        "dinner" => println!("salad"),
        _ => println!("cocoa")    
    }

    // --- loops
    let mut cats = 0;

    loop {
        cats += 1;
        println!("I adopted a cat. Total: {}", cats);

        if cats == 3 {
            break;
        }
    }

}