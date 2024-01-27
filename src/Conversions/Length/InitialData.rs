use std::path;

use inquire::{Select};
#[path = "Types/Imperial/Furlong.rs"] mod Furlong;
#[path = "Types/Metric/Millimeter.rs"] mod Millimeter;
#[path = "Types/Metric/Centimeters.rs"] mod Centimeters;
#[path = "Types/Metric/Decimeter.rs"] mod Decimeters;
#[path = "Types/Metric/Kilometer.rs"] mod Kilometer;
#[path = "Types/Imperial/Chain.rs"] mod Chain;
#[path = "Types/Imperial/Inch.rs"] mod Inch;
#[path = "Types/Imperial/Feet.rs"] mod Feet;
#[path = "Types/Imperial/Mile.rs"] mod Mile;
#[path = "Types/Imperial/Yard.rs"] mod Yard;
#[path = "Types/Metric/Meters.rs"] mod Meter;
pub fn get_length_type() {
    /* Actually Getting Whether or not user wants to convert from Metric or Imperial unlike Temperature
    Because of how many different types of units there are for length it makes more sense to have this extra step 
    */
    let mut SystemsOfMeasurement = Select::new("Do you use metric or imperial? ", vec!["metric", "imperial"]).prompt();
    match SystemsOfMeasurement {
        Ok(system) => {
            match system {
                "metric" => {
                    let test = vec!["millimeters", "centimeters", "meters", "kilometers", "decimeters"];
                    let chosen_item = Select::new("Select a unit: ", test).prompt();
                    match chosen_item {
                        Ok(item) => {
                            match item {
                                "millimeters" => {
                                    Millimeter::convert_millimeters();
                                },
                                "centimeters" => {
                                    Centimeters::convert_centimeters();
                                },
                                "meters" => {
                                    Meter::convert_meter();
                                },
                                "kilometers" => {
                                    Kilometer::convert_kilometer();
                                },
                                "decimeters" => {
                                    Decimeters::convert_decimeter();
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
                                    Chain::convert_chain();
                                },
                                "inches" => {
                                    Inch::inch_loop();
                                },
                                "feet" => {
                                    Feet::convert_feet();
                                },
                                "yards" => {
                                    Yard::convert_yard();
                                },
                                "miles" => {
                                    Mile::convert_miles();
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

