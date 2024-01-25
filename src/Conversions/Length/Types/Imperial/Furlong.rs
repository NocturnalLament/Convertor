use inquire::{Select, Text};

pub struct Furlong {
    value: f64,
}

impl Furlong {
    pub fn new(value: f64) -> Self {
        Self {
            value,
        }
    }

    pub fn show_chain(&self) -> f64 {
        self.value * 10.0
    }

    pub fn show_feet(&self) -> f64 {
        self.value * 660.0
    }

    pub fn show_inch(&self) -> f64 {
        self.value * 7920.0
    }

    pub fn show_mile(&self) -> f64 {
        self.value * 0.125
    }

    pub fn show_yard(&self) -> f64 {
        self.value * 220.0
    }

    pub fn show_centimeters(&self) -> f64 {
        self.value * 201168.0
    }

    pub fn show_meters(&self) -> f64 {
        self.value * 201.168
    }    

    pub fn show_kilometers(&self) -> f64 {
        self.value * 0.201168
    }

    pub fn show_decimeters(&self) -> f64 {
        self.value * 2011.68
    }
}

pub fn furlong_prompt() {
    let amount = Text::new("Enter the amount of furlongs you want to convert to other units of measurement: ").prompt();
    match amount {
        Ok(amount) => {
            match amount.parse::<f64>() {
                Ok(amount_f) => {
                    let amount = Furlong::new(amount_f);
                    display_furlong(&amount);
                }

                Err(_) => {
                    println!("Error");
                }
            }
        },
        Err(_) => {
            println!("Error");
        }
    }
}

fn display_furlong(f: &Furlong) {
    let choice = Select::new("Select a unit of measurement to convert to: ", vec![
        "Chain", "Feet", "Inch", "Mile", "Yard", "Centimeter", "Meter", "Kilometer", "Decimeter"]).prompt();
        match choice {
            Ok(choice) => {
                match choice {
                    "Chain" => {
                        println!("{} furlongs is {} chains.", f.value, f.show_chain());
                    },
                    "Feet" => {
                        println!("{} furlongs is {} feet.", f.value, f.show_feet());
                    },
                    "Inch" => {
                        println!("{} furlongs is {} inches.", f.value, f.show_inch());
                    },
                    "Mile" => {
                        println!("{} furlongs is {} miles.", f.value, f.show_mile());
                    },
                    "Yard" => {
                        println!("{} furlongs is {} yards.", f.value, f.show_yard());
                    },
                    "Centimeter" => {
                        println!("{} furlongs is {} centimeters.", f.value, f.show_centimeters());
                    },
                    "Meter" => {
                        println!("{} furlongs is {} meters.", f.value, f.show_meters());
                    },
                    "Kilometer" => {
                        println!("{} furlongs is {} kilometers.", f.value, f.show_kilometers());
                    },
                    "Decimeter" => {
                        println!("{} furlongs is {} decimeters.", f.value, f.show_decimeters());
                    },
                    _ => todo!(),
                }
            },
            Err(_) => {
                println!("Error");
            }
        }
}