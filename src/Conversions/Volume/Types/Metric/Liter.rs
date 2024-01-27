use inquire::{Select, Text};


#[derive(Default)]
struct Liter {
    value: f64,
}

impl Liter {
    fn new(value: f64) -> Liter {
        Liter {
            value,
        }
    }

    fn get_value(&self) -> f64 {
        self.value
    }

    fn set_value(&mut self, new_value: f64) {
        self.value = new_value;
    }

    fn to_teaspoon(&self) -> f64 {
        self.value * 202.884
    }

    fn to_tablespoon(&self) -> f64 {
        self.value * 67.628
    }

    fn to_fluid_ounce(&self) -> f64 {
        self.value * 33.814
    }

    fn to_gill(&self) -> f64 {
        self.value * 4.22675
    }

    fn to_cup(&self) -> f64 {
        self.value * 4.22675
    }

    fn to_pint(&self) -> f64 {
        self.value * 2.11338
    }

    fn to_quart(&self) -> f64 {
        self.value * 1.05669
    }

    fn to_gallon(&self) -> f64 {
        self.value * 0.264172
    }

    fn to_centiliters(&self) -> f64 {
        self.value * 100.0
    }

    fn to_deciliters(&self) -> f64 {
        self.value * 10.0
    }

    fn to_milliliters(&self) -> f64 {
        self.value * 1000.0
    }
}

fn liter_factory() -> Liter {
    let mut liter = Liter::default();
    let mut liter_am_str = Text::new("How many liters do you want to convert?").prompt().unwrap();
    match liter_am_str.parse::<f64>() {
        Ok(liter_am) => {
            liter.set_value(liter_am);
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    liter
}

fn liter_imperial(l: &Liter) {
    let mut options = vec!["Teaspoon", "Tablespoon", "Fluid Ounce", "Gill", "Cup", "Pint", "Quart", "Gallon"];
    let select_liter = Select::new("Select an imperial unit to convert to: ", options).prompt();
    match select_liter {
        Ok(liter) => {
            match liter {
                "Teaspoon" => {
                    println!("{} liters is {} teaspoons", l.get_value(), l.to_teaspoon());
                },
                "Tablespoon" => {
                    println!("{} liters is {} tablespoons", l.get_value(), l.to_tablespoon());
                },
                "Fluid Ounce" => {
                    println!("{} liters is {} fluid ounces", l.get_value(), l.to_fluid_ounce());
                },
                "Gill" => {
                    println!("{} liters is {} gills", l.get_value(), l.to_gill());
                },
                "Cup" => {
                    println!("{} liters is {} cups", l.get_value(), l.to_cup());
                },
                "Pint" => {
                    println!("{} liters is {} pints", l.get_value(), l.to_pint());
                },
                "Quart" => {
                    println!("{} liters is {} quarts", l.get_value(), l.to_quart());
                },
                "Gallon" => {
                    println!("{} liters is {} gallons", l.get_value(), l.to_gallon());
                },
                _ => {
                    println!("Error");
                }
            }
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn liter_metric(l: &Liter) {
    let options = vec!["Centiliters", "Deciliters", "Milliliters", ];
    let select_liter = Select::new("Select a metric unit to convert to: ", options).prompt();
    match select_liter {
        Ok(liter) => {
            match liter {
                "Centiliters" => {
                    println!("{} liters is {} centiliters", l.get_value(), l.to_centiliters());
                },
                "Deciliters" => {
                    println!("{} liters is {} deciliters", l.get_value(), l.to_deciliters());
                },
                "Milliliters" => {
                    println!("{} liters is {} milliliters", l.get_value(), l.to_milliliters());
                },
                _ => {
                    println!("Error");
                }
            }
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn switch_liter() {
    let l = liter_factory();
    let options = vec!["Imperial", "Metric"];
    let select_liter = Select::new("Select a unit type to convert from: ", options).prompt();
    match select_liter {
        Ok(liter) => {
            match liter {
                "Imperial" => {
                    liter_imperial(&l);
                },
                "Metric" => {
                    liter_metric(&l);
                },
                _ => {
                    println!("Error");
                }
            }
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

pub fn convert_liter() {
    let convert_liter_loop: bool = true;
    while convert_liter_loop {
        switch_liter();
        let options = vec!["Yes", "No"];
        let select_liter = Select::new("Do you want to convert another liter?", options).prompt();
        match select_liter {
            Ok(liter) => {
                match liter {
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
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}