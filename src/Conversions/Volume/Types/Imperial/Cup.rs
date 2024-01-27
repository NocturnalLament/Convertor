use inquire::{Select, Text};

#[derive(Default)]
struct Cup {
    value: f64,
}

impl Cup {
    pub fn new(value: f64) -> Cup {
        Cup {
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
        self.value * 48.0
    }

    pub fn to_tablespoon(&self) -> f64 {
        self.value * 16.0
    }

    pub fn to_fluid_ounce(&self) -> f64 {
        self.value * 8.0
    }

    pub fn to_gill(&self) -> f64 {
        self.value * 2.0
    }

    pub fn to_pint(&self) -> f64 {
        self.value / 2.0
    }

    pub fn to_quart(&self) -> f64 {
        self.value / 4.0
    }

    pub fn to_gallon(&self) -> f64 {
        self.value / 16.0
    }

    pub fn to_centiliters(&self) -> f64 {
        self.value * 236.59
    }

    pub fn to_deciliters(&self) -> f64 {
        self.value * 23.659
    }

    pub fn to_liters(&self) -> f64 {
        self.value / 4.227
    }

    pub fn to_milliliters(&self) -> f64 {
        self.value * 236.59
    }
}

fn prompt_for_cup() -> Cup {
    let mut user_cup = Cup::default();
    let user_cup_val_str = Text::new("Enter the value of cups to convert: ").prompt();
    match user_cup_val_str {
        Ok(user_cup_val) => {
            match user_cup_val.parse::<f64>() {
                Ok(user_cup_f) => {
                    user_cup.set_value(user_cup_f);
                    user_cup
                
                },
                Err(_) => {
                    println!("Please enter a valid number.");
                    prompt_for_cup()
                
            }
        }
            
        },
        Err(_) => {
            println!("Please enter a valid number.");
            prompt_for_cup()
        }
    }
}

fn imperial_cup(c: &Cup) {
    let cup_select = Select::new("Select a conversion: ", vec!["Teaspoon", "Tablespoon", "Fluid Ounce", "Gill", "Pint", "Quart", "Gallon"]).prompt();
    match cup_select {
        Ok(cup_sel) => {
            match cup_sel {
                "Teaspoon" => {
                    println!("{} cups is {} teaspoons", c.get_value(), c.to_teaspoon());
                },
        
                "Tablespoon" => {
                    println!("{} cups is {} tablespoons", c.get_value(), c.to_tablespoon());
                },
        
                "Fluid Ounce" => {
                    println!("{} cups is {} fluid ounces", c.get_value(), c.to_fluid_ounce());
                },
        
                "Gill" => {
                    println!("{} cups is {} gills", c.get_value(), c.to_gill());
                },
        
                "Pint" => {
                    println!("{} cups is {} pints", c.get_value(), c.to_pint());
                },
        
                "Quart" => {
                    println!("{} cups is {} quarts", c.get_value(), c.to_quart());
                },
        
                "Gallon" => {
                    println!("{} cups is {} gallons", c.get_value(), c.to_gallon());
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

fn metric_cup(c: &Cup) {
    let cup_select = Select::new("Select a conversion: ", vec!["Milliliter", "Centiliter", "Deciliter", "Liter"]).prompt();
    match cup_select {
        Ok(cup_sel) => {
            match cup_sel {
                "Milliliter" => {
                    println!("{} cups is {} milliliters", c.get_value(), c.to_milliliters());
                },
        
                "Centiliter" => {
                    println!("{} cups is {} centiliters", c.get_value(), c.to_centiliters());
                },
        
                "Deciliter" => {
                    println!("{} cups is {} deciliters", c.get_value(), c.to_deciliters());
                },
        
                "Liter" => {
                    println!("{} cups is {} liters", c.get_value(), c.to_liters());
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

fn cup_switch() {
    let cup_select = Select::new("Select a unit type: ", vec!["Imperial", "Metric"]).prompt();
    match cup_select {
        Ok(cup_sel) => {
            match cup_sel {
                "Imperial" => {
                    let cup = prompt_for_cup();
                    imperial_cup(&cup);
                },
        
                "Metric" => {
                    let cup = prompt_for_cup();
                    metric_cup(&cup);
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

pub fn convert_cup() {
    let mut convert_cup_loop: bool = true;
    while convert_cup_loop {
        cup_switch();
        let mut cup_continue = Select::new("Do you want to convert another cup?", vec!["Yes", "No"]).prompt();
        match cup_continue {
            Ok(ans) => {
                match ans {
                    "Yes" => {
                        convert_cup_loop = true;
                    },
        
                    "No" => {
                        convert_cup_loop = false;
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