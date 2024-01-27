use inquire::{Select, Text};

/* 
    Declare struct for pint which holds a value of a floating 64 bit number
*/
pub struct Pint {
    value: f64,
}

impl Pint {
    // Function to create a new pint
    pub fn new(value: f64) -> Pint {
        Pint {
            value,
        }
    }

    // Function to get the value of the pint
    pub fn get_value(&self) -> f64 {
        self.value
    }

    // Function to set the value of the pint
    pub fn set_value(&mut self, new_value: f64) {
        self.value = new_value;
    }

    // Function to convert the pint to teaspoons
    pub fn to_teaspoon(&self) -> f64 {
        self.value * 115.3
    }

    // Function to convert the pint to tablespoons
    pub fn to_tablespoon(&self) -> f64 {
        self.value * 38.43
    }

    // Function to convert the pint to fluid ounces
    pub fn to_fluid_ounce(&self) -> f64 {
        self.value * 19.22
    }

    // Function to convert the pint to cups
    pub fn to_cup(&self) -> f64 {
        self.value * 2.0
    }

    // Function to convert the pint to quarts
    pub fn to_quart(&self) -> f64 {
        self.value / 2.0
    }

    // Function to convert the pint to gallons
    pub fn to_gallon(&self) -> f64 {
        self.value / 8.0
    }

    // Function to convert the pint to centiliters
    pub fn to_centiliters(&self) -> f64 {
        self.value * 473.176473
    }

    // Function to convert the pint to deciliters
    pub fn to_deciliters(&self) -> f64 {
        self.value * 47.3176473
    }

    // Function to convert the pint to liters
    pub fn to_liters(&self) -> f64 {
        self.value * 4.73176473
    }

    // Function to convert the pint to milliliters
    pub fn to_milliliters(&self) -> f64 {
        self.value * 4731.76473
    }
}

// Function to prompt the user for the amount of pints
fn prompt_for_pint() -> Pint {
    let amount = Text::new("Enter the amount of pints: ").prompt();
    let mut pint = Pint::new(0.0);
    match amount {
        Ok(amount) => {
            match amount.parse::<f64>() {
                Ok(amount) => {
                    pint.set_value(amount);
                    pint
                },
                Err(_) => {
                    println!("Error");
                    pint
                }
            }
        },
        Err(_) => {
            println!("Error");
            pint
        }
    }
}

// Function to display the metric conversions
fn display_metric(pint: &Pint) {
    let met_type = Select::new("What unit type are you converting to?", vec!["Milliliter", "Liter", "Centiliter", "Deciliter"]).prompt();
    match met_type {
        Ok(met) => {
            match met {
                "Milliliter" => {
                    println!("{} pints is {} milliliters", pint.get_value(), pint.to_milliliters());
                },
                "Liter" => {
                    println!("{} pints is {} liters", pint.get_value(), pint.to_liters());
                },
                "Centiliter" => {
                    println!("{} pints is {} centiliters", pint.get_value(), pint.to_centiliters());
                },
                "Deciliter" => {
                    println!("{} pints is {} deciliters", pint.get_value(), pint.to_deciliters());
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

// Function to display the imperial conversions
fn display_imperial(pint: &Pint) {
    let imp_type = Select::new("What unit type are you converting to?", vec!["Teaspoon", "Tablespoon", "Fluid Ounce", "Cup", "Quart", "Gallon"]).prompt();
    match imp_type {
        Ok(imp) => {
            match imp {
                "Teaspoon" => {
                    println!("{} pints is {} teaspoons", pint.get_value(), pint.to_teaspoon());
                },
                "Tablespoon" => {
                    println!("{} pints is {} tablespoons", pint.get_value(), pint.to_tablespoon());
                },
                "Fluid Ounce" => {
                    println!("{} pints is {} fluid ounces", pint.get_value(), pint.to_fluid_ounce());
                },
                "Cup" => {
                    println!("{} pints is {} cups", pint.get_value(), pint.to_cup());
                },
                "Quart" => {
                    println!("{} pints is {} quarts", pint.get_value(), pint.to_quart());
                },
                "Gallon" => {
                    println!("{} pints is {} gallons", pint.get_value(), pint.to_gallon());
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

// Function to get the type of volume
fn get_type() {
    // Prompt the user to select a volume type
    let volume_select = Select::new("Select a volume type: ", vec!["Imperial", "Metric"]).prompt();
    // Return the selected volume type or the error if there is one
    let pint = prompt_for_pint();
    match volume_select {
        //If the user selects a volume type, match the volume type
        Ok(volume) => {
            match volume {
                "Imperial" => {
                    display_imperial(&pint);
                },
                "Metric" => {
                    display_metric(&pint);
                },
                &_ => {
                    println!("Error");
                }
            }
        },
        Err(e) => {println!("Error: {}", e)},
    }
}

// Entry point
pub fn convert_pint() {
    let convert_loop_running: bool = true;
    while convert_loop_running {
        get_type();
        let continue_loop = Select::new("Would you like to get another pint? (y/n) ", vec!["y", "n"]).prompt();
        match continue_loop {
            Ok(ans) => {
                match ans {
                    "y" => {
                        continue;
                    },
                    "n" => {
                        break;
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
}

