use std::io;

fn main() {
    println!("Enter your weight (kg):");
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let input: f32 = input.trim().parse().unwrap();

    let mars_weight =  calculate_weight_on_mars(input);
    println!("weight on Mars: {}kg", format!("{:.2}", mars_weight));
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
