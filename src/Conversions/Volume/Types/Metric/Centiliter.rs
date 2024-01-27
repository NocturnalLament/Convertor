use inquire::{Select, Text};

#[derive(Default)]
pub struct Centiliter {
    value: f64,
}

impl Centiliter {
    pub fn new(value: f64) -> Centiliter {
        Centiliter {
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
    pub fn to_milliliters(&self) -> f64 {
        self.value * 10.0
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

fn centiliter_factory() -> Centiliter {
    let mut centiliter = Centiliter::default();
    let mut centiliter_value = Text::new("Enter the centiliter value: ").prompt();
    match centiliter_value {
        Ok(value) => {
            match value.parse::<f64>() {
                Ok(val) => {
                    centiliter.set_value(val);
                    centiliter
                },
                Err(_) => {
                    println!("Error: Invalid value");
                    centiliter_factory()
                }
            }
        },
        Err(_) => {
            println!("Error: Invalid value");
            centiliter_factory()
        }
    }
}

fn centiliter_imperial(c: &Centiliter) {
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

    let imp_type = Select::new("Select the imperial unit to convert to: ", options).prompt();
    match imp_type.unwrap() {
        "Teaspoon" => {
            println!("{} centiliters is equal to {} teaspoons", c.get_value(), c.to_teaspoon());
        },
        "Tablespoon" => {
            println!("{} centiliters is equal to {} tablespoons", c.get_value(), c.to_tablespoon());
        },
        "Fluid Ounce" => {
            println!("{} centiliters is equal to {} fluid ounces", c.get_value(), c.to_fluid_ounce());
        },
        "Gill" => {
            println!("{} centiliters is equal to {} gills", c.get_value(), c.to_gill());
        },
        "Cup" => {
            println!("{} centiliters is equal to {} cups", c.get_value(), c.to_cup());
        },
        "Pint" => {
            println!("{} centiliters is equal to {} pints", c.get_value(), c.to_pint());
        },
        "Quart" => {
            println!("{} centiliters is equal to {} quarts", c.get_value(), c.to_quart());
        },
        "Gallon" => {
            println!("{} centiliters is equal to {} gallons", c.get_value(), c.to_gallon());
        },
        _ => {
            println!("Error: Invalid option");
        }
    }
}

fn metric_centiliter(c: &Centiliter) {
    let options = vec![
        "Milliliter",
        "Deciliter",
        "Liter",
        "Centiliter",
    ];
    let metric_type = Select::new("Select the metric unit to convert to: ", options).prompt();
    match metric_type.unwrap() {
        "Milliliter" => {
            println!("{} centiliters is equal to {} milliliters", c.get_value(), c.to_milliliters());
        },
        "Deciliter" => {
            println!("{} centiliters is equal to {} deciliters", c.get_value(), c.to_deciliters());
        },
        "Liter" => {
            println!("{} centiliters is equal to {} liters", c.get_value(), c.to_liters());
        },
        "Centiliter" => {
            println!("{} centiliters is equal to {} centiliters", c.get_value(), c.to_centiliters());
        },
        _ => {
            println!("Error: Invalid option");
        }
    }
}

fn switch_centiliter() {
    let options = vec![
        "Imperial",
        "Metric",
    ];

    let unit_type = Select::new("Select the unit type to convert from: ", options).prompt().unwrap();
    match unit_type {
        "Imperial" => {
            let centiliter = centiliter_factory();
            centiliter_imperial(&centiliter);
        },
        "Metric" => {
            let centiliter = centiliter_factory();
            metric_centiliter(&centiliter);
        },
        _ => {
            println!("Error: Invalid option");
        }
    }
}

pub fn convert_centiliter() {
    let loop_running = true;
    while loop_running {
        switch_centiliter();
        let options = vec![
            "Yes",
            "No",
        ];
        let continue_loop = Select::new("Do you want to convert another centiliter?", options).prompt().unwrap();
        match continue_loop {
            "Yes" => {
                continue;
            },
            "No" => {
                break;
            },
            _ => {
                println!("Error: Invalid option");
            }
        }
    }
}

