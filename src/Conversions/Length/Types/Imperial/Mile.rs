struct Mile {
    value: f64,
}

impl Mile {
    fn new(value: f64) -> Self {
        Self {
            value,
        }
    }

    fn show_centimeters(&self) -> f64 {
        self.value * 160934.0
    }

    fn show_meters(&self) -> f64 {
        self.value * 1609.34
    }

    fn show_kilometers(&self) -> f64 {
        self.value * 1.60934
    }

    fn show_miles(&self) -> f64 {
        self.value
    }

    fn show_yards(&self) -> f64 {
        self.value * 1760.0
    }

    fn show_feet(&self) -> f64 {
        self.value * 5280.0
    }

    fn show_millimeters(&self) -> f64 {
        self.value * 1609340.0
    }

    fn show_inches(&self) -> f64 {
        self.value * 63360.0
    }
}