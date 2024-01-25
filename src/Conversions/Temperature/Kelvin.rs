use inquire::{Select, Text};

struct KelvinUnit {
    value: f64,
    as_fahrenheit: f64,
    as_celsius: f64,
}
//Convert to fahrenheit
fn kelvin_to_fahrenheit(temp: &f64) -> f64 {
    (temp - 273.15) * 9.0 / 5.0 + 32.0
}
//Convert to celsius
fn kelvin_to_celsius(temp: &f64) -> f64 {
    temp - 273.15
}
//Kelvin Struct
impl KelvinUnit {
    fn new(temp: f64) -> Self {
        Self {
            value: temp,
            as_fahrenheit: kelvin_to_fahrenheit(&temp),
            as_celsius: kelvin_to_celsius(&temp),
        }
    }
    //Show fahrenheit
    fn show_fahrenheit(&self) -> f64 {
        self.as_fahrenheit
    }
    //Show celsius
    fn show_celsius(&self) -> f64 {
        self.as_celsius
    }
}

pub fn get_kelvin() {
    let amount = Text::new("How many degrees Kelvin? ").prompt();
    match amount {
        Ok(am) => {
            /*
                Get amount to convert
            */
            let am: f64 = am.parse().unwrap();
            //Create kelvin obj and convert to both fahrenheit and celsius
            let kelvin = KelvinUnit::new(am);
            let mut choices: Vec<String> = Vec::new();
            choices.push("Fahrenheit".to_string());
            choices.push("Celsius".to_string());
            //Select which conversion to show
            let mut temp_select = Select::new("Select a temperature type to convert to: ", choices).prompt();
            match temp_select {
                Ok(temp) => {
                    match temp.as_str() {
                        "Fahrenheit" => {
                            println!("{} degrees Kelvin is {} degrees Fahrenheit", kelvin.value, kelvin.show_fahrenheit());
                        },
                        "Celsius" => {
                            println!("{} degrees Kelvin is {} degrees Celsius", kelvin.value, kelvin.show_celsius());
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