struct Feet {
    value: f64,
}

impl Feet {
    fn new(value: f64) -> Self {
        Self {
            value,
        }
    }

    fn show_centimeters(&self) -> f64 {
        self.value * 30.48
    }

    fn show_meters(&self) -> f64 {
        self.value * 0.3048
    }

    fn show_kilometers(&self) -> f64 {
        self.value * 0.0003048
    }

    fn show_miles(&self) -> f64 {
        self.value * 0.000189394
    }

    fn show_yards(&self) -> f64 {
        self.value * 0.333333
    }

    fn show_feet(&self) -> f64 {
        self.value
    }

    fn show_millimeters(&self) -> f64 {
        self.value * 304.8
    }

    fn show_inches(&self) -> f64 {
        self.value * 12.0
    }
}