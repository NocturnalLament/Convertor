use inquire::{Select, Text};

pub struct Gallon {
    value: f64,
}

impl Gallon {
    pub fn new(value: f64) -> Gallon {
        Gallon {
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
        self.value * 768.0
    }

    pub fn to_tablespoon(&self) -> f64 {
        self.value * 256.0
    }

    pub fn to_fluid_ounce(&self) -> f64 {
        self.value * 128.0
    }

    pub fn to_cup(&self) -> f64 {
        self.value * 16.0
    }

    pub fn to_pint(&self) -> f64 {
        self.value / 8.0
    }

    pub fn to_quart(&self) -> f64 {
        self.value * 4.0
    }

    pub fn to_gill(&self) -> f64 {
        self.value / 16.0
    }

    pub fn to_centiliters(&self) -> f64 {
        self.value * 378.5
    }

    pub fn to_deciliters(&self) -> f64 {
        self.value * 37.854
    }

    pub fn to_milliliters(&self) -> f64 {
        self.value * 3785.4
    }

    pub fn to_liters(&self) -> f64 {
        self.value * 3.7854
    }
}

fn gallon_factory() -> Gallon {
    let value_text = Text::new("Enter the number of gallons: ").prompt();
    let mut gal: Gallon = Gallon::new(0.0);
    match value_text {
        Ok(value) => {
            match value.parse::<f64>() {
                Ok(value) => {
                    gal.set_value(value);
                    gal
                },
                Err(_) => {
                    println!("Error");
                    Gallon::new(0.0)
                }
            }
        },
        Err(_) => {
            println!("Error");
            Gallon::new(0.0)
        }
    }
}

fn convert_imperial(g: &Gallon) {
    let conv_type = Select::new("Convert to: ", vec!["teaspoons", "tablespoons", "fluid ounces", "cups", "pints", "quarts", "gills"]).prompt();
    match conv_type {
        Ok(conv) => {
            match conv {
                "teaspoons" => {
                    println!("{} gallons is {} teaspoons", g.get_value(), g.to_teaspoon());
                },
                "tablespoons" => {
                    println!("{} gallons is {} tablespoons", g.get_value(), g.to_tablespoon());
                },
                "fluid ounces" => {
                    println!("{} gallons is {} fluid ounces", g.get_value(), g.to_fluid_ounce());
                },
                "cups" => {
                    println!("{} gallons is {} cups", g.get_value(), g.to_cup());
                },
                "pints" => {
                    println!("{} gallons is {} pints", g.get_value(), g.to_pint());
                },
                "quarts" => {
                    println!("{} gallons is {} quarts", g.get_value(), g.to_quart());
                },
                "gills" => {
                    println!("{} gallons is {} gills", g.get_value(), g.to_gill());
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

fn convert_metric(g: &Gallon) {
    let conv_type = Select::new("Convert to: ", vec!["milliliters", "liters", "centiliters", "deciliters"]).prompt();
    match conv_type {
        Ok(conv) => {
            match conv {
                "milliliters" => {
                    println!("{} gallons is {} milliliters", g.get_value(), g.to_milliliters());
                },
                "liters" => {
                    println!("{} gallons is {} liters", g.get_value(), g.to_liters());
                },
                "centiliters" => {
                    println!("{} gallons is {} centiliters", g.get_value(), g.to_centiliters());
                },
                "deciliters" => {
                    println!("{} gallons is {} deciliters", g.get_value(), g.to_deciliters());
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

fn gallon_switch() {
    let gal = gallon_factory();
        let unit_type = Select::new("Convert to: ", vec!["Imperial", "Metric"]).prompt();
        match unit_type {
            Ok(unit) => {
                match unit {
                    "Imperial" => {
                        convert_imperial(&gal);
                    },
                    "Metric" => {
                        convert_metric(&gal);
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


pub fn convert_gallon() {
    let gallon_running: bool = true;
    while gallon_running {
        gallon_switch();
        let continue_loop = Select::new("Would you like to get another gallon? (y/n) ", vec!["y", "n"]).prompt();
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