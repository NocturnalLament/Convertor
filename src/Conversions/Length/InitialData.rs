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
                                    let mill_conv = Text::new("How many millimeters are you converting? ").prompt();
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
                                    let mut furlong_loop_running: bool = true;
                                                    while furlong_loop_running {
                                                        Furlong::furlong_prompt()
                                                        let mut continue_running = Select::new("Would you like to continue and convert another furlong amount? (y/n) ", vec!["y", "n"]).prompt();
                                                    }
                                            }
                                        },
                                        Err(_) => {
                                            println!("Error");
                                        }
                                    }
                                },
                                "Chains" => {
                                    println!("Chains");
                                },
                                "inches" => {
                                    println!("inches");
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

