use inquire::{Select, Text};

#[derive(Default)]
struct Milliliter {
    value: f64,
}

impl Milliliter {
    fn new(value: f64) -> Milliliter {
        Milliliter {
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
        self.value
    }
}

fn milligram_factory() -> Milliliter {
    let milligram = Text::new("Enter the number of milligrams you wish to convert to imperial units").prompt();
    match milligram {
        Ok(mg) => {
            let mg: f64 = mg.parse().unwrap();
            let milligram = Milliliter::new(mg);
            milligram
        },
        Err(_) => {
            println!("Error");
            Milliliter::default()
        }
    }

}

fn milligram_imperial(m: &Milliliter) {
    let m_options = Select::new("Select an imperial unit", vec!["Teaspoon", "Tablespoon", "Fluid Ounce", "Gill", "Cup", "Pint", "Quart", "Gallon"]).prompt();
    match m_options {
        Ok(m_opt) => {
            match m_opt {
                "Teaspoon" => {
                    println!("{} milligrams is {} teaspoons", m.get_value(), m.to_teaspoon());
                },
                "Tablespoon" => {
                    println!("{} milligrams is {} tablespoons", m.get_value(), m.to_tablespoon());
                },
                "Fluid Ounce" => {
                    println!("{} milligrams is {} fluid ounces", m.get_value(), m.to_fluid_ounce());
                },
                "Gill" => {
                    println!("{} milligrams is {} gills", m.get_value(), m.to_gill());
                },
                "Cup" => {
                    println!("{} milligrams is {} cups", m.get_value(), m.to_cup());
                },
                "Pint" => {
                    println!("{} milligrams is {} pints", m.get_value(), m.to_pint());
                },
                "Quart" => {
                    println!("{} milligrams is {} quarts", m.get_value(), m.to_quart());
                },
                "Gallon" => {
                    println!("{} milligrams is {} gallons", m.get_value(), m.to_gallon());
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

fn milligram_metric(m: &Milliliter) {
    let m_options = Select::new("Select a metric unit", vec!["Centiliter", "Deciliter", "Liter", "Milliliter"]).prompt();
    match m_options {
        Ok(m_opt) => {
            match m_opt {
                "Centiliter" => {
                    println!("{} milligrams is {} centiliters", m.get_value(), m.to_centiliters());
                },
                "Deciliter" => {
                    println!("{} milligrams is {} deciliters", m.get_value(), m.to_deciliters());
                },
                "Liter" => {
                    println!("{} milligrams is {} liters", m.get_value(), m.to_liters());
                },
                "Milliliter" => {
                    println!("{} milligrams is {} milliliters", m.get_value(), m.to_milliliters());
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

fn milligram_switch() {
    let milligram = milligram_factory();
    let unit_type = Select::new("What unit type are you converting from?", vec!["Imperial", "Metric"]).prompt();
    match unit_type {
        Ok(unit) => {
            match unit {
                "Imperial" => {
                    milligram_imperial(&milligram);
                },
                "Metric" => {
                    milligram_metric(&milligram);
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

pub fn convert_milligram() {
    let mill: bool = true;
    while mill {
        milligram_switch();
        let milligram = Select::new("Would you like to convert another unit?", vec!["Yes", "No"]).prompt();
        match milligram {
            Ok(mg) => {
                match mg {
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