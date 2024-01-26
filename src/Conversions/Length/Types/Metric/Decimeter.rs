use inquire::{Select, Text, error::InquireResult};

pub struct Decimeter {
    value: f64,
}

impl Decimeter {
    pub fn new(value: f64) -> Self {
        Self {
            value,
        }
    }

    pub fn to_millimeters(&self) {
        println!("{}", self.value * 10.0);
    }

    pub fn to_meters(&self) {
        println!("{}", self.value / 10.0);
    }

    pub fn to_kilometers(&self) {
        println!("{}", self.value / 10000.0);
    }

    pub fn to_centimeters(&self) {
        println!("{}", self.value * 10.0);
    }

    pub fn to_chain(&self) {
        println!("{}", self.value / 201.168);
    }

    pub fn to_furlong(&self) {
        println!("{}", self.value / 2011.68);
    }

    pub fn to_inch(&self) {
        println!("{}", self.value * 3.937);
    }

    pub fn to_feet(&self) {
        println!("{}", self.value / 3.048);
    }

    pub fn to_yard(&self) {
        println!("{}", self.value / 9.144);
    }

    pub fn to_mile(&self) {
        println!("{}", self.value / 16093.44);
    }
}

fn convert_decimeter_imperial() {
    let dec = Text::new("How many decimeters are you converting? ").prompt();
    match dec {
        Ok(dec_val) => {
            match dec_val.parse::<f64>() {
                Ok(dec_float) => {
                    let dec = Decimeter::new(dec_float);
                    let choice = Select::new("Convert to: ", vec!["Inches", "Feet", "Yards", "Miles"]).prompt();
                    match choice {
                        Ok(choice_val) => {
                            match choice_val {
                                "Inches" => {
                                    dec.to_inch();
                                },
                                "Feet" => {
                                    dec.to_feet();
                                },
                                "Yards" => {
                                    dec.to_yard();
                                },
                                "Miles" => {
                                    dec.to_mile();
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

fn metric_switch() {
    let decimeter_am: InquireResult<String> = Text::new("How many decimeters are you converting? ").prompt();
    match decimeter_am {
        Ok(dec) => {
            match dec.parse::<f64>() {
                Ok(dec_float) => {
                    let dec = Decimeter::new(dec_float);
                    let choice = Select::new("Convert to: ", vec!["Millimeters", "Meters", "Kilometers", "Centimeters"]).prompt();
                    match choice {
                        Ok(choice_val) => {
                            match choice_val{
                                "Millimeters" => {
                                    dec.to_millimeters();
                                },
                                "Meters" => {
                                    dec.to_meters();
                                },
                                "Kilometers" => {
                                    dec.to_kilometers();
                                },
                                "Centimeters" => {
                                    dec.to_centimeters();
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

fn conversion_switch() {
    let choice = Select::new("Convert to: ", vec!["Imperial", "Metric"]).prompt();
    match choice {
        Ok(choice_val) => {
            match choice_val {
                "Imperial" => {
                    convert_decimeter_imperial();
                },
                "Metric" => {
                    metric_switch();
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

pub fn convert_decimeter() {
    let loop_bool: bool = true;
    while loop_bool {
        conversion_switch();
        let decision = Select::new("Convert to another decimeter? ", vec!["Yes", "No"]).prompt().unwrap();
        match decision {
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