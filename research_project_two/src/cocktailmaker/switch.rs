use rppal::gpio::Gpio;
use rppal::gpio::InputPin;

type Callback = fn(a: rppal::gpio::Level);

pub struct Switch
{
    pin: InputPin,
    callback: Callback
}

impl Switch
{
    pub fn new(pin: u8, c: Callback, pullup: bool) -> Switch
    {
        let mut p: InputPin;
        if pullup
        {
            p = Gpio::new().expect("cannot make new").get(pin).expect("cannot get pin").into_input_pullup();
        }
        else
        {
            p = Gpio::new().expect("cannot make new").get(pin).expect("cannot get pin").into_input();
        }
        let mut s = Switch{pin: p, callback: c};
        s.pin.set_async_interrupt(rppal::gpio::Trigger::Both, c);
        s
    }

    pub fn get_state(&self) -> rppal::gpio::Level
    {
        self.pin.read()
    }
}

