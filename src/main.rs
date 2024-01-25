use inquire::Text;
#[path = "Conversions/Temperature/InitialData.rs"] mod TempType;
#[path = "Conversions/Length/InitialData.rs"] mod LengthType;
enum Conversion {
    Temperature,
    Length,
}
fn entry_point() {
    let mut TypesOfConversion = Vec::new();
    TypesOfConversion.push("Temperature");
    TypesOfConversion.push("Length");
    
    let mut conversion_select = inquire::Select::new("Select a conversion type: ", TypesOfConversion).prompt();
    match conversion_select {
        Ok(conversion) => {
            match conversion {
                "Temperature" => {
                    TempType::temp_entry_point();
                },
                "Length" => {
                    LengthType::get_length_type();
                },
                &_ => todo!(),
            }
        },
        Err(_) => {
            println!("Error")
        }
    }
}

fn main() {
    let mut prog_running: bool = true;
    while prog_running {
        entry_point();
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
