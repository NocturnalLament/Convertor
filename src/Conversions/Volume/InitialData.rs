use std::path;

use inquire::{Select};
#[path = "Types/Imperial/Teaspoon.rs"] mod Teaspoon;
#[path = "Types/Imperial/Tablespoon.rs"] mod Tablespoon;
#[path = "Types/Imperial/Quart.rs"] mod Quart;
#[path = "Types/Imperial/Pint.rs"] mod Pint;
#[path = "Types/Imperial/Gill.rs"] mod Gill;
#[path = "Types/Imperial/Gallon.rs"] mod Gallon;
#[path = "Types/Imperial/FlOz.rs"] mod FlOz;
#[path = "Types/Imperial/Cup.rs"] mod Cup;
#[path = "Types/Metric/Milliliter.rs"] mod Milliliter;
#[path = "Types/Metric/Milligram.rs"] mod Milligram;
#[path = "Types/Metric/Liter.rs"] mod Liter;
#[path = "Types/Metric/Gram.rs"] mod Gram;
#[path = "Types/Metric/Deciliter.rs"] mod Deciliter;
#[path = "Types/Metric/Centiliter.rs"] mod Centiliter;
// Function to get the type of volume

fn get_imperial() {
    // Prompt the user to select a volume type
    let volume_select = Select::new("Select a volume type: ", vec!["Teaspoon", "Tablespoon", "Fluid Ounce", "Cup", "Pint", "Quart", "Gallon", "Gill"]).prompt();
    // Return the selected volume type or the error if there is one
    match volume_select {
        //If the user selects a volume type, match the volume type
        Ok(volume) => {
            match volume {
                "Teaspoon" => {
                    Teaspoon::convert_teaspoon();
                },
                "Tablespoon" => {
                    Tablespoon::convert_tablespoon();
                },
                "Quart" => {
                    Quart::convert_quart();
                },
                "Pint" => {
                    Pint::convert_pint();
                },
                "Gill" => {
                    Gill::gill_entry_point();
                },
                "Gallon" => {
                    Gallon::convert_gallon();
                },
                "Fluid Ounce" => {
                    FlOz::convert_fl_oz();
                },
                "Cup" => {
                    Cup::convert_cup();
                },
                &_ => {
                    println!("Error");
                
                }
            }
            
        },
        Err(e) => {println!("Error: {}", e)},
    }
}

fn get_metric() {
    // Prompt the user to select a volume type
    let volume_select = Select::new("Select a volume type: ", vec!["Milliliter", "Milligram", "Liter", "Centiliter", "Deciliter"]).prompt();
    match volume_select {
        Ok(volume) => {
            match volume {
                "Milliliter" => {
                    Milliliter::convert_milliliter();
                },
                "Milligram" => {
                    Milligram::convert_milligram();
                },
                "Liter" => {
                    Liter::convert_liter();
                },
                "Gram" => {
                    Gram::convert_gram();
                }
                "Centiliter" => {
                    Centiliter::convert_centiliter();
                },
                "Deciliter" => {
                    Deciliter::convert_deciliter();
                },
                &_ => {
                    println!("Error");
                }
            }
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    
    }
}

pub fn volume_entry_point() {
    // Flag to keep the program running;
    let mut loop_flag = true;
    while loop_flag {
        // Prompt the user to select a volume type
        let volume_select = Select::new("Select a volume type: ", vec!["Imperial", "Metric"]).prompt();
        // Return the selected volume type or the error if there is one
        match volume_select {
            //If the user selects a volume type, match the volume type
            Ok(volume) => {
                match volume {
                    "Imperial" => {
                        get_imperial();
                    },
                    "Metric" => {
                        get_metric();
                    },
                    &_ => {
                        println!("Error");
                    }
                }
            },
            Err(e) => {println!("Error: {}", e)},
        }
        let continue_loop = Select::new("Would you like to get another volume? (y/n) ", vec!["y", "n"]).prompt();
        match continue_loop {
            Ok(ans) => {
                match ans {
                    "y" => {
                        continue;
                    },
                    "n" => {
                        loop_flag = false;
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