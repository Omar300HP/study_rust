use std::io;

fn main() {
    loop {
        println!("Enter your weight(kg): ");

        let weight: f32 = read_weight();

        let mars_weight: f32 = calculate_weight_on_mars(weight);

        println!(
            "Your weight on earth= {}kg, Weight on mars will be = {}kg",
            weight,
            format!("{:.2}", mars_weight)
        );

        println!("Do you want to calculate again? (y/n)");

        let should_continue = read_response();

        if !should_continue {
            break;
        }
    }
}

fn read_response() -> bool {
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();
    if answer.trim().to_lowercase() != "y" {
        return false;
    }

    true
}

fn read_weight() -> f32 {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    input
        .trim()
        .parse()
        .expect("You must enter a valid number!")
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
