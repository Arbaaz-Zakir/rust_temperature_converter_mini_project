use std::io;

fn main() {

    loop {
        println!("\nMENU \nplease enter a 1 - to convert from fahrenheit to celcius \n\n Enter 2 - to convert from celsius to fahrenheit \n\nType \"quit\" to quit the entire program");
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to readline");

        let input = input.trim();

        
        if input.trim() == "quit" { break }

        let mut parsed_str: f64 = 0.0;

        'far_to_cel_loop: loop {

            println!("\nplease enter a temperature to convert from fahrenheit to celcius OR type \"exit\" to leave to the menu");

            if input.trim() == "exit" { break 'far_to_cel_loop };

            match input.parse::<f64>() {
                Ok(num) => parsed_str = num,
                Err(er) => println!("Got an error: {}", er),
            }

            let temp = temperature_converter_fahrenheit_to_celsius(parsed_str);
            println!("Conversion: {input} fahrenheit to celcius is {temp:.2}!");

        }

        'cel_to_far_loop: loop{

            println!("\nplease enter a temperature to convert from celcius to fahrenheit OR type \"exit\" to leave to the menu");

            if input.trim() == "exit" { break 'cel_to_far_loop };
            
            match input.parse::<f64>() {
                Ok(num) => parsed_str = num,
                Err(er) => println!("Got an error: {}", er),
            }

        }

    }

}

fn temperature_converter_fahrenheit_to_celsius(fahrenheit_temp: f64) -> f64 {
    return (5.0 / 9.0) * (fahrenheit_temp -32.0);
}

fn temperature_converter_celsius_to_fahrenheit(celsius_temp: f64) -> f64 {
    return celsius_temp * (9 / 5) + 32;
}
