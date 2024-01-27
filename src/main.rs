// Import the Text struct from the inquire crate
use inquire::Text;

// Import the temperature and length conversion modules
#[path = "Conversions/Temperature/InitialData.rs"] mod TempType;
#[path = "Conversions/Length/InitialData.rs"] mod LengthType;
#[path = "Conversions/Volume/InitialData.rs"] mod VolumeType;
// Define an enum for the types of conversions
enum Conversion {
    Temperature,
    Length,
}

// Define the entry point function
fn entry_point() {
    // Initialize a vector to hold the types of conversions
    let mut TypesOfConversion = Vec::new();
    TypesOfConversion.push("Temperature");
    TypesOfConversion.push("Length");
    TypesOfConversion.push("Volume");
    TypesOfConversion.push("Open Source Projects We Love 💖");
    
    // Prompt the user to select a conversion type
    let mut conversion_select = inquire::Select::new("Select a conversion type: ", TypesOfConversion).prompt();
    // Match the user's selection
    match conversion_select {
        Ok(conversion) => {
            match conversion {
                // If Temperature is selected, call the temperature conversion function
                "Temperature" => {
                    TempType::temp_entry_point();
                },
                // If Length is selected, call the length conversion function
                "Length" => {
                    let mut loop_running: bool = true;
                    while loop_running {
                        LengthType::get_length_type();
                        // Ask the user if they want to convert another length
                        let mut continue_running = Text::new("Would you like to get another length?? (y/n) ").prompt();
                        match continue_running {
                            Ok(ans) => {
                                match ans.as_str() {
                                    "y" => {
                                        loop_running = true;
                                    },
                                    "n" => {
                                        loop_running = false;
                                    },
                                    &_ => todo!(),
                                }
                            },
                            Err(_) => {
                                println!("Error")
                            }
                        }
                    }
                },
                "Volume" => {
                    VolumeType::volume_entry_point();
                },
                "Open Source Projects We Love 💖" => {
                    println!("We love the following open source projects:");
                    println!("Inquire - https://github.com/mikaelmello/inquire");
                },
                
                &_ => todo!(),
            }
        },
        Err(_) => {
            println!("Error")
        }
    }
}

// Define the main function
fn main() {
    // Print a welcome message
    println!("Welcome to convertor! - Version {}", env!("CARGO_PKG_VERSION"));
    println!("2024 NocturnalLament");
    let mut prog_running: bool = true;
    while prog_running {
        // Call the entry point function
        entry_point();
        // Ask the user if they want to continue
        let mut continue_running = Text::new("Would you like to continue? (y/n) ").prompt();
        match continue_running {
            Ok(ans) => {
                match ans.as_str() {
                    "y" => {
                        prog_running = true;
                    },
                    "n" => {
                        prog_running = false;
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