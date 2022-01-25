use std::io;

fn main() {
    println!("Please input temperature.");

    let mut temperature = String::new();
    io::stdin().read_line(&mut temperature).expect("Failed to read line");
    let temperature: i32 = temperature.trim().parse().expect("Please type a number!");

    println!("Please input scale(C or F).");

    let mut scale = String::new();
    io::stdin().read_line(&mut scale).expect("Failed to read line");

    if scale.as_str().trim() == "C" {
        let temp_float = temperature as f32;
        println!("{}°C is {}°F", temperature, temp_float * 1.8 + 32.0)
    } else if scale.as_str().trim() == "F" {
        let temp_float = temperature as f32;
        println!("{}°F is {}°C", temperature, (temp_float - 32.0) / 1.8)
    } else {
        println!("No such scale!");
    }
}
