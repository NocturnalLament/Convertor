use inquire::{Text, Select};

struct Chain {
    value: f64,
}

impl Chain {
    fn new(value: f64) -> Self {
        Self {
            value,
        }
    }

    fn show_chain(&self) -> f64 {
        self.value
    }

    fn show_feet(&self)  {
        let feet_val = self.value * 66.0;
        println!("{}", feet_val);
    }

    fn show_furlong(&self)  {
        let furlong_val = self.value * 0.1;
        println!("{}", furlong_val);
    }

    fn show_inch(&self)  {
        let inch_val = self.value * 792.0;
        println!("{}", inch_val);
    }

    fn show_mile(&self) {
        let mile_val = self.value * 0.0125;
        println!("{}", mile_val);
    }

    fn show_yard(&self)  {
        let yard_val = self.value * 22.0;
        println!("{}", yard_val);

    }

    fn show_centimeters(&self)  {
        let centi_val = self.value * 20116.8;
        println!("{}", centi_val);
    }

    fn show_meters(&self)  {
        let meters_val = self.value * 20.1168;
        println!("{}", meters_val);
    }

    fn show_kilometers(&self)  {
        let kilometer_val = self.value * 0.0201168;
        println!("{}", kilometer_val);
    }

    fn show_millimeters(&self)  {
        let millimeter_val = self.value * 201168.0;
        println!("{}", millimeter_val);
    }

    fn show_decimeters(&self)  {
        let decimeter_val = self.value * 2011.68;
        println!("{}", decimeter_val);
    }
}

fn convert_metric() {
    let item = Text::new("How many chains are you converting? ").prompt();
    match item {
        Ok(item_val) => {
            match item_val.parse::<f64>() {
                Ok(item_val_f) => {
                    let chain = Chain::new(item_val_f);
                    let chain_option = Select::new("What would you like to convert to? ", vec!["Feet", "Furlong", "Inch", "Mile", "Yard", "Centimeter", "Meter", "Kilometer", "Millimeter", "Decimeter"]).prompt();
                    match chain_option {
                        Ok(chain_option) => {
                            match chain_option {
                                "Centimeters" => {
                                    chain.show_centimeters();
                                },
                                "Decimeter" => {
                                    chain.show_decimeters();
                                },
                                "Meters" => {
                                    chain.show_meters();
                                },
                                "Kilometers" => {
                                    chain.show_kilometers();
                                },
                                "Millimeters" => {
                                    chain.show_millimeters();
                                },
                                "Feet" => {
                                    chain.show_feet();
                                },
                                &_ => {
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

fn convert_imperial() {
    let item = Text::new("How many chains are you converting? ").prompt();
    match item {
        Ok(item_val) => {
            match item_val.parse::<f64>() {
                Ok(item_val_f) => {
                    let chain = Chain::new(item_val_f);
                    let chain_option = Select::new("What would you like to convert to? ", vec!["Feet", "Furlong", "Inch", "Mile", "Yard"]).prompt();
                    match chain_option {
                        Ok(chain_option) => {
                            match chain_option {
                                "Feet" => {
                                    chain.show_feet();
                                },
                                "Furlong" => {
                                    chain.show_furlong();
                                },
                                "Inch" => {
                                    chain.show_inch();
                                },
                                "Mile" => {
                                    chain.show_mile();
                                },
                                "Yard" => {
                                    chain.show_yard();
                                },
                                &_ => {
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
    let option = Select::new("Would you like to convert to imperial or metric? ", vec!["Metric", "Imperial"]).prompt().unwrap();
    match option {
        "Metric" => {
            convert_metric();
        },
        "Imperial" => {
            //convert_imperial();
            convert_imperial();
        },
        &_ => {
            println!("Error");
        }
    }
}

pub fn convert_chain() {
    let running = true;
    while running {
        convert_switch();
        let option = Select::new("Would you like to convert another chain?", vec!["Yes", "No"]).prompt();
        match option {
            Ok(choice) => {
                match choice {
                    "Yes" => {
                        continue;
                    },
                    "No" => {
                        break;
                    },
                    _ => {
                        println!("Unexpected choice");
                    }
                }
            },
            Err(_) => {
                println!("Error getting user input");
                break;
            }
        }
    }
}