use inquire::{Select, InquireError};
#[path = "Fahrenheit.rs"] mod Fahrenheit;
#[path = "Celsius.rs"] mod Celsius;
#[path = "Kelvin.rs"] mod Kelvin;

fn get_temp_type() -> Result<String, InquireError>{
    let temp_select = Select::new("Select a temperature type: ", vec!["Fahrenheit", "Celsius", "Kelvin"]).prompt();
    match temp_select {
        Ok(temp) => {
            Ok(temp.to_string())
        },
        Err(_) => {
            todo!()
        }
    }
}

pub fn temp_entry_point() {
    let mut prog_running: bool = true;
    while prog_running {
        get_data_type();
        let mut continue_running = Select::new("Would you like to get another temperature? (y/n) ", vec!["y", "n"]).prompt();
        match continue_running {
            Ok(ans) => {
                match ans {
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


fn match_temp(temp: &Result<String, InquireError>) {
    match temp {
        Ok(temp) => {
            match temp.as_str() {
                "Fahrenheit" => {
                    Fahrenheit::conversion_from();
                },
                "Celsius" => {
                    Celsius::get_celsius();
                },
                "Kelvin" => {
                    Kelvin::get_kelvin();
                },
                &_ => todo!(),
            }
        }
        Err(_) => {
            println!("Error")
        }
    }
}

pub fn get_data_type() {
    let temp_select = get_temp_type();
    match_temp(&temp_select);
}

