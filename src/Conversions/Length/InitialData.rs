use inquire::{Select};
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
                                    println!("millimeters");
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
                    let test = vec!["inches", "feet", "yards", "miles"];
                    for item in test {
                        println!("{}", item);
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

