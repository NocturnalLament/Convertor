use inquire::{Select, Text};
pub fn get_length_type() {
    /* Actually Getting Whether or not user wants to convert from Metric or Imperial unlike Temperature
    Because of how many different types of units there are for length it makes more sense to have this extra step 
    */
    let mut SystemsOfMeasurement = Select::new("Do you use metric or imperial? ", vec!["metric", "imperial"]).prompt();
    match SystemsOfMeasurement {
        Ok(system) {
            match system {
                "metric" => {
                    metric_entry_point();
                },
                "imperial" => {
                    imperial_entry_point();
                },
            }
        },
        Err(_) => {
            println!("Error")
        }
    }
}

