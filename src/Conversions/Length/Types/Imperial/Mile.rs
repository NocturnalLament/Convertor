use inquire::{Select, Text};

struct Mile {
    value: f64,
}

impl Mile {
    fn new(value: f64) -> Self {
        Self {
            value,
        }
    }

    fn show_centimeters(&self) -> f64 {
        self.value * 160934.0
    }

    fn show_meters(&self) -> f64 {
        self.value * 1609.34
    }

    fn show_kilometers(&self) -> f64 {
        self.value * 1.60934
    }

    fn show_miles(&self) -> f64 {
        self.value
    }

    fn show_yards(&self) -> f64 {
        self.value * 1760.0
    }

    fn show_feet(&self) -> f64 {
        self.value * 5280.0
    }

    fn show_millimeters(&self) -> f64 {
        self.value * 1609340.0
    }

    fn show_inches(&self) -> f64 {
        self.value * 63360.0
    }

    fn show_furlong(&self) -> f64 {
        self.value * 8.0
    }

    fn show_chain(&self) -> f64 {
        self.value * 80.0
    }

    fn show_decimeters(&self) -> f64 {
        self.value * 16093.4
    }
}
fn convert_imperial() {
    let amount = Text::new("How many miles are you converting? ").prompt();
    match amount {
        Ok(amount_val) => {
            let amount_val = amount_val.parse::<f64>().unwrap();
            let mile = Mile::new(amount_val);
            let choice = Select::new("What are you converting to?", vec!["Miles", "Yards", "Feet", "Inches", "Furlong", "Chain"]).prompt();
            match choice {
                Ok(choice_val) => {
                    match choice_val {
                        
                        "Yards" => {
                            println!("{}", mile.show_yards());
                        }
                        "Feet" => {
                            println!("{}", mile.show_feet());
                        }
                        "Inches" => {
                            println!("{}", mile.show_inches());
                        },
                        "Furlong" => {
                            println!("{}", mile.show_furlong());
                        },
                        "Chain" => {
                            println!("{}", mile.show_chain());
                        },
                        _ => {
                            println!("Error");
                        }
                    }
                }
                Err(_) => {
                    println!("Error");
                }
            }
        }
        Err(_) => {
            println!("Error");
        }
    }

}

fn convert_metric() {
    let amount = Text::new("How many miles are you converting? ").prompt();
    match amount {
        Ok(am) => {
            match am.parse::<f64>() {
                Ok(am_val) => {
                    let mile = Mile::new(am_val);
                    let choice = Select::new("What are you converting to?", vec!["Centimeters", "Meters", "Kilometers"]).prompt();
                    match choice {
                        Ok(choice_val) => {
                            match choice_val {
                                "Centimeters" => {
                                    println!("{}", mile.show_centimeters());
                                },
                                "Meters" => {
                                    println!("{}", mile.show_meters());
                                },
                                "Kilometers" => {
                                    println!("{}", mile.show_kilometers());
                                },
                                "Decimeters" => {
                                    println!("{}", mile.show_decimeters());
                                },
                                "Millimeters" => {
                                    println!("{}", mile.show_millimeters());
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
}
fn convert_switch() {
    let mile_to = Select::new("Are you converting Miles to Imperial or Metric? ", vec!["Imperial", "Metric"]).prompt();
    match mile_to {
        Ok(mile) => {
            match mile {
                "Imperial" => {
                    convert_imperial();
                },
                "Metric" => {
                    convert_metric();
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

pub fn convert_miles() {
    let loop_running = true;
    while loop_running {
        
    }
}