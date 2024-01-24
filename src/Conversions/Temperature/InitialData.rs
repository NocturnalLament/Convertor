use inquire::Select;
#[path = "Fahrenheit.rs"] mod Fahrenheit;
#[path = "Celsius.rs"] mod Celsius;
#[path = "Kelvin.rs"] mod Kelvin;


pub fn get_data_type() {
    let mut TypesOfTemp = Vec::new();
    TypesOfTemp.push("Fahrenheit");
    TypesOfTemp.push("Celsius");
    TypesOfTemp.push("Kelvin");
    let mut temp_select = Select::new("Select a temperature type: ", TypesOfTemp).prompt();
    match temp_select {
        Ok(temp) => {
            match temp {
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

