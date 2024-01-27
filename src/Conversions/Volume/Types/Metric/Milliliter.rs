use inquire::{Select, Text};


#[derive(Default)]
struct Milliliter {
    value: f64,
}

impl Milliliter {
    pub fn new(value: f64) -> Milliliter {
        Milliliter {
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
        self.value * 0.202884
    }

    pub fn to_tablespoon(&self) -> f64 {
        self.value * 0.067628
    }

    pub fn to_fluid_ounce(&self) -> f64 {
        self.value * 0.033814
    }

    pub fn to_gill(&self) -> f64 {
        self.value * 0.00422675
    }

    pub fn to_cup(&self) -> f64 {
        self.value * 0.00422675
    }

    pub fn to_pint(&self) -> f64 {
        self.value * 0.00211338
    }

    pub fn to_quart(&self) -> f64 {
        self.value * 0.00105669
    }

    pub fn to_gallon(&self) -> f64 {
        self.value * 0.000264172
    }

    pub fn to_centiliters(&self) -> f64 {
        self.value * 0.1
    }

    pub fn to_deciliters(&self) -> f64 {
        self.value * 0.01
    }

    pub fn to_liters(&self) -> f64 {
        self.value * 0.001
    }

}

fn milliliter_factory() -> Milliliter {
    let mut milliliter = Milliliter::default();
    let milliliter_value = Text::new("Enter the number of milliliters: ").prompt();
    match milliliter_value {
        Ok(val) => {
           match val.parse::<f64>() {
               Ok(val_f) => {
                   milliliter.set_value(val_f);
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
    milliliter
}

fn milliliter_imperial(m: &Milliliter) {
    let imp_type = Select::new("Select the imperial unit to convert to: ", vec!["Teaspoon", "Tablespoon", "Fluid Ounce", "Gill", "Cup", "Pint", "Quart", "Gallon"]).prompt();
    match imp_type {
        Ok(imp) => {
            match imp {
                "Teaspoon" => {
                    println!("{} milliliters is {} teaspoons", m.get_value(), m.to_teaspoon());
                },
                "Tablespoon" => {
                    println!("{} milliliters is {} tablespoons", m.get_value(), m.to_tablespoon());
                },
                "Fluid Ounce" => {
                    println!("{} milliliters is {} fluid ounces", m.get_value(), m.to_fluid_ounce());
                },
                "Gill" => {
                    println!("{} milliliters is {} gills", m.get_value(), m.to_gill());
                },
                "Cup" => {
                    println!("{} milliliters is {} cups", m.get_value(), m.to_cup());
                },
                "Pint" => {
                    println!("{} milliliters is {} pints", m.get_value(), m.to_pint());
                },
                "Quart" => {
                    println!("{} milliliters is {} quarts", m.get_value(), m.to_quart());
                },
                "Gallon" => {
                    println!("{} milliliters is {} gallons", m.get_value(), m.to_gallon());
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

fn milliliter_metric(m: &Milliliter) {
    let met_type = Select::new("Select the metric unit to convert to: ", vec!["Centiliter", "Deciliter", "Liter", "Milliliter"]).prompt();
    match met_type {
        Ok(met) => {
            match met {
                "Centiliter" => {
                    println!("{} milliliters is {} centiliters", m.get_value(), m.to_centiliters());
                },
                "Deciliter" => {
                    println!("{} milliliters is {} deciliters", m.get_value(), m.to_deciliters());
                },
                "Liter" => {
                    println!("{} milliliters is {} liters", m.get_value(), m.to_liters());
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

fn milliliter_switch() {
    let milliliter = milliliter_factory();
    let unit_type = Select::new("Select the unit type to convert to: ", vec!["Imperial", "Metric"]).prompt();
    match unit_type {
        Ok(unit) => {
            match unit {
                "Imperial" => {
                    milliliter_imperial(&milliliter);
                },
                "Metric" => {
                    milliliter_metric(&milliliter);
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

pub fn convert_milliliter() {
    let convert_loop_running: bool = true;
    while convert_loop_running {
        milliliter_switch();
        let continue_loop = Select::new("Do you want to convert another unit?", vec!["Yes", "No"]).prompt();
        match continue_loop {
            Ok(cont) => {
                match cont {
                    "Yes" => {
                        continue;
                    },
                    "No" => {
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