use inquire::{Select, Text};

struct Inch {
    value: f64,
}

impl Inch {
    fn new(value: f64) -> Self {
        Self {
            value,
        }
    }

    fn show_furlong(&self) -> f64 {
        self.value * 0.000015783
    }

    fn show_centimeters(&self) -> f64 {
        self.value * 2.54
    }

    fn show_decimeters(&self) -> f64 {
        self.value * 0.254
    }

    fn show_meters(&self) -> f64 {
        self.value * 0.0254
    }

    fn show_kilometers(&self) -> f64 {
        self.value * 0.0000254
    }

    fn show_miles(&self) -> f64 {
        self.value * 0.000015783
    }

    fn show_yards(&self) -> f64 {
        self.value * 0.027778
    }

    fn show_feet(&self) -> f64 {
        self.value * 0.083333
    }

    fn show_millimeters(&self) -> f64 {
        self.value * 25.4
    }

    fn show_inches(&self) -> f64 {
        self.value
    }
}



fn inch_imperial(inch: &Inch) {
    
    let inch_option = Select::new("What would you like to convert to? ", vec!["Miles", "Yards", "Feet", "Furlong", "Inches"]).prompt();
    match inch_option {
        Ok(inches) => {
            match inches {
                "Miles" => {
                    println!("{}", inch.show_miles());
                },
                "Yards" => {
                    println!("{}", inch.show_yards());
                },
                "Feet" => {
                    println!("{}", inch.show_feet());
                },
                "Furlong" => {
                    println!("{}", inch.show_furlong());
                },
                "Inches" => {
                    println!("{}", inch.show_inches());
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

fn inch_metric(inch: &Inch) {
    let inch_option = Select::new("What would you like to convert to? ", vec!["Centimeters", "Meters", "Kilometers", "Millimeters", "Decimeter"]).prompt();
    match inch_option {
        Ok(inches) => {
            match inches {
                "Centimeters" => {
                    println!("{}", inch.show_centimeters());
                },
                "Meters" => {
                    println!("{}", inch.show_meters());
                },
                "Kilometers" => {
                    println!("{}", inch.show_kilometers());
                },
                "Millimeters" => {
                    println!("{}", inch.show_millimeters());
                },
                "Decimeter" => {
                    println!("{}", inch.show_decimeters());
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



fn inch_switch() {
    let amount = Text::new("How many inches are you converting? ").prompt();
    match amount {
        Ok(amount) => {
            match amount.parse::<f64>() {
                Ok(amount) => {
                    let inch = Inch::new(amount);
                    let inch_option = Select::new("Imperial or Metric? ", vec!["Imperial", "Metric"]).prompt();
                    match inch_option {
                        Ok(inches) => {
                            match inches {
                                "Imperial" => {
                                    inch_imperial(&inch);
                                },
                                "Metric" => {
                                    inch_metric(&inch);
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

pub fn inch_loop() {
    let loop_running: bool = true;
    while loop_running {
        inch_switch();
        let continue_loop = Select::new("Would you like to convert another inch value? ", vec!["Yes", "No"]).prompt();
        match continue_loop {
            Ok(continue_loop) => {
                match continue_loop {
                    "Yes" => {
                        inch_loop();
                    },
                    "No" => {
                        println!("Exiting...");
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