use rppal::gpio::Gpio;
use rppal::gpio::{OutputPin,InputPin};
use std::time::{Duration, SystemTime};
use std::thread::sleep;

pub struct UltraSonic
{
    trigger: OutputPin,
    echo: InputPin
}

impl UltraSonic
{
    pub fn new(trigger: u8, echo: u8) -> UltraSonic
    {
        UltraSonic{trigger: Gpio::new().expect("cannot make new").get(trigger).expect("cannot get pin").into_output(), echo: Gpio::new().expect("cannot make new").get(echo).expect("cannot get pin").into_input()}
    }

    pub fn get_distance(&mut self)
    {
        self.trigger.set_high();
        sleep(Duration::from_nanos(1000));
        self.trigger.set_low();
        let elapsed = SystemTime::now();
        let now = SystemTime::now();
        while self.echo.read() == rppal::gpio::Level::Low
        {
            now = SystemTime::now();
        }
        while self.echo.read() == rppal::gpio::Level::High
        {
            elapsed = SystemTime::now();
        }
        let e = elapsed - now;
        elapsed*34300/2
    }

    pub fn get_avg_distance(&mut self, count: u8)
    {
        avg = 0
        for i in 0..count
        {
            avg += self.get_distance();
        }
        avg/count
    }
}