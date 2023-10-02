use std::collections::HashSet;

pub trait Observer {
    fn update(&mut self, temp: f64, humidity: f64, pressure: f64);
}

pub trait DisplayElement {
    fn display(&self);
}

pub struct CurrentConditionsDisplay {
    temperature: f64,
    humidity: f64,
}

impl CurrentConditionsDisplay {
    pub fn new() -> Self {
        Self {
            temperature: 0.0,
            humidity: 0.0,
        }
    }
}

impl Observer for CurrentConditionsDisplay {
    fn update(&mut self, temp: f64, humidity: f64, pressure: f64) {
        self.temperature = temp;
        self.humidity = humidity;
    }
}

impl DisplayElement for CurrentConditionsDisplay {
    fn display(&self) {
        println!(
            "Current conditions: {}C degreees and {}% humidity",
            self.temperature, self.humidity
        )
    }
}

pub trait Subject<'a, O>
where
    O: Observer,
{
    fn register_observer(&mut self, observer: &'a O);
    fn remove_observer(&mut self, observer: &'a O);
    fn notify_observer(&self);
}

struct WeatherData<'a, O> {
    observers: Vec<&'a O>,
}

impl<'a, O> Subject<'a, O> for WeatherData<'a, O>
where
    O: Observer,
{
    fn register_observer(&mut self, observer: &'a O) {
        self.observers.push(observer)
    }

    fn remove_observer(&mut self, observer: &O) {
        todo!()
    }

    fn notify_observer(&self) {
        todo!()
    }
}

impl<'a, O> WeatherData<'a, O> {
    pub fn new() -> Self {
        Self { observers: vec![] }
    }

    pub fn test(&self) {}
}

fn main() {
    let mut weather_data = WeatherData::new();
    let current_condition_display = CurrentConditionsDisplay::new();
    weather_data.register_observer(&current_condition_display);
}
