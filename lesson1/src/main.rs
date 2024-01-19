use std::io;

const C: f32 = 32.0;

//celsius to fahrenheit
fn c_to_f(c_temp: f32) -> f32 { 
    (c_temp * (9.0 / 5.0)) + C
}

//fahrenheit to celsius
fn f_to_c(f_temp: f32) -> f32 { 
    (f_temp - C) * (5.0 / 9.0)
}

fn convert(temperature: f32, choice: u8) -> Option<f32> {
    match choice {
        1 => Some(c_to_f(temperature)),
        2 => Some(f_to_c(temperature)),
        _ => None
    }
}

fn main() {
    println!("Temperature converter \n (1) C to F \n (2) F to C");

    let mut choice: String = String::new();

    io::stdin().read_line(&mut choice).unwrap();

    let number = choice.trim().parse::<u8>().expect("Please type a number!");

    println!("Enter temperature:");

    let mut temperature = String::new();

    io::stdin().read_line(&mut temperature).unwrap();

    let temperature = temperature.trim().parse::<f32>().expect("Please type a number!");

    match convert(temperature, number) {
        Some(result) => println!("Result: {result}"),
        None => println!("Unknown result")
    }
}