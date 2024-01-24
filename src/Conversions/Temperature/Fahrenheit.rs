use inquire::{Select, Text};
#[path = "TempTypeEnum.rs"] mod TempType;

struct Fahrenheit{
    temp: f64,
    as_celsius: f64,
    as_kelvin: f64,
}

impl Fahrenheit {
    fn new(temp: f64) -> Self {
        Self {
            temp,
            as_celsius: fahrenheit_to_celsius(&temp),
            as_kelvin: fahrenheit_to_kelvin(temp),
        }
    }
}

pub fn conversion_from() {
    let mut TypesOfTemp = Vec::new();
    TypesOfTemp.push("Celsius");
    TypesOfTemp.push("Kelvin");
    //Prompt user for temperature to convert to
    let mut temp_select = Select::new("Select a temperature type: ", TypesOfTemp).prompt();
    match temp_select {
        Ok(temp) => {
            match temp {
                "Celsius" => {
                    let amount = Text::new("How many degrees Celsius? ").prompt();
                    match amount {
                        Ok(am) => {
                            let am: f64 = am.parse().unwrap();
                            let result = celsius_to_fahrenheit(&am);
                            println!("{} degrees Celsius is {} degrees Fahrenheit", am, result);
                        },
                        Err(_) => {
                            println!("Error")
                        }
                    }
                },
                "Kelvin" => {
                    let amount = Text::new("How many degrees Kelvin? ").prompt();
                    match amount {
                        Ok(am) => {
                            let am: f64 = am.parse().unwrap();
                            let result = kelvin_to_fahrenheit(am);
                            println!("{} degrees Kelvin is {} degrees Fahrenheit", am, result);
                        },
                        Err(_) => {
                            println!("Error")
                        }
                    }
                },
                &_ => todo!(),          
            }
        },
       
        Err(_) => {
            println!("Error")
        }
    }
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

