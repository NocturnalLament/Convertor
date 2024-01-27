use inquire::{Select, Text};

struct Centimeters {
    value: f64,
}

impl Centimeters {
    pub fn new(value: f64) -> Self {
        Self {
            value,
        }
    }

    pub fn to_millimeters(&self) {
        println!("{}", self.value * 10.0);
    }

    pub fn to_meters(&self) {
        println!("{}", self.value / 100.0);
    }

    pub fn to_kilometers(&self) {
        println!("{}", self.value / 100000.0);
    }

    pub fn to_decimeters(&self) {
        println!("{}", self.value / 10.0);
    }

    pub fn to_chain(&self) {
        println!("{}", self.value / 2012.8);
    }

    pub fn to_furlong(&self) {
        println!("{}", self.value / 20120.0);
    }

    pub fn to_inch(&self) {
        println!("{}", self.value / 2.54);
    }

    pub fn to_feet(&self) {
        println!("{}", self.value / 30.48);
    }

    pub fn to_yard(&self) {
        println!("{}", self.value / 91.44);
    }

    pub fn to_mile(&self) {
        println!("{}", self.value / 160934.4);
    }

    
}

fn convert_centimeters_imperial() {
    let cent = Text::new("How many centimeters are you converting? ").prompt();
    match cent {
        Ok(cent_val) => {
            match cent_val.parse::<f64>() {
                Ok(cent_val) => {
                    let cent = Centimeters::new(cent_val);
                    let choice = Select::new("Select what you want to convert to: ", vec!["Chain", "Furlong", "Inch", "Feet", "Yard", "Mile"]).prompt();
                    match choice {
                        Ok(choice) => {
                            match choice {
                                "Chain" => {
                                    cent.to_chain();
                                },
                                "Furlong" => {
                                    cent.to_furlong();
                                },
                                "Inch" => {
                                    cent.to_inch();
                                },
                                "Feet" => {
                                    cent.to_feet();
                                },
                                "Yard" => {
                                    cent.to_yard();
                                },
                                "Mile" => {
                                    cent.to_mile();
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

fn convert_centimeters_metric() {
    let cent = Text::new("How many centimeters are you converting? ").prompt();
    match cent {
        Ok(cent_val) => {
            match cent_val.parse::<f64>() {
                Ok(cent_val) => {
                    let cent = Centimeters::new(cent_val);
                    let choice = Select::new("Select what you want to convert to: ", vec!["Millimeter", "Meter", "Kilometer", "Decimeter"]).prompt();
                    match choice {
                        Ok(choice) => {
                            match choice {
                                "Millimeter" => {
                                    cent.to_millimeters();
                                },
                                "Meter" => {
                                    cent.to_meters();
                                },
                                "Kilometer" => {
                                    cent.to_kilometers();
                                },
                                "Decimeter" => {
                                    cent.to_decimeters();
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

fn convert_centimeters_main() {
    let choice = Select::new("Select what conv type", vec!["Imperial", "Metric"]).prompt();
    match choice {
        Ok(choice) => {
            match choice {
                "Imperial" => {
                    convert_centimeters_imperial();
                },
                "Metric" => {
                    convert_centimeters_metric();
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

pub fn convert_centimeters() {
    let centimeter_loop = true;
    while centimeter_loop {
        convert_centimeters_main();
        let choice = Select::new("Do you want to another celsius? ", vec!["Yes", "No"]).prompt();
        match choice {
            Ok(choice) => {
                match choice {
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

