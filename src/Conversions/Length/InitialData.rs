use inquire::{Text, Select};
#[path = "Types/Imperial/Furlong.rs"] mod Furlong;
pub fn get_length_type() {
    /* Actually Getting Whether or not user wants to convert from Metric or Imperial unlike Temperature
    Because of how many different types of units there are for length it makes more sense to have this extra step 
    */
    let mut SystemsOfMeasurement = Select::new("Do you use metric or imperial? ", vec!["metric", "imperial"]).prompt();
    match SystemsOfMeasurement {
        Ok(system) => {
            match system {
                "metric" => {
                    let test = vec!["millimeters", "centimeters", "meters", "kilometers"];
                    let chosen_item = Select::new("Select a unit: ", test).prompt();
                    match chosen_item {
                        Ok(item) => {
                            match item {
                                "millimeters" => {
                                    convert_millimeters();
                                },
                                "centimeters" => {
                                    println!("centimeters");
                                },
                                "meters" => {
                                    println!("meters");
                                },
                                "kilometers" => {
                                    println!("kilometers");
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
                "imperial" => {
                    let chosen_item = Select::new("Select a unit: ", vec!["Furlong","Chains","inches", "feet", "yards", "miles"]).prompt();
                    match chosen_item {
                        Ok(item) => {
                            match item {
                                "Furlong" => {
                                    convert_furlong();
                                },
                                "Chains" => {
                                    println!("Chains");
                                },
                                "inches" => {
                                    convert_inch();
                                },
                                "feet" => {
                                    println!("feet");
                                },
                                "yards" => {
                                    println!("yards");
                                },
                                "miles" => {
                                    println!("miles");
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

fn convert_millimeters() {
    let mill_conv = Text::new("How many millimeters are you converting? ").prompt();
    match mill_conv {
        Ok(mill) => {
            match mill.parse::<f64>() {
                Ok(mill_as_f) => {
                    let mill_as_f = mill_as_f;
                    let mill = Millimeter::new(mill_as_f);
                    let convert_to = Select::new("What would you like to convert to? ", vec!["centimeters", "meters", "kilometers"]).prompt();
                    match convert_to {
                        Ok(convert_to) => {
                            match convert_to {
                                "centimeters" => {
                                    println!("{} millimeters is {} centimeters.", mill.value, mill.show_centimeters());
                                },
                                "meters" => {
                                    println!("{} millimeters is {} meters.", mill.value, mill.show_meters());
                                },
                                "kilometers" => {
                                    println!("{} millimeters is {} kilometers.", mill.value, mill.show_kilometers());
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
    // match mill_conv {
    //     Ok(mill) => {
    //         match mill.parse::<f64>() {
    //             Ok(mill_as_f) => {
    //                 let mill_as_f = mill_as_f;
    //                 let mill = Millimeter::new(mill_as_f);
    //                 mill.show_centimeters();
    //                 mill.show_meters();
    //                 mill.show_kilometers();
    //             },
    //             Err(_) => {
    //                 println!("Error");
    //             }
    //         }
    //     },
    //     Err(_) => {
    //         println!("Error");
    //     }
    // }
}

fn convert_furlong() {
    let mut furlong_loop_running: bool = true;
    while furlong_loop_running {
        Furlong::furlong_prompt();
        let mut continue_running = Select::new("Would you like to continue and convert to another amount? (y/n) ", vec!["y", "n"]).prompt();
        match continue_running {
            Ok(ans) => {
                match ans {
                    "y" => {
                        furlong_loop_running = true;
                    },
                    "n" => {
                        furlong_loop_running = false;
                    },
                    &_ => todo!(),
                }
            },
            Err(_) => {
                println!("Error")
            }
        }
    }
}

fn convert_inch() {
    let mut inch_loop_running: bool = true;
    while inch_loop_running {
        println!("test");
        let mut continue_running = Select::new("Would you like to continue and convert to another amount? (y/n) ", vec!["y", "n"]).prompt();
        match continue_running {
            Ok(ans) => {
                match ans {
                    "y" => {
                        inch_loop_running = true;
                    },
                    "n" => {
                        inch_loop_running = false;
                    },
                    &_ => todo!(),
                }
            },
            Err(_) => {
                println!("Error")
            }
        }
    }
}