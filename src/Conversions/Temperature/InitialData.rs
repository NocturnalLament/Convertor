// Importing necessary modules and dependencies
use inquire::{Select, InquireError};
#[path = "Fahrenheit.rs"] mod Fahrenheit;
#[path = "Celsius.rs"] mod Celsius;
#[path = "Kelvin.rs"] mod Kelvin;

// Function to get the type of temperature from the user
fn get_temp_type() -> Result<String, InquireError> {
    // Prompt the user to select a temperature type
    let temp_select = Select::new("Select a temperature type: ", vec!["Fahrenheit", "Celsius", "Kelvin"]).prompt();
    // Return the selected temperature type or the error if there is one
    match temp_select {
        Ok(temp) => Ok(temp.to_string()),
        Err(e) => Err(e),
    }
}

// Entry point for the temperature conversion program
pub fn temp_entry_point() {
    // Flag to keep the program running
    let mut prog_running: bool = true;
    // Keep running the program until the user decides to stop
    while prog_running {
        // Get the temperature type from the user
        match get_data_type() {
            Ok(_) => {
                // Ask the user if they want to continue running the program
                let continue_running = Select::new("Would you like to get another temperature? (y/n) ", vec!["y", "n"]).prompt();
                match continue_running {
                    Ok(ans) => {
                        // Stop the program if the user answers 'n'
                        if ans == "n" {
                            prog_running = false;
                        }
                    },
                    Err(_) => {
                        // Print an error message and stop the program if there is an error getting user input
                        println!("An error occurred while getting user input");
                        prog_running = false;
                    }
                }
            },
            Err(_) => {
                // Print an error message and stop the program if there is an error getting the temperature type
                println!("An error occurred while getting temperature type");
                prog_running = false;
            }
        }
    }
}

// Function to handle Fahrenheit conversions
fn fahrenheit_loop() {
    // Flag to keep the Fahrenheit conversion running
    let mut fahrenheit_running: bool = true;
    // Keep running the Fahrenheit conversion until the user decides to stop
    while fahrenheit_running {
        // Perform the Fahrenheit conversion
        Fahrenheit::conversion_from();
        // Ask the user if they want to continue running the Fahrenheit conversion
        let continue_running = Select::new("Would you like to get another fahrenheit val? (y/n) ", vec!["y", "n"]).prompt();
        match continue_running {
            Ok(ans) => {
                // Stop the Fahrenheit conversion if the user answers 'n'
                if ans == "n" {
                    fahrenheit_running = false;
                }
            },
            Err(_) => {
                // Print an error message and stop the Fahrenheit conversion if there is an error getting user input
                println!("An error occurred while getting user input");
                fahrenheit_running = false;
            }
        }
    }
}

fn celsius_loop() {
    // Flag to keep the Celsius convertor runnin
    let mut cels_loop_running: bool = true;
    // Keep running the Celsius convertor until the user decides to stop
    while cels_loop_running {
        // Perform the Celsius conversion
        Celsius::get_celsius();
        // Ask the user if they want to continue running the Celsius conversion
        let continue_running = Select::new("Would you like to get another celsius val? (y/n) ", vec!["y", "n"]).prompt();
        match continue_running {
            Ok(ans) => {
                // Stop the Celsius conversion if the user answers 'n'
                if ans == "n" {
                    cels_loop_running = false;
                }
            },
            Err(_) => {
                // Print an error message and stop the Celsius conversion if there is an error getting user input
                println!("An error occurred while getting user input");
                cels_loop_running = false;
            }
        }
    }
}

fn kelvin_loop() {
    // Flag to keep the Kelvin convertor running
    let mut kelvin_loop_running: bool = true;
    // Keep running the Kelvin convertor until the user decides to stop
    while kelvin_loop_running {
        // Perform the Kelvin conversion
        Kelvin::get_kelvin();
        // Ask the user if they want to continue running the Kelvin conversion
        let continue_running = Select::new("Would you like to get another kelvin val? (y/n) ", vec!["y", "n"]).prompt();
        match continue_running {
            Ok(ans) => {
                // Stop the Kelvin conversion if the user answers 'n'
                if ans == "n" {
                    kelvin_loop_running = false;
                }
            },
            Err(_) => {
                // Print an error message and stop the Kelvin conversion if there is an error getting user input
                println!("An error occurred while getting user input");
                kelvin_loop_running = false;
            }
        }
    }
}

// Function to match the temperature type and call the appropriate function
fn match_temp(temp: &Result<String, InquireError>) {
    match temp {
        Ok(temp) => {
            match temp.as_str() {
                "Fahrenheit" => fahrenheit_loop(),
                "Celsius" => celsius_loop(),
                "Kelvin" => kelvin_loop(),
                _ => println!("Invalid temperature type"),
            }
        },
        Err(_) => println!("Error getting temperature type"),
    }
}

// Function to get the temperature type and call the appropriate function
pub fn get_data_type() -> Result<(), InquireError> {
    let temp_select = get_temp_type();
    match_temp(&temp_select);
    Ok(())
}