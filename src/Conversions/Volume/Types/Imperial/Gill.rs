use inquire::{Select, Text};
//Struct to hold Gill data
struct Gill {
    value: f64,
}

impl Gill {
    pub fn new(value: f64) -> Gill {
        Gill {
            value,
        }
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }

    pub fn set_value(&mut self, new_value: f64) {
        self.value = new_value;
    }

    pub fn to_teaspoon(&self) -> f64 {
        self.value * 24.0
    }

    pub fn to_tablespoon(&self) -> f64 {
        self.value * 8.0
    }

    pub fn to_fluid_ounce(&self) -> f64 {
        self.value * 4.0
    }

    pub fn to_cup(&self) -> f64 {
        self.value / 2.0
    }

    pub fn to_gallon(&self) -> f64 {
        self.value / 32.0
    }

    pub fn to_pint(&self) -> f64 {
        self.value / 4.0
    }

    pub fn to_quart(&self) -> f64 {
        self.value / 8.0
    }

    pub fn to_centiliters(&self) -> f64 {
        self.value * 11.829
    }

    pub fn to_deciliters(&self) -> f64 {
        self.value * 1.1829
    }

    pub fn to_liters(&self) -> f64 {
        self.value / 8.454
    }

    pub fn to_milliliters(&self) -> f64 {
        self.value * 118.29
    }
}

fn gill_prompt() -> Gill {
    let amount = Text::new("Enter the amount of gill: ").prompt();
    let mut gill = Gill::new(0.0);
    match amount {
        Ok(amount) => {
            match amount.parse::<f64>() {
                Ok(amount) => {
                    gill.set_value(amount);
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
    gill

}

fn get_imperial_gill(g: &Gill) {
    let met_type = Select::new("What unit type are you converting to?", vec!["Teaspoon", "Tablespoon", "Fluid Ounce", "Cup", "Gallon", "Pint", "Quart",]).prompt();
    match met_type {
        Ok(met) => {
            match met {
                "Teaspoon" => {
                    println!("{} gills is {} teaspoons", g.get_value(), g.to_teaspoon());
                },
                "Tablespoon" => {
                    println!("{} gills is {} tablespoons", g.get_value(), g.to_tablespoon());
                },
                "Fluid Ounce" => {
                    println!("{} gills is {} fluid ounces", g.get_value(), g.to_fluid_ounce());
                },
                "Cup" => {
                    println!("{} gills is {} cups", g.get_value(), g.to_cup());
                },
                "Gallon" => {
                    println!("{} gills is {} gallons", g.get_value(), g.to_gallon());
                },
                "Pint" => {
                    println!("{} gills is {} pints", g.get_value(), g.to_pint());
                },
                "Quart" => {
                    println!("{} gills is {} quarts", g.get_value(), g.to_quart());
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

fn get_metric_gill(g: &Gill) {
    let met_type = Select::new("What unit type are you converting to?", vec!["Milliliter", "Liter", "Centiliter", "Deciliter"]).prompt();
    match met_type {
        Ok(met) => {
            match met {
                "Milliliter" => {
                    println!("{} gills is {} milliliters", g.get_value(), g.to_milliliters());
                },
                "Liter" => {
                    println!("{} gills is {} liters", g.get_value(), g.to_liters());
                },
                "Centiliter" => {
                    println!("{} gills is {} centiliters", g.get_value(), g.to_centiliters());
                },
                "Deciliter" => {
                    println!("{} gills is {} deciliters", g.get_value(), g.to_deciliters());
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

fn switch_gill() {
    let gill = gill_prompt();
    let unit_type = Select::new("What unit type are you converting from?", vec!["Imperial", "Metric"]).prompt();
    match unit_type {
        Ok(unit) => {
            match unit {
                "Imperial" => {
                    get_imperial_gill(&gill);
                },
                "Metric" => {
                    get_metric_gill(&gill);
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

pub fn gill_entry_point() {
    let loop_running: bool = true;
    while loop_running {
        switch_gill();
        let continue_loop = Select::new("Would you like to get another gill? (y/n) ", vec!["y", "n"]).prompt();
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