use inquire::{Select, Text};

pub struct Teaspoon {
    // The value of the volume in teaspoons
    value: f64,
}

impl Teaspoon {
    pub fn new(value: f64) -> Teaspoon {
        Teaspoon {
            value,
        }
    }
    // Function to get the value of the volume in teaspoons
    pub fn get_value(&self) -> f64 {
        self.value
    }

    // Function to set the value of the volume in teaspoons
    pub fn set_value(&mut self, new_value: f64) {
        self.value = new_value;
    }

    // Function to convert the volume from teaspoons to tablespoons
    pub fn to_tablespoon(&self) -> f64 {
        self.value / 3.0
    }

    // Function to convert the volume from teaspoons to fluid ounces
    pub fn to_fluid_ounce(&self) -> f64 {
        self.value / 6.0
    }

    // Function to convert the volume from teaspoons to cups
    pub fn to_cup(&self) -> f64 {
        self.value / 48.0
    }

    // Function to convert the volume from teaspoons to pints
    pub fn to_pint(&self) -> f64 {
        self.value / 96.0
    }

    // Function to convert the volume from teaspoons to quarts
    pub fn to_quart(&self) -> f64 {
        self.value / 192.0
    }

    // Function to convert the volume from teaspoons to gallons
    pub fn to_gallon(&self) -> f64 {
        self.value / 768.0
    }
}

fn create_teaspoon() -> Teaspoon {
    let amount = Text::new("Enter the amount of teaspoons: ").prompt();
    let mut teaspoon = Teaspoon::new(0.0);
    match amount {
        Ok(amount) => {
            match amount.parse::<f64>() {
                Ok(amount) => {
                    teaspoon.set_value(amount);
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
    teaspoon
}

fn display_metric(teaspoons: &Teaspoon) {
    let met_type = Select::new("What unit type are you converting to?", vec!["Milliliter", "Liter", "Centiliter", "Deciliter"]).prompt();
    match met_type {
        Ok(met) => {
            match met {
                "Milliliter" => {
                    println!("{} teaspoons is {} milliliters", teaspoons.get_value(), teaspoons.to_tablespoon());
                },
                "Liter" => {
                    println!("{} teaspoons is {} liters", teaspoons.get_value(), teaspoons.to_fluid_ounce());
                },
                "Centiliter" => {
                    println!("{} teaspoons is {} centiliters", teaspoons.get_value(), teaspoons.to_cup());
                },
                "Deciliter" => {
                    println!("{} teaspoons is {} deciliters", teaspoons.get_value(), teaspoons.to_pint());
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

fn display_imperial(teaspoons: &Teaspoon) {
    let imp_type = Select::new("What unit type are you converting to?", vec![
        "Tablespoon", "Fluid Ounce", "Cup", "Pint", "Quart", "Gallon", "Gill"]).prompt();
        match imp_type {
            Ok(imp) => {
                match imp {
                    "Tablespoon" => {
                        println!("{} teaspoons is {} tablespoons", teaspoons.get_value(), teaspoons.to_tablespoon());
                    },
                    "Fluid Ounce" => {
                        println!("{} teaspoons is {} fluid ounces", teaspoons.get_value(), teaspoons.to_fluid_ounce());
                    },
                    "Cup" => {
                        println!("{} teaspoons is {} cups", teaspoons.get_value(), teaspoons.to_cup());
                    },
                    "Pint" => {
                        println!("{} teaspoons is {} pints", teaspoons.get_value(), teaspoons.to_pint());
                    },
                    "Quart" => {
                        println!("{} teaspoons is {} quarts", teaspoons.get_value(), teaspoons.to_quart());
                    },
                    "Gallon" => {
                        println!("{} teaspoons is {} gallons", teaspoons.get_value(), teaspoons.to_gallon());
                    },
                    "Gill" => {
                        println!("{} teaspoons is {} gills", teaspoons.get_value(), teaspoons.to_gallon() * 4.0);
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



fn teaspoon_switch() {
    let teaspoon_type = Select::new("What unit type are you converting to?", 
    vec!["Imperial", "Metric"]).prompt();
    match teaspoon_type {
        Ok(tea_type) => {
            let create_the_teaspoon = create_teaspoon();
            match tea_type {
                "Imperial" => {
                    display_imperial(&create_the_teaspoon);
                },
                "Metric" => {
                    display_metric(&create_the_teaspoon);
                },
                &_ => {
                    println!("Error");
                }
            }
        },
        Err(_) => {
            println!("Error");
        }
    }
}

pub fn convert_teaspoon() {
    // Flag to keep the teaspoon conversion running
    let mut loop_running: bool = true;
    while loop_running {
        teaspoon_switch();
        let conversion_continue = Select::new("Would you like to get another teaspoon? (y/n) ", vec!["y", "n"]).prompt();
        match conversion_continue {
            Ok(ans) => {
                if ans == "n" {
                    loop_running = false;
                }
            },
            Err(_) => {
                println!("Error");
            }
        }
    }
}