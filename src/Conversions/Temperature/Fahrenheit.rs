use inquire::{Select, Text};

// Define a struct to represent a temperature in Fahrenheit
struct Fahrenheit{
    temp: f64, // The temperature in Fahrenheit
    as_celsius: f64, // The equivalent temperature in Celsius
    as_kelvin: f64, // The equivalent temperature in Kelvin
}

// Implement methods for the Fahrenheit struct
impl Fahrenheit {
    // Define a constructor for the Fahrenheit struct
    fn new(temp: f64) -> Self {
        Self {
            temp, // Set the Fahrenheit temperature
            as_celsius: fahrenheit_to_celsius(&temp), // Convert the Fahrenheit temperature to Celsius
            as_kelvin: fahrenheit_to_kelvin(temp), // Convert the Fahrenheit temperature to Kelvin
        }
    }

    // Define a method to print the Fahrenheit temperature in Kelvin
    fn show_kelvin(&self) {
        // Print the Fahrenheit temperature and its equivalent in Kelvin
        println!("{} degrees Fahrenheit is {} degrees Kelvin!", self.temp, self.as_kelvin);
    }

    // Define a method to print the Fahrenheit temperature in Celsius
    fn show_celsius(&self) {
        // Print the Fahrenheit temperature and its equivalent in Celsius
        println!("{} degrees Fahrenheit is {} degrees Celsius!", self.temp, self.as_celsius);
    }
}

fn fahrenheit_prompt(f: &Fahrenheit) {
    let mut choices: Vec<String> = Vec::new();
            choices.push("Celsius".to_string());
            choices.push("Kelvin".to_string());
    let mut temp_select = Select::new("Select a temperature type: ", choices).prompt();
    match temp_select {
        Ok(temp) => {
            match temp.as_str() {
                "Celsius" => {
                    f.show_celsius();
                },
                "Kelvin" => {
                    f.show_kelvin();
                },
                &_ => todo!(),
            }
        },
        Err(_) => {
            println!("Error")
        }
    }
}

pub fn conversion_from() {
    let mut TypesOfTemp = Vec::new();
    TypesOfTemp.push("Celsius");
    TypesOfTemp.push("Kelvin");
    //Prompt user for temperature to convert to
    let mut fahrenheit_in = Text::new("How many degrees Fahrenheit? ").prompt();
    match fahrenheit_in {
        Ok(am) => {
            match am.parse::<f64>() {
                Ok(am_as_f) => {
                    let f = Fahrenheit::new(am_as_f);
                    fahrenheit_prompt(&f);
                },
                Err(e) => {
                    println!("Error")
                }
            }
        },

        Err(e) => {
            println!("Error")
        }
    }
    // let mut temp_select = Select::new("Select a temperature type: ", TypesOfTemp).prompt();
}



fn fahrenheit_to_celsius(temp: &f64) -> f64 {
    (temp - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(temp: &f64) -> f64 {
    (temp * 9.0 / 5.0) + 32.0
}

fn celsius_to_kelvin(temp: f64) -> f64 {
    temp + 273.15
}

fn kelvin_to_celsius(temp: f64) -> f64 {
    temp - 273.15
}

fn kelvin_to_fahrenheit(temp: f64) -> f64 {
    (temp - 273.15) * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_kelvin(temp: f64) -> f64 {
    (temp - 32.0) * 5.0 / 9.0 + 273.15
}

