struct Chain {
    value: f64,
}

impl Chain {
    fn new(value: f64) -> Self {
        Self {
            value,
        }
    }

    fn show_chain(&self) -> f64 {
        self.value
    }

    fn show_feet(&self) -> f64 {
        self.value * 66.0
    }

    fn show_furlong(&self) -> f64 {
        self.value * 0.1
    }

    fn show_inch(&self) -> f64 {
        self.value * 792.0
    }

    fn show_mile(&self) -> f64 {
        self.value * 0.0125
    }

    fn show_yard(&self) -> f64 {
        self.value * 22.0
    }

    fn show_centimeters(&self) -> f64 {
        self.value * 20116.8
    }

    fn show_meters(&self) -> f64 {
        self.value * 20.1168
    }

    fn show_kilometers(&self) -> f64 {
        self.value * 0.0201168
    }

    fn show_millimeters(&self) -> f64 {
        self.value * 201168.0
    }

    fn show_decimeters(&self) -> f64 {
        self.value * 2011.68
    }
}