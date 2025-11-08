use std::io;

fn main() {

    loop {
        let mut input = String::new();
        let mut parsed_value: f64 = 0.0;

        println!("-----------------------------------------------------------------------------");
        println!("Welcome to your personal temperature converter! \nTo convert from celsius to fahrenheit enter 'CTF'\nTo convert from fahrenheit to celsius enter 'FTC'\nElse to exit enter 'exit'");
        println!("-----------------------------------------------------------------------------");

        io::stdin().read_line(&mut input).expect("Failed to readline");

        if input.trim() == "exit" {
            break;
        } else if input.trim() == "CTF" {
            let mut input = String::new();
            println!("Enter a celcius  value to convert to fahrenheit: ");
            io::stdin().read_line(&mut input).expect("Failed to readline");
            //println!("{input}");
            let input = input.trim();
            match input.parse::<f64>() {
                Ok(num) => parsed_value = num,
                Err(er) => println!("Error thrown when parsing user input to float: {}", er)
            }
            
            let parsed_value = temperature_converter_celsius_to_fahrenheit(parsed_value);
            println!("The celcius value {} converted to fahrenheit is: {}", input.trim(), parsed_value);

        } else if input.trim() == "FTC" {
            let mut input = String::new();
            println!("Enter a fahrenheit value to convert to celcius: ");
            io::stdin().read_line(&mut input).expect("Failed to readline");

            let input = input.trim();
            match input.parse::<f64>() {
                Ok(num) => parsed_value = num,
                Err(er) => println!("Error thrown when parsing user input to float:{}", er)
            }

            let parsed_value = temperature_converter_fahrenheit_to_celcius(parsed_value);
            println!("The fahrenheit value {} converted to celcius is: {}", input, parsed_value);
            
        }



    }


}

fn temperature_converter_fahrenheit_to_celcius(fahrenheit_temp: f64) -> f64 {
    (5.0 / 9.0) * (fahrenheit_temp - 32.0)
}

fn temperature_converter_celsius_to_fahrenheit(celsius_temp: f64) -> f64 {
    return celsius_temp * (9.0 / 5.0) + 32.0;
}
