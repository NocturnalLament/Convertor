use inquire::{Select, Text};

#[derive(Default)]
struct Gram {
    value: f64,
}

impl Gram {
    fn new(value: f64) -> Gram {
        Gram {
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
        self.value * 0.001
    }

    fn to_milliliters(&self) -> f64 {
        self.value * 1.0
    }
}

fn prompt_for_gram() -> Gram {
    let mut gram = Gram::default();
    let value_str = Text::new("Enter the value of Grams: ").prompt();
    //gram.set_value(value);
    match value_str {
        Ok(value) => {
            match value.parse::<f64>() {
                Ok(value_f) => {
                    gram.set_value(value_f);
                },
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    gram
}

fn gram_imperial(g: &Gram) {
    let options = vec!["Teaspoon", "Tablespoon", "Fluid Ounce", "Gill", "Cup", "Pint", "Quart", "Gallon"];
    let imp_type = Select::new("Select the imperial unit to convert to: ", options).prompt();
    match imp_type {
        Ok(imp_sel) => {
            match imp_sel {
                "Teaspoon" => {
                    println!("{} Grams is equal to {} Teaspoons", g.get_value(), g.to_teaspoon());
                },
                "Tablespoon" => {
                    println!("{} Grams is equal to {} Tablespoons", g.get_value(), g.to_tablespoon());
                },
                "Fluid Ounce" => {
                    println!("{} Grams is equal to {} Fluid Ounces", g.get_value(), g.to_fluid_ounce());
                },
                "Gill" => {
                    println!("{} Grams is equal to {} Gills", g.get_value(), g.to_gill());
                },
                "Cup" => {
                    println!("{} Grams is equal to {} Cups", g.get_value(), g.to_cup());
                },
                "Pint" => {
                    println!("{} Grams is equal to {} Pints", g.get_value(), g.to_pint());
                },
                "Quart" => {
                    println!("{} Grams is equal to {} Quarts", g.get_value(), g.to_quart());
                },
                "Gallon" => {
                    println!("{} Grams is equal to {} Gallons", g.get_value(), g.to_gallon());
                },
                &_ => {
                    println!("Error");
                }
            }
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn gram_metric(g: &Gram) {
    let options = vec!["Centiliters", "Deciliters", "Liters", "Milliliters"];
    let met_type = Select::new("Select the metric unit to convert to: ", options).prompt();
    match met_type {
        Ok(met_sel) => {
            match met_sel {
                "Centiliters" => {
                    println!("{} Grams is equal to {} Centiliters", g.get_value(), g.to_centiliters());
                },
                "Deciliters" => {
                    println!("{} Grams is equal to {} Deciliters", g.get_value(), g.to_deciliters());
                },
                "Liters" => {
                    println!("{} Grams is equal to {} Liters", g.get_value(), g.to_liters());
                },
                "Milliliters" => {
                    println!("{} Grams is equal to {} Milliliters", g.get_value(), g.to_milliliters());
                },
                &_ => {
                    println!("Error");
                }
            }
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn gram_switch() {
    let options = vec!["Imperial", "Metric"];
    let unit_type = Select::new("Select the unit type to convert from: ", options).prompt();
    match unit_type {
        Ok(unit_sel) => {
            match unit_sel {
                "Imperial" => {
                    let g = prompt_for_gram();
                    gram_imperial(&g);
                },
                "Metric" => {
                    let g = prompt_for_gram();
                    gram_metric(&g);
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

pub fn convert_gram() {
    let convert_loop = true;
    while convert_loop {
        gram_switch();
        let options = vec!["Yes", "No"];
        let continue_loop = Select::new("Do you want to convert another Gram?", options).prompt();
        match continue_loop {
            Ok(ans) => {
                match ans {
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

