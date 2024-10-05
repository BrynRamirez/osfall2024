const FARENHEIT:f32 = 32.0;

fn fahrenheit_to_celsius(f:f64) -> f64
{
    let far = FARENHEIT as f64;
    let cels = (f - far) * 5.0/9.0;
    return cels
}

fn celsius_to_fahrenheit(c:f64) -> f64
{
    let far = FARENHEIT as f64;
    let cels = (c * 9.0/5.0) + far;
    return cels
}

pub fn main() {
    let mut temp_fahr = 50;
    let mut temp_celsius = 10;
    println!("{}F to C: {}", temp_fahr, fahrenheit_to_celsius(temp_fahr as f64));
    println!("{}C to F: {}\n", temp_celsius, celsius_to_fahrenheit(temp_celsius as f64));
    
    for _x in 0..5 {
        temp_fahr +=1;
        temp_celsius +=1;

        println!("{}F to C: {}", temp_fahr, fahrenheit_to_celsius(temp_fahr as f64));
        println!("{}C to F: {}\n", temp_celsius, celsius_to_fahrenheit(temp_celsius as f64));
    }
}
