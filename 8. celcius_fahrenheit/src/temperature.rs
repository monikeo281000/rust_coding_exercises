
pub enum Scale {
    Celsius,
    Fahrenheit,
}

pub struct Temperature {
    degrees: f32,
    scale: Scale,
}

impl Temperature {
    pub fn new(degrees: f32, scale: Scale) -> Self {
        Self {
            degrees,
            scale,
        }
    }

    pub fn degrees(&self) -> &f32 {
        &self.degrees
    }

    pub fn scale(&self) -> String {
        match &self.scale {
            Scale::Celsius => "celsius".to_string(),
            Scale::Fahrenheit => "fahrenheit".to_string(),
        }
    }

    /*
     pub fn to_celsius(self) -> f32 {
        match self.scale {
            Scale::Celsius  => self.degrees,
            Scale::Fahrenheit => celsius(self.degrees),
        }
     }
     */ 

    pub fn to_celsius(&mut self) {
        match self.scale {
            Scale::Fahrenheit => {
                self.scale =  Scale::Celsius;
                self.degrees = celsius(self.degrees);
            },
            _ => {},
        }
    }

    pub fn to_fahrenheit(&mut self) {
        match self.scale {
            Scale::Celsius => {
                self.scale = Scale::Fahrenheit;
                self.degrees = fahrenheit(self.degrees);
            },
            _ => {},
        }
    }
}

fn celsius(fahrenheit: f32) -> f32 {
    (5./9.) * (fahrenheit - 32.)
}

fn fahrenheit(celsius: f32) -> f32 {
    (9./5.) * celsius + 32.
}
