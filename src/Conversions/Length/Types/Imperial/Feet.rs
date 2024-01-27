use inquire::{Select, Text};

struct Feet {
    value: f64,
}

impl Feet {
    fn new(value: f64) -> Self {
        Self {
            value,
        }
    }

    fn show_centimeters(&self) -> f64 {
        self.value * 30.48
    }

    fn show_chain(&self) -> f64 {
        self.value * 0.0151515
    }

    fn show_meters(&self) -> f64 {
        self.value * 0.3048
    }

    fn show_kilometers(&self) -> f64 {
        self.value * 0.0003048
    }

    fn show_miles(&self) -> f64 {
        self.value * 0.000189394
    }

    fn show_yards(&self) -> f64 {
        self.value * 0.333333
    }

    fn show_feet(&self) -> f64 {
        self.value
    }

    fn show_millimeters(&self) -> f64 {
        self.value * 304.8
    }

    fn show_inches(&self) -> f64 {
        self.value * 12.0
    }
}

fn convert_metric() {
    let amount = Text::new("How many feet are you converting? ").prompt();
    match amount {
        Ok(amount_val) => {
            let amount_val = amount_val.parse::<f64>().unwrap();
            let feet = Feet::new(amount_val);
            let choice = Select::new("What are you converting to?", vec!["Centimeters", "Decimeters", "Kilometers", "Meters", "Millimeters"]).prompt();
            match choice {
                Ok(choice_val) => {
                    match choice_val {
                        "Centimeters" => {
                            println!("{}", feet.show_centimeters());
                        },
                        "Decimeters" => {
                            println!("{}", feet.show_meters());
                        
                        },
                        "Kilometers" => {
                            println!("{}", feet.show_kilometers());              
                        },
                        "Meters" => {
                            println!("{}", feet.show_meters());
                        
                        },
                        "Millimeters" => {
                            println!("{}", feet.show_millimeters());
                        }
                        _ => println!("Error"),
                    }
                },
                Err(e) => println!("Error: {}", e),
            }
        },
        Err(e) => println!("Error: {}", e),
    }
}

fn convert_imperial() {
    let amount = Text::new("How many feet are you converting? ").prompt();
    match amount {
        Ok(amount_val) => {
            let amount_val = amount_val.parse::<f64>().unwrap();
            let feet = Feet::new(amount_val);
            let choice = Select::new("What are you converting f to?", vec!["Chain", "Feet", "Furlong", "Inch", "Mile"]).prompt();
            match choice {
                Ok(choice_val) => {
                    println!("{}", choice_val);
                    match choice_val {
                        "Chain" => {
                            println!("{}", feet.show_chain());
                        },
                        "Furlong" => {
                            println!("{}", feet.show_meters());
                        },
                        "Inch" => {
                            println!("{}", feet.show_inches());
                        }
                        "Mile" => {
                            println!("{}", feet.show_miles());
                        },
                        "Yard" => {
                            println!("{}", feet.show_yards());
                        }
                        _ => println!("Error: {}", choice_val),
                    }
                },
                Err(e) => println!("Error: {}", e),
            }
        },
        Err(e) => println!("Error: {}", e),
    }

}

fn feet_switch() {
    let choice = Select::new("What are you converting to?", vec!["Metric", "Imperial"]).prompt();
    match choice {
        Ok(choice_val) => {
            match choice_val {
                "Metric" => convert_metric(),
                "Imperial" =>{ convert_imperial()},
                _ => println!("Errord: {}", choice_val),
            }
        },
        Err(e) => println!("Error: {}", e),
    }
}

pub fn convert_feet() {
    let feet_loop_running = true;
    while feet_loop_running {
        feet_switch();
        let choice = Select::new("Do you want to convert again?", vec!["Yes", "No"]).prompt();
        match choice {
            Ok(choice_val) => {
                match choice_val {
                    "Yes" => continue,
                    "No" => break,
                    _ => println!("Error"),
                }
            },
            Err(e) => println!("Error: {}", e),
        }
    }
}