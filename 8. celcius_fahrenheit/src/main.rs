
pub mod temperature;
use temperature::{Scale, Temperature};

fn test01() {
    let temperature_value: f32 = 70.;    
    let temperature_type = Scale::Fahrenheit;

    let mut my_temperature = Temperature::new(temperature_value, temperature_type);

    println!("degrees = {}", my_temperature.degrees());
    println!("type = {}", my_temperature.scale());

    my_temperature.to_celsius();
    
    println!();
    println!("Fahrenheit to Celsius");
    println!("degrees = {}", my_temperature.degrees());
    println!("type = {}", my_temperature.scale());

    my_temperature.to_fahrenheit();

    println!();
    println!("Celsius to Fahrenheit");
    println!("degree = {}", my_temperature.degrees());
    println!("type = {}", my_temperature.scale());
}

fn main() {
    test01();
}

