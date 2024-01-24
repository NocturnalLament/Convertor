use inquire::{Select, Text};
pub fn get_celsius()  {
    let amount = Text::new("How many degrees Celsius? ").prompt();
    match amount {
        Ok(am) => {
            // let am: f64 = am.parse().unwrap();
            let float_amount = am.parse::<f64>().unwrap();
            // let result = celsius_to_fahrenheit(&am);
            // println!("{} degrees Celsius is {} degrees Fahrenheit", am, result);
            let mut choices: Vec<String> = Vec::new();
            choices.push("Fahrenheit".to_string());
            choices.push("Kelvin".to_string());
            let mut temp_select = Select::new("Select a temperature type: ", choices).prompt();
            match temp_select {
                Ok(temp) => {
                    match temp.as_str() {
                        "Fahrenheit" => {
                            let result = celsius_to_fahrenheit(&am.parse::<f64>().unwrap());
                            let out = Celsius {
                                temp: float_amount,
                                as_fahrenheit: celsius_to_fahrenheit(&float_amount),
                                as_kelvin: celsius_to_kelvin(&float_amount),
                            };
                            println!("test: {}", out.temp);
                        },
                        "Kelvin" => {
                            let result = celsius_to_kelvin(&am.parse::<f64>().unwrap());
                            println!("{} degrees Celsius is {} degrees Kelvin", am, result);
                        },
                        &_ => todo!(),
                    }
                },
                Err(_) => {
                    println!("Error")
                }
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

    fn show_fahrenheit(&self) -> f64 {
        self.as_fahrenheit
    }
    
    fn show_kelvin(&self) -> f64 {
        self.as_kelvin
    }
}

fn celsius_to_fahrenheit(temp: &f64) -> f64 {
    (temp * 1.8) + 32.0
}

fn celsius_to_kelvin(temp: &f64) -> f64 {
    temp + 273.15
}