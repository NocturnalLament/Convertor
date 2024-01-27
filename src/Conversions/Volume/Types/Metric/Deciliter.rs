use inquire::{Select, Text};

#[derive(Default)]
struct Deciliter {
    value: f64,
}

impl Deciliter {
    fn new(value: f64) -> Deciliter {
        Deciliter {
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
        self.value * 0.202884
    }

    fn to_tablespoon(&self) -> f64 {
        self.value * 0.067628
    }

    fn to_fluid_ounce(&self) -> f64 {
        self.value * 0.033814
    }

    fn to_gill(&self) -> f64 {
        self.value * 0.00422675
    }

    fn to_cup(&self) -> f64 {
        self.value * 0.00422675
    }

    fn to_pint(&self) -> f64 {
        self.value * 0.00211338
    }

    fn to_quart(&self) -> f64 {
        self.value * 0.00105669
    }

    fn to_gallon(&self) -> f64 {
        self.value * 0.000264172
    }

    fn to_centiliters(&self) -> f64 {
        self.value * 0.1
    }

    fn to_deciliters(&self) -> f64 {
        self.value * 0.01
    }

    fn to_liters(&self) -> f64 {
        self.value * 0.1
    }

    fn to_milliliters(&self) -> f64 {
        self.value * 100.0
    }
}

fn deciliter_factory() -> Deciliter {
    let mut deciliter = Deciliter::default();
    let mut deciliter_value = Text::new("Enter the number of deciliters: ").prompt();
    match deciliter_value {
        Ok(value) => {
            deciliter.set_value(value.parse::<f64>().unwrap());
        },
        Err(_) => {
            println!("Error!");
        }
    }
    deciliter
}

fn deciliter_imperial(d: &Deciliter) {
    let options = vec![
        "Teaspoon",
        "Tablespoon",
        "Fluid Ounce",
        "Gill",
        "Cup",
        "Pint",
        "Quart",
        "Gallon",
    ];

    let deciliter_select = Select::new("Select a conversion: ", options).prompt();
    match deciliter_select {
        Ok(deciliter_select) => {
            match deciliter_select {
                "Teaspoon" => {
                    println!("{} deciliters is equal to {} teaspoons", d.get_value(), d.to_teaspoon());
                },
                "Tablespoon" => {
                    println!("{} deciliters is equal to {} tablespoons", d.get_value(), d.to_tablespoon());
                },
                "Fluid Ounce" => {
                    println!("{} deciliters is equal to {} fluid ounces", d.get_value(), d.to_fluid_ounce());
                },
                "Gill" => {
                    println!("{} deciliters is equal to {} gills", d.get_value(), d.to_gill());
                },
                "Cup" => {
                    println!("{} deciliters is equal to {} cups", d.get_value(), d.to_cup());
                },
                "Pint" => {
                    println!("{} deciliters is equal to {} pints", d.get_value(), d.to_pint());
                },
                "Quart" => {
                    println!("{} deciliters is equal to {} quarts", d.get_value(), d.to_quart());
                },
                "Gallon" => {
                    println!("{} deciliters is equal to {} gallons", d.get_value(), d.to_gallon());
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

fn deciliter_metric(d: &Deciliter) {
    let options = vec![
        "Centiliter",
        "Deciliter",
        "Liter",
        "Milliliter",
    ];

    let deciliter_select = Select::new("Select a conversion: ", options).prompt();
    match deciliter_select {
        Ok(deciliter_select) => {
            match deciliter_select {
                "Centiliter" => {
                    println!("{} deciliters is equal to {} centiliters", d.get_value(), d.to_centiliters());
                },
                "Deciliter" => {
                    println!("{} deciliters is equal to {} deciliters", d.get_value(), d.to_deciliters());
                },
                "Liter" => {
                    println!("{} deciliters is equal to {} liters", d.get_value(), d.to_liters());
                },
                "Milliliter" => {
                    println!("{} deciliters is equal to {} milliliters", d.get_value(), d.to_milliliters());
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

fn switch_deciliter() {
    let deciliter_options = vec![
        "Imperial",
        "Metric",
    ];

    let deciliter_select = Select::new("Select a unit type: ", deciliter_options).prompt();
    match deciliter_select {
        Ok(deciliter_select) => {
            match deciliter_select {
                "Imperial" => {
                    let deciliter = deciliter_factory();
                    deciliter_imperial(&deciliter);
                },
                "Metric" => {
                    let deciliter = deciliter_factory();
                    deciliter_metric(&deciliter);
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

pub fn convert_deciliter() {
    let deciliter_loop = true;
    while deciliter_loop {
        switch_deciliter();
        let deciliter_opt = vec!["Yes", "No"];
        let deciliter_continue = Select::new("Would you like to convert another deciliter?", deciliter_opt).prompt();
        match deciliter_continue.unwrap() {
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
    }
}