use inquire::{Select, Text};

pub struct Tablespoon {
    value: f64,
}

impl Tablespoon {
    pub fn new(value: f64) -> Tablespoon {
        Tablespoon {
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
        self.value * 3.0
    }

    pub fn to_fluid_ounce(&self) -> f64 {
        self.value / 2.0
    }

    pub fn to_cup(&self) -> f64 {
        self.value / 16.0
    }

    pub fn to_pint(&self) -> f64 {
        self.value / 32.0
    }

    pub fn to_quart(&self) -> f64 {
        self.value / 64.0
    }

    pub fn to_gallon(&self) -> f64 {
        self.value / 256.0
    }

    pub fn to_centiliters(&self) -> f64 {
        self.value * 1.478676478125
    }

    pub fn to_deciliters(&self) -> f64 {
        self.value * 0.1478676478125
    }

    pub fn to_liters(&self) -> f64 {
        self.value * 0.01478676478125
    }

    pub fn to_milliliters(&self) -> f64 {
        self.value * 14.78676478125
    }
}

fn prompt_for_tablespoon() -> Tablespoon {
    let amount = Text::new("Enter the amount of tablespoons: ").prompt();
    let mut tablespoon = Tablespoon::new(0.0);
    match amount {
        Ok(amount_val) => {
            match amount_val.parse::<f64>() {
                Ok(amount_f) => {
                    tablespoon.set_value(amount_f);
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
    tablespoon
}

fn get_imperial(tablespoon: &Tablespoon){
    let imp_type = Select::new("What unit type are you converting to?", vec!["Teaspoon", "Fluid Ounce", "Cup", "Pint", "Quart", "Gallon"]).prompt();
    match imp_type {
        Ok(imp) => {
            match imp {
                "Teaspoon" => {
                    println!("{} tablespoons is {} teaspoons", tablespoon.get_value(), tablespoon.to_teaspoon());
                },
                "Fluid Ounce" => {
                    println!("{} tablespoons is {} fluid ounces", tablespoon.get_value(), tablespoon.to_fluid_ounce());
                },
                "Cup" => {
                    println!("{} tablespoons is {} cups", tablespoon.get_value(), tablespoon.to_cup());
                },
                "Pint" => {
                    println!("{} tablespoons is {} pints", tablespoon.get_value(), tablespoon.to_pint());
                },
                "Quart" => {
                    println!("{} tablespoons is {} quarts", tablespoon.get_value(), tablespoon.to_quart());
                },
                "Gallon" => {
                    println!("{} tablespoons is {} gallons", tablespoon.get_value(), tablespoon.to_gallon());
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

fn get_metric(tablespoon: &Tablespoon) {
    let met_type = Select::new("What unit type are you converting to?", vec!["Milliliter", "Liter", "Centiliter", "Deciliter"]).prompt();
    match met_type {
        Ok(met) => {
            match met {
                "Milliliter" => {
                    println!("{} tablespoons is {} milliliters", tablespoon.get_value(), tablespoon.to_milliliters());
                },
                "Liter" => {
                    println!("{} tablespoons is {} liters", tablespoon.get_value(), tablespoon.to_liters());
                },
                "Centiliter" => {
                    println!("{} tablespoons is {} centiliters", tablespoon.get_value(), tablespoon.to_centiliters());
                },
                "Deciliter" => {
                    println!("{} tablespoons is {} deciliters", tablespoon.get_value(), tablespoon.to_deciliters());
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

pub fn convert_tablespoon() {
    let tablespoon = prompt_for_tablespoon();
    let unit_type = Select::new("What unit type are you converting to?", vec!["Imperial", "Metric"]).prompt();
    match unit_type {
        Ok(unit) => {
            match unit {
                "Imperial" => {
                    get_imperial(&tablespoon);
                },
                "Metric" => {
                    get_metric(&tablespoon);
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

pub fn convert_to_tablespoon() {
    let loop_running: bool = true;
    while loop_running {
        convert_tablespoon();
        let continue_loop = Select::new("Would you like to get another tablespoon? (y/n) ", vec!["y", "n"]).prompt();
        match continue_loop {
            Ok(ans) => {
                if ans == "n" {
                    break;
                }
            },
            Err(_) => {
                println!("Error");
            }
        }
    }
}