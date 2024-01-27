use inquire::{Select, Text, error::InquireResult};

pub struct Kilometer {
    value: f64,
}

impl Kilometer {
    pub fn new(value: f64) -> Self {
        Self {
            value,
        }
    }

    pub fn to_millimeters(&self) {
        println!("{}", self.value * 1000000.0);
    }

    pub fn to_meters(&self) {
        println!("{}", self.value * 1000.0);
    }

    pub fn to_decimeters(&self) {
        println!("{}", self.value * 10000.0);
    }

    pub fn to_centimeters(&self) {
        println!("{}", self.value * 100000.0);
    }

    pub fn to_chain(&self) {
        println!("{}", self.value * 49.71);
    }

    pub fn to_furlong(&self) {
        println!("{}", self.value * 4.971);
    }

    pub fn to_inch(&self) {
        println!("{}", self.value * 39370.079);
    }

    pub fn to_feet(&self) {
        println!("{}", self.value * 3280.84);
    }

    pub fn to_yard(&self) {
        println!("{}", self.value * 1093.613);
    }

    pub fn to_mile(&self) {
        println!("{}", self.value / 1.609);
    }
}

fn kilo_imperial() {
    let kilo = Text::new("How many kilometers are you converting? ").prompt();
    match kilo {
        Ok(kilo_val) => {
            match kilo_val.parse::<f64>() {
                Ok(kilo_float) => {
                    let kilo = Kilometer::new(kilo_float);
                    let kilo_conv = Select::new("What would you like to convert to? ", 
                    vec!["Chain", "Furlong", "Inches", "Feet", "Yards", "Miles"]).prompt();
                    match kilo_conv {
                        Ok(kilo_conv_val) => {
                            match kilo_conv_val {
                                "Chain" => {
                                    kilo.to_chain();
                                },
                                "Furlong" => {
                                    kilo.to_furlong();
                                },
                                "Inches" => {
                                    kilo.to_inch();
                                },
                                "Feet" => {
                                    kilo.to_feet();
                                },
                                "Yards" => {
                                    kilo.to_yard();
                                },
                                "Miles" => {
                                    kilo.to_mile();
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

fn kilo_metric() {
    let kilo = Text::new("How many kilometers are you converting? ").prompt();
    match kilo {
        Ok(kilo_val) => {
            match kilo_val.parse::<f64>() {
                Ok(kilo_float) => {
                    let kilo = Kilometer::new(kilo_float);
                    let kilo_conv = Select::new("What would you like to convert to? ", 
                    vec!["Millimeters", "Meters", "Decimeters", "Centimeters"]).prompt();
                    match kilo_conv {
                        Ok(kilo_conv_val) => {
                            match kilo_conv_val {
                                "Millimeters" => {
                                    kilo.to_millimeters();
                                },
                                "Meters" => {
                                    kilo.to_meters();
                                },
                                "Decimeters" => {
                                    kilo.to_decimeters();
                                },
                                "Centimeters" => {
                                    kilo.to_centimeters();
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

fn kilo_switch() {
    let kilo_type= Select::new("What would you like to convert to? ", vec!["Imperial", "Metric"]).prompt();
    match kilo_type {
        Ok(kilo_type_val) => {
            match kilo_type_val {
                "Imperial" => {
                    kilo_imperial();
                },
                "Metric" => {
                    kilo_metric();
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

pub fn convert_kilometer() {
    let convert_loop: bool = true;
    while convert_loop {
        kilo_switch();
        let choice = Select::new("Convert another kilo?", vec!["Yes", "No"]).prompt();
        match choice {
            Ok(chosen) => {
                match chosen {
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