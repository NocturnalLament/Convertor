use inquire::{InquireError, Select, Text};

pub struct Millimeter {
    pub value: f64
}

impl Millimeter {
    pub fn new(value: f64) -> Millimeter {
        Millimeter {
            value
        }
    }

    pub fn show_centimeters(&self) {
        println!("{}", self.value / 10.0);
    }

    pub fn show_meters(&self) {
        println!("{}", self.value / 1000.0);
    }

    pub fn show_kilometers(&self) {
        println!("{}", self.value / 1000000.0);
    }

    pub fn show_decimeter(&self) {
        println!("{}", self.value / 100.0);
    }

    pub fn show_chain(&self) {
        println!("{}", self.value / 201168.0);
    }

    pub fn show_furlong(&self) {
        println!("{}", self.value / 2011680.0);
    }

    pub fn show_inch(&self) {
        println!("{}", self.value / 25.4);
    }

    pub fn show_feet(&self) {
        println!("{}", self.value / 304.8);
    }

    pub fn show_yard(&self) {
        println!("{}", self.value / 914.4);
    }

    pub fn show_mile(&self) {
        println!("{}", self.value / 1609344.0);
    }
}

fn convert_metric() {
    //Get how many millimeters are to be converted.
    let mill = Text::new("How many millimeters are you converting? ").prompt();
    //Unwraps the value of mill and converts it to a f64.
    match mill {
            Ok(mill_val) => {
                match mill_val.parse::<f64>() {
                    //If parse is successful, create new Millimeter object.
                    Ok(mill_as_f) => {
                        let mill_as_f = mill_as_f;
                        let mill: Millimeter = Millimeter::new(mill_as_f);
                        //Get what unit the user wants to convert to.
                        let chosen_item = Select::new("Select a unit: ", vec!["centimeters", "meters", "kilometers", "decimeters"]).prompt();
                        //Unwrap the value of chosen_item and convert it to a &str.
                        match chosen_item {
                            Ok(item) => {
                                //Match the &str to the correct unit.
                                match item {
                                    //Call the correct function to convert the millimeters to the correct unit.
                                    "centimeters" => {
                                        mill.show_centimeters();
                                    },
                                    "meters" => {
                                        mill.show_meters();
                                    },
                                    "kilometers" => {
                                        mill.show_kilometers();
                                    },
                                    "decimeters" => {
                                        mill.show_decimeter();
                                    },
                                    _ => {
                                        println!("Error");
                                    }
                                }
                            },
                            //If the user doesn't select a unit, print error.
                            Err(_) => {
                                println!("Error");
                            }
                        }
                    },
                    //If parse is unsuccessful, print error.
                    Err(_) => {
                        println!("Error");
                    }
                }
            },
            //If the user doesn't enter a value, print error.
            Err(_) => {
                println!("Error");
            }
        }
}

fn convert_imperial() {
    let mill: Result<String, InquireError> = Text::new("How many millimeters are you converting? ").prompt();
    match mill {
        Ok(millimeter) => {
            match millimeter.parse::<f64>() {
                Ok(mill_as_f) => {
                    let mill_as_f = mill_as_f;
                    let mill: Millimeter = Millimeter::new(mill_as_f);
                    let chosen_item = Select::new("Select a unit: ", vec!["Furlong","Chains","inches", "feet", "yards", "miles"]).prompt();
                    match chosen_item {
                        Ok(item) => {
                            match item {
                                "Furlong" => {
                                    mill.show_furlong();
                                },
                                "Chains" => {
                                    mill.show_chain();
                                },
                                "inches" => {
                                    mill.show_inch();
                                },
                                "feet" => {
                                    mill.show_feet();
                                },
                                "yards" => {
                                    mill.show_yard();
                                },
                                "miles" => {
                                    mill.show_mile();
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

fn convert_metric_or_imperial() {
    let convert_to = Select::new("What would you like to convert to? ", vec!["metric", "imperial"]).prompt();
    match convert_to {
        Ok(convert_to) => {
            match convert_to {
                "metric" => {
                    convert_metric();
                },
                "imperial" => {
                    convert_imperial();
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

pub fn convert_millimeters() {
    let loop_running = true;
    while loop_running {
        convert_metric_or_imperial();
        let continue_loop = Select::new("Would you like to convert another millimeter value? ", vec!["Yes", "No"]).prompt();
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
            Err(_) => {
                println!("Error");
            }
        }
    }
}