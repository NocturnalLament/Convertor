use inquire::{Select, Text};

struct FlOz {
    value: f64,
}

impl FlOz {
    pub fn new(value: f64) -> FlOz {
        FlOz {
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
        self.value * 6.0
    }

    pub fn to_tablespoon(&self) -> f64 {
        self.value * 2.0
    }

    pub fn to_cup(&self) -> f64 {
        self.value / 8.0
    }

    pub fn to_pint(&self) -> f64 {
        self.value / 16.0
    }

    pub fn to_quart(&self) -> f64 {
        self.value / 32.0
    }

    pub fn to_gallon(&self) -> f64 {
        self.value / 128.0
    }

    pub fn to_gill(&self) -> f64 {
        self.value / 4.0
    }

    pub fn to_centiliters(&self) -> f64 {
        self.value * 2.957
    }

    pub fn to_deciliters(&self) -> f64 {
        self.value * 0.2957
    }

    pub fn to_liters(&self) -> f64 {
        self.value / 33.814
    }

    pub fn to_milliliters(&self) -> f64 {
        self.value * 29.574
    }
}

fn fl_oz_prompt() -> FlOz {
    let value_text = Text::new("Enter the number of fluid ounces: ").prompt();
    match value_text {
        Ok(value) => {
            match value.parse::<f64>() {
                Ok(value) => {
                    FlOz::new(value)
                },

                Err(_) => {
                    println!("Please enter a valid number!");
                    fl_oz_prompt()
                }
            }
        },
        Err(_) => {
            println!("Please enter a valid number!");
            fl_oz_prompt()
        }
    }
}

fn fl_oz_imperial(fl: &FlOz) {
    let select_choice = Select::new("What unit type are you converting to?", vec!["Teaspoon", "Tablespoon", "Cup", "Pint", "Quart", "Gallon", "Gill"]).prompt();
    match select_choice {
        Ok(selection) => {
            match selection {
                "Teaspoon" => {
                    println!("{} fluid ounces is {} teaspoons", fl.get_value(), fl.to_teaspoon());
                },

                "Tablespoon" => {
                    println!("{} fluid ounces is {} tablespoons", fl.get_value(), fl.to_tablespoon());
                },

                "Cup" => {
                    println!("{} fluid ounces is {} cups", fl.get_value(), fl.to_cup());
                },

                "Pint" => {
                    println!("{} fluid ounces is {} pints", fl.get_value(), fl.to_pint());
                },

                "Quart" => {
                    println!("{} fluid ounces is {} quarts", fl.get_value(), fl.to_quart());
                },

                "Gallon" => {
                    println!("{} fluid ounces is {} gallons", fl.get_value(), fl.to_gallon());
                },

                "Gill" => {
                    println!("{} fluid ounces is {} gills", fl.get_value(), fl.to_gill());
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

fn fl_oz_metric(fl: &FlOz) {
    let select_choice = Select::new("What unit type are you converting to?", vec!["Milliliter", "Liter", "Centiliter", "Deciliter"]).prompt();
    match select_choice {
        Ok(selection) => {
            match selection {
                "Milliliter" => {
                    println!("{} fluid ounces is {} milliliters", fl.get_value(), fl.to_milliliters());
                },

                "Liter" => {
                    println!("{} fluid ounces is {} liters", fl.get_value(), fl.to_liters());
                },

                "Centiliter" => {
                    println!("{} fluid ounces is {} centiliters", fl.get_value(), fl.to_centiliters());
                },

                "Deciliter" => {
                    println!("{} fluid ounces is {} deciliters", fl.get_value(), fl.to_deciliters());
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

fn fl_oz_switch() {
    let fl = fl_oz_prompt();
    let select_choice = Select::new("What unit type are you converting from?", vec!["Imperial", "Metric"]).prompt();
    match select_choice {
        Ok(selection) => {
            match selection {
                "Imperial" => {
                    fl_oz_imperial(&fl);
                },

                "Metric" => {
                    fl_oz_metric(&fl);
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

pub fn convert_fl_oz() {
    let loop_running: bool = true;
    while loop_running {
        fl_oz_switch();
        let select_choice = Select::new("Would you like to convert another unit?", vec!["Yes", "No"]).prompt();
        match select_choice {
            Ok(selection) => {
                match selection {
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