use std::io;

fn main() {
    println!("Please type a number to calculate : ");
    let mut temp_input = String::new();
    io::stdin()
        .read_line(&mut temp_input)
        .expect("Failed to read standard input/output.");

    let temp_input: f64 = temp_input.trim().parse()
        .expect("Please type a number.");

    let temp_calculated = convert(TemperatureType::Centigrade, &temp_input);
    println!("{} in Centigrade is {} in Fahrenheit.", &temp_input, &temp_calculated);

    let temp_calculated = convert(TemperatureType::Fahrenheit, &temp_input);
    println!("{} in Fahrenheit is {} in Centigrade.", &temp_input, &temp_calculated);
}

enum TemperatureType {
    Fahrenheit,
    Centigrade,
}

fn convert(temp_type: TemperatureType, t: &f64) -> f64 {
    match temp_type {
        TemperatureType::Fahrenheit => {
            (*t - 32.0) / 1.8
        },
        TemperatureType::Centigrade => {
            *t * 1.8 + 32.0
        }
    }
}