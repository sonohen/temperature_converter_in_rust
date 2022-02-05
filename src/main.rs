use std::io;

fn main() {
    println!("Please type a number to calculate : ");
    let mut temp_num = String::new();
    io::stdin()
        .read_line(&mut temp_num)
        .expect("Failed to read standard input/output.");

    let temp_num: f64 = temp_num.trim().parse()
        .expect("Please type a number.");

    let temp = convert(TemperatureType::Centigrade, temp_num);
    println!("{} in Centigrade is {} in Fahrenheit.", &temp_num, &temp);

    let temp = convert(TemperatureType::Fahrenheit, temp_num);
    println!("{} in Fahrenheit is {} in Centigrade.", &temp_num, &temp);
}

enum TemperatureType {
    Fahrenheit,
    Centigrade,
}

fn convert(temp_type: TemperatureType, temp: f64) -> f64 {
    match temp_type {
        TemperatureType::Fahrenheit => {
            (&temp - 32.0) / 1.8
        },
        TemperatureType::Centigrade => {
            &temp * 1.8 + 32.0
        }
    }
}