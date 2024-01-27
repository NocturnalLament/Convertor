use std::f32::consts::E;

use inquire::{Select, Text};

struct Yard {
    value: f64,
}

impl Yard {
    fn new(value: f64) -> Self {
        Self {
            value,
        }
    }

    fn show_centimeters(&self) -> f64 {
        self.value * 91.44
    }

    fn show_meters(&self) -> f64 {
        self.value * 0.9144
    }

    fn show_kilometers(&self) -> f64 {
        self.value * 0.0009144
    }

    fn show_miles(&self) -> f64 {
        self.value * 0.000568182
    }

    fn show_yards(&self) -> f64 {
        self.value
    }

    fn show_feet(&self) -> f64 {
        self.value * 3.0
    }

    fn show_millimeters(&self) -> f64 {
        self.value * 914.4
    }

    fn show_decimeters(&self) -> f64 {
        self.value * 9.144
    }

    fn show_inches(&self) -> f64 {
        self.value * 36.0
    }

    fn show_furlong(&self) -> f64 {
        self.value * 0.00454545
    }

    fn show_chain(&self) -> f64 {
        self.value * 0.0454545
    }

    fn to_miles(&self) -> f64 {
        self.value * 0.000568182
    }


}

fn yard_metric() {
    let amount = Text::new("How many yards are you converting? ").prompt();
    match amount {
        Ok(amount_val) => {
            let amount_val = amount_val.parse::<f64>().unwrap();
            let yard = Yard::new(amount_val);
            let choice = Select::new("What are you converting to?", vec!["Centimeters", "Kilometers", "Meters", "Millimeters", "Decimeters"]).prompt();
            match choice {
                Ok(choice_val) => {
                    match choice_val {
                        "Centimeters" => {
                            println!("{}", yard.show_centimeters());
                        },
                        "Kilometers" => {
                            println!("{}", yard.show_kilometers());
                        },
                        "Meters" => {
                            println!("{}", yard.show_meters());
                        },
                        "Millimeters" => {
                            println!("{}", yard.show_millimeters());
                        },
                        "Decimeters" => {
                            println!("{}", yard.show_decimeters());
                        },

                        _ => {
                            println!("Error");
                        }
                    }
                },
                Err(_) => {
                    println!("Error");
                }
            }
        },
        Err(_) => {
            println!("Error");
        }
    }
}

fn yard_imperial() {
    let amount = Text::new("How many yards are you converting? ").prompt();
    match amount {
        Ok(am) => {
            let amount_f = am.parse::<f64>().unwrap();
            let yard = Yard::new(amount_f);
            let yard_option = Select::new("What would you like to convert to? ", vec!["Feet", "Inches", "Miles", "Chains", "Furlongs"]).prompt();
    match yard_option {
        Ok(yards) => {
            match yards {
                "Feet" => {
                    println!("{}", yard.show_feet());
                },
                "Inches" => {
                    println!("Inches: {}", yard.show_inches());
                },
                "Miles" => {
                    println!("Miles: {}",yard.to_miles());
                },
                "Chains" => {
                    println!("Chains: {}", yard.show_chain());
                },
                "Furlongs" => {
                    println!("Furlongs: {}", yard.show_furlong());
                },
                _ => {
                    println!("Error");
                }
            }
        },
        Err(_) => {
            println!("Error");
        }
    }
        },
        Err(_) => {
            println!("Error");  
        }
    }

}

fn yard_switch() {
    let yard_option = Select::new("What would you like to convert to? ", vec!["Imperial", "Metric"]).prompt();
    match yard_option {
        Ok(yards) => {
            match yards {
                "Imperial" => {
                    yard_imperial();
                },
                "Metric" => {
                    yard_metric();
                },
                _ => {
                    println!("Error");
                }
            }
        },
        Err(_) => {
            println!("Error");
        }
    }

    }

pub fn convert_yard() {
    let running = true;
    while running {
        yard_switch();
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