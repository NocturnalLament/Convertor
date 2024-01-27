use inquire::{Select, Text};
pub fn get_celsius()  {
    let amount = Text::new("How many degrees Celsius? ").prompt();
    match amount {
        Ok(am) => {
            // let am: f64 = am.parse().unwrap();
            let float_amount = am.parse::<f64>().unwrap();


            // Flag to control the while loop for continuous prompting
            let mut celsius_running: bool = true;
            
            // Create a new Celsius object with the inputted temperature
            let c = Celsius::new(float_amount);
            while celsius_running {
                celsius_prompt(&c);
                //Prompt user for whether or not they want to continue
                let mut continue_running = Select::new("Would you like to continue? (y/n) ", vec!["y", "n"]).prompt();
                match continue_running {
                    Ok(ans) => {
                        match ans {
                            // If yes, continue the loop
                            "y" => {
                                celsius_running = true;
                            },
                            // If no, break the loop
                            "n" => {
                                celsius_running = false;
                            },
                            &_ => todo!(),
                        }
                    },
                    // If error, print error
                    Err(_) => {
                        println!("Error")
                    }
                }
            }
        },
        Err(_) => {
            println!("Error")
        }
    }
}

//Prompt for celsius conversion
fn celsius_prompt(c: &Celsius) {
    let mut choices: Vec<String> = Vec::new();
            choices.push("Fahrenheit".to_string());
            choices.push("Kelvin".to_string());
    let mut temp_select = Select::new("Select a temperature type: ", choices).prompt();
    match temp_select {
        Ok(temp) => {
            match temp.as_str() {
                //match to and display result
                "Fahrenheit" => {
                    c.show_fahrenheit();
                },
                "Kelvin" => {
                    c.show_kelvin();
                },
                &_ => todo!(),
            }
        },
        Err(_) => {
            println!("Error")
        }
    }
}

struct Celsius{
    temp: f64,
    as_fahrenheit: f64,
    as_kelvin: f64,
}

impl Celsius {
    fn new(temp: f64) -> Self {
        Self {
            temp,
            as_fahrenheit: celsius_to_fahrenheit(&temp),
            as_kelvin: celsius_to_kelvin(&temp),
        }
    }

    fn show_fahrenheit(&self) {
        println!("{} degrees Celsius is {} degrees Fahrenheit!", self.temp, self.as_fahrenheit);
    }
    
    fn show_kelvin(&self)  {
        println!("{} degrees Celsius is {} degrees Kelvin!", self.temp, self.as_kelvin);
    }
}

fn celsius_to_fahrenheit(temp: &f64) -> f64 {
    (temp * 1.8) + 32.0
}

fn celsius_to_kelvin(temp: &f64) -> f64 {
    temp + 273.15
}