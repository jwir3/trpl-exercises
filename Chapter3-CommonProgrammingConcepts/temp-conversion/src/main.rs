use std::io;

fn main() {
    loop {
        println!("Please enter a temperature to convert:");
        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");
        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You must enter a number.");
                continue;
            }
        };

        println!("What units is this in?");
        let mut units = String::new();
        io::stdin()
            .read_line(&mut units)
            .expect("Failed to read line");

        let units = units.trim();
        if units == "f" || units == "F" {
            let c = convert_to_celsius(temp);
            println!("Temperature in Celsius: {}", c);
        } else if units == "c" || units == "C" {
            let f = convert_to_fahrenheit(temp);
            println!("Temperature in Fahrenheit: {}", f);
        } else {
            println!("Invalid units entered. You must enter C for Celsius or F for Fahrenheit");
            continue;
        }
    }
}

fn convert_to_celsius(f: f32) -> f32 {
    ((f - 32.) * 5.) / 9.
}

fn convert_to_fahrenheit(c: f32) -> f32 {
    ((9. / 5.) * c) + 32.
}
