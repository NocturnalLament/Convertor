use inquire::{Select, Text};

pub struct Meter {
    value: f64
}

impl Meter {
    fn new(value: f64) -> Self {
        Self {
            value,
        }
    }

    fn show_centimeters(&self) -> f64 {
        self.value * 100.0
    }

    fn show_meters(&self) -> f64 {
        self.value
    }

    fn show_kilometers(&self) -> f64 {
        self.value * 0.001
    }

    fn show_miles(&self) -> f64 {
        self.value * 0.000621371
    }

    fn show_yards(&self) -> f64 {
        self.value * 1.09361
    }

    fn show_feet(&self) -> f64 {
        self.value * 3.28084
    }

    fn show_millimeters(&self) -> f64 {
        self.value * 1000.0
    }

    fn show_decimeters(&self) -> f64 {
        self.value * 10.0
    }

    fn show_inches(&self) -> f64 {
        self.value * 39.3701
    }

    fn show_furlong(&self) -> f64 {
        self.value * 0.00497096
    }

    fn show_chain(&self) -> f64 {
        self.value * 0.0497097
    }

    fn to_miles(&self) -> f64 {
        self.value * 0.000621371
    }

    fn to_yards(&self) -> f64 {
        self.value * 1.09361
    }

    fn to_feet(&self) -> f64 {
        self.value * 3.28084
    }

    fn to_inches(&self) -> f64 {
        self.value * 39.3701
    }

    fn to_furlong(&self) -> f64 {
        self.value * 0.00497096
    }

    fn to_chain(&self) -> f64 {
        self.value * 0.0497097
    }

    fn to_decimeters(&self) -> f64 {
        self.value * 10.0
    }

    fn to_millimeters(&self) -> f64 {
        self.value * 1000.0
    }

    fn to_centimeters(&self) -> f64 {
        self.value * 100.0
    }

    fn to_kilometers(&self) -> f64 {
        self.value * 0.001
    }
}

fn convert_meter_metric() {
    let amount = Text::new("How many meters are you converting? ").prompt().unwrap();
    match amount.parse::<f64>() {
        Ok(amount_val) => {
            let meter = Meter::new(amount_val);
            let choice = Select::new("What are you converting to?", vec!["Centimeters", "Decimeters", "Kilometers", "Meters", "Millimeters"]).prompt().unwrap();
            match choice {
                "Centimeters" => {
                    println!("{}", meter.show_centimeters());
                },
                "Decimeters" => {
                    println!("{}", meter.show_decimeters());
                },
                "Kilometers" => {
                    println!("{}", meter.show_kilometers());
                },
                "Meters" => {
                    println!("{}", meter.show_meters());
                },
                "Millimeters" => {
                    println!("{}", meter.show_millimeters());
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

fn convert_meter_imperial() {
    let amount = Text::new("How many meters are you converting? ").prompt().unwrap();
    match amount.parse::<f64>() {
        Ok(amount_val) => {
            let meter = Meter::new(amount_val);
            let choice = Select::new("What are you converting to?", vec!["Miles", "Yards", "Feet", "Inches", "Furlong", "Chain"]).prompt().unwrap();
            match choice {
                "Miles" => {
                    println!("{}", meter.show_miles());
                },
                "Yards" => {
                    println!("{}", meter.show_yards());
                },
                "Feet" => {
                    println!("{}", meter.show_feet());
                },
                "Inches" => {
                    println!("{}", meter.show_inches());
                },
                "Furlong" => {
                    println!("{}", meter.show_furlong());
                },
                "Chain" => {
                    println!("{}", meter.show_chain());
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

fn convert_switch(){
    let choice = Select::new("What are you converting to?", vec!["Metric", "Imperial"]).prompt().unwrap();
    match choice {
        "Metric" => {
            convert_meter_metric();
        },
        "Imperial" => {
            convert_meter_imperial();
        },
        _ => {
            println!("Error");
        }
    }
}

pub fn convert_meter() {
    let loop_running = true;
    while loop_running {
        convert_switch();
        let choice = Select::new("Would you like to convert again?", vec!["Yes", "No"]).prompt().unwrap();
        match choice {
            "Yes" => {
                continue;
            },
            "No" => {
                break;
            },
            _ => {
                println!("Error");
            }
        }
    }
}