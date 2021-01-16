use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;
use rppal::i2c::I2c;
use std::thread::sleep;
use std::time::Duration;

pub struct Lcd
{
    bus: rppal::i2c::I2c,
    e: OutputPin,
    rs: OutputPin
}

enum Instruction
{
    FunctionSet = 0b00111000,
    DisplayOn = 0b00001111,
    ClearDisplayAndCursorHome = 0b00000001,
    NewLine = 0b11000000
}

impl Lcd
{
    pub fn new(bus: u8, e: u8, rs: u8) -> Lcd
    {
        let mut l = Lcd{bus: I2c::with_bus(bus).expect("cannot make new"), e: Gpio::new().expect("cannot make new").get(e).expect("cannot get pin").into_output(), rs: Gpio::new().expect("cannot make new").get(rs).expect("cannot get pin").into_output()};
        l.bus.set_slave_address(0x38).expect("error");
        l.send_instruction(Instruction::FunctionSet);
        l.send_instruction(Instruction::DisplayOn);
        l.send_instruction(Instruction::ClearDisplayAndCursorHome);
        l
    }

    fn send_instruction(&mut self, value: Instruction)
    {
        self.rs.set_low();
        self.e.set_high();
        self.set_data_bits(value as u8);
        self.e.set_high();
        sleep(Duration::from_millis(1));
        self.e.set_low();
    }

    fn send_character(&mut self, value: u8)
    {
        self.rs.set_high();
        self.e.set_high();
        self.set_data_bits(value);
        self.e.set_high();
        sleep(Duration::from_millis(1));
        self.e.set_low();
    }

    fn set_data_bits(&mut self, byte: u8)
    {
        self.bus.smbus_write_byte(0x38, byte as u8).expect("error");
    }
    
    pub fn write_message(&mut self, value: &str)
    {
        for i in value.chars()
        {
            self.send_character(i as u8);
        }
    }

    pub fn write_scroll(&mut self, head: &str, body: &str)
    {
        self.send_instruction(Instruction::ClearDisplayAndCursorHome);
        if body.len() > 16
        {
            let first = &body[..16];
            self.write_message(head);
            self.send_instruction(Instruction::NewLine);
            self.write_message(first);
            let last = &body[16..];
            for (index, _word) in last.chars().enumerate()
            {
                sleep(Duration::from_secs_f32(0.5));
                self.send_instruction(Instruction::ClearDisplayAndCursorHome);
                self.write_message(head);
                self.send_instruction(Instruction::NewLine);
                self.write_message(&body[index+1..16+index+1]);
            }
        }
        else
        {
            self.write_message(head);
            self.send_instruction(Instruction::NewLine);
            self.write_message(body);
        }
    }
}