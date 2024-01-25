struct Yard {
    value: f64,
}

impl Yard {
    fn new(value: f64) -> Self {
        Self {
            value,
        }
    }

    fn show_centimeters(&self) -> f64 {
        self.value * 91.44
    }

    fn show_meters(&self) -> f64 {
        self.value * 0.9144
    }

    fn show_kilometers(&self) -> f64 {
        self.value * 0.0009144
    }

    fn show_miles(&self) -> f64 {
        self.value * 0.000568182
    }

    fn show_yards(&self) -> f64 {
        self.value
    }

    fn show_feet(&self) -> f64 {
        self.value * 3.0
    }

    fn show_millimeters(&self) -> f64 {
        self.value * 914.4
    }

    fn show_inches(&self) -> f64 {
        self.value * 36.0
    }
}