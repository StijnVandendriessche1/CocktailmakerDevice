use rppal::i2c::I2c;
use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;

pub fn run()
{
    let lcd = I2c::with_bus(1);
}

fn init()
{
    let mut E = Gpio::new().expect("cannot make new").get(27).expect("cannot get pin").into_output();
    let mut RS = Gpio::new().expect("cannot make new").get(17).expect("cannot get pin").into_output();
}

// def __init__(self, E = 27, RS = 17):
//         self.E = E
//         self.RS = RS
//         GPIO.setup([self.E, self.RS], GPIO.OUT)
//         GPIO.output(self.E, GPIO.HIGH)
//         self.send_instruction(inst["function_set"])
//         self.send_instruction(inst["display_on"])
//         self.send_instruction(inst["clear_display_&_cursor_home"])