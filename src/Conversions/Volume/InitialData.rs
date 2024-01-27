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
    let volume_select = Select::new("Select a volume type: ", vec!["Milliliter", "Liter", "Centiliter", "Deciliter"]).prompt();
}

pub fn volume_entry_point() {
    // Flag to keep the program running;
    let mut prog_running: bool = true;
    // Keep running the program until the user decides to stop
    while prog_running {
        // Get the volume type from the user
        let volume_type = Select::new("What unit type are you converting from?", vec!["Imperial", "Metric"]).prompt();
        match volume_type {
            Ok(ans) => {
                // Route the user to the appropriate volume type
                match ans {
                    "Imperial" => {
                        get_imperial();
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
let continue_loop = Select::new("Would you like to get another volume? (y/n) ", vec!["y", "n"]).prompt();
match continue_loop {
    Ok(ans) => {
        match ans {
            "y" => {
                prog_running = true;
            },
            "n" => {
                prog_running = false;
            },
            &_ => {
                println!("Error");
            }
        }
    },
    Err(_) => {
        println!("Error");
}
}}}