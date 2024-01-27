use inquire::{Select, Text};

pub struct Quart {
    value: f64,
}

impl Quart {
    pub fn new(value: f64) -> Quart {
        Quart {
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
        self.value * 192.0
    }

    pub fn to_tablespoon(&self) -> f64 {
        self.value * 64.0
    }

    pub fn to_fluid_ounce(&self) -> f64 {
        self.value * 32.0
    }

    pub fn to_cup(&self) -> f64 {
        self.value * 4.0
    }

    pub fn to_pint(&self) -> f64 {
        self.value * 2.0
    }

    pub fn to_gallon(&self) -> f64 {
        self.value / 4.0
    }

    pub fn to_centiliters(&self) -> f64 {
        self.value * 946.352946
    }

    pub fn to_deciliters(&self) -> f64 {
        self.value * 94.6352946
    }

    pub fn to_liters(&self) -> f64 {
        self.value * 9.46352946
    }

    pub fn to_milliliters(&self) -> f64 {
        self.value * 9463.52946
    }
}

fn quart_imperial(q: &Quart) {
    let imperial_type = Select::new("What unit type are you converting to?", vec!["Teaspoon", "Tablespoon", "Fluid Ounce", "Cup", "Pint", "Gallon", "Gill"]).prompt();
    match imperial_type {
        Ok(imp) => {
            match imp {
                "Teaspoon" => {
                    println!("{} quarts is {} teaspoons", q.get_value(), q.to_teaspoon());
                },
                "Tablespoon" => {
                    println!("{} quarts is {} tablespoons", q.get_value(), q.to_tablespoon());
                },
                "Fluid Ounce" => {
                    println!("{} quarts is {} fluid ounces", q.get_value(), q.to_fluid_ounce());
                },
                "Cup" => {
                    println!("{} quarts is {} cups", q.get_value(), q.to_cup());
                },
                "Pint" => {
                    println!("{} quarts is {} pints", q.get_value(), q.to_pint());
                },
                "Gallon" => {
                    println!("{} quarts is {} gallons", q.get_value(), q.to_gallon());
                },
                "Gill" => {
                    println!("{} quarts is {} gills", q.get_value(), q.to_gallon());
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

fn quart_metric(q: &Quart) {
    let metric_type = Select::new("What unit type are you converting to?", vec!["Milliliter", "Liter", "Centiliter", "Deciliter"]).prompt();
    match metric_type {
        Ok(met) => {
            match met {
                "Milliliter" => {
                    println!("{} quarts is {} milliliters", q.get_value(), q.to_milliliters());
                },
                "Liter" => {
                    println!("{} quarts is {} liters", q.get_value(), q.to_liters());
                },
                "Centiliter" => {
                    println!("{} quarts is {} centiliters", q.get_value(), q.to_centiliters());
                },
                "Deciliter" => {
                    println!("{} quarts is {} deciliters", q.get_value(), q.to_deciliters());
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

fn quart_prompt() -> Quart {
    let mut quart = Quart {
        value: 0.0,
    };

    let quart_value = Text::new("Enter the number of quarts: ").prompt();
    match quart_value {
        Ok(q) => {
            let q_val = q.parse::<f64>();
            match q_val {
                Ok(q) => {
                    quart.set_value(q);
                    return quart
                },
                Err(_) => {
                    println!("Error");
                }
            }
            return quart
        },
        Err(_) => {
            println!("Error");
            return quart
        }
    }
    quart
}

pub fn convert_quart() {
    let quart = quart_prompt();
    let unit_type = Select::new("What unit type are you converting to?", vec!["Imperial", "Metric"]).prompt();
    match unit_type {
        Ok(unit) => {
            match unit {
                "Imperial" => {
                    quart_imperial(&quart);
                },
                "Metric" => {
                    quart_metric(&quart);
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