use rppal::gpio::Gpio;
use rppal::gpio::{InputPin,OutputPin};
use std::time::{Duration,SystemTime};
use std::thread::sleep;

pub struct Ultrasoon
{
    trigger: OutputPin,
    echo: InputPin
}

impl Ultrasoon
{
    pub fn new(trigger: u8, echo: u8) -> Ultrasoon
    {
        Ultrasoon
        {
            trigger: Gpio::new().expect("kon geen nieuwe pin aanmaken").get(trigger).expect("kon pin niet opvragen").into_output(),
            echo: Gpio::new().expect("kon geen nieuwe pin aanmaken").get(echo).expect("kon pin niet opvragen").into_input()
        }
    }

    pub fn get_distance(&mut self) -> f64
    {
        self.trigger.set_high();
        sleep(Duration::from_micros(10));
        self.trigger.set_low();
        let mut start = SystemTime::now();
        let mut stop = start.elapsed().unwrap().as_secs_f64();
        while self.echo.read() == rppal::gpio::Level::Low
        {
            start = SystemTime::now();
        }
        while self.echo.read() == rppal::gpio::Level::High
        {
            stop = start.elapsed().unwrap().as_secs_f64();
        }
        stop / 2.0 * 343.0
    }

    pub fn get_avg_distance(&mut self, count: u8) -> f64
    {
        let mut a = 0.0;
        for i in 0..count
        {
            a += self.get_distance();
        }
        a/count as f64
    }
}