struct Inch {
    value: f64,
}

impl Inch {
    fn new(value: f64) -> Self {
        Self {
            value,
        }
    }

    fn show_centimeters(&self) -> f64 {
        self.value * 2.54
    }

    fn show_meters(&self) -> f64 {
        self.value * 0.0254
    }

    fn show_kilometers(&self) -> f64 {
        self.value * 0.0000254
    }

    fn show_miles(&self) -> f64 {
        self.value * 0.000015783
    }

    fn show_yards(&self) -> f64 {
        self.value * 0.027778
    }

    fn show_feet(&self) -> f64 {
        self.value * 0.083333
    }

    fn show_millimeters(&self) -> f64 {
        self.value * 25.4
    }

    fn show_inches(&self) -> f64 {
        self.value
    }
}