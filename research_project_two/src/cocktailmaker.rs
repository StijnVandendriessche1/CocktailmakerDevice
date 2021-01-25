use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use isahc::prelude::*;

mod switch;
use switch::Switch;

mod ultrasoon;
use ultrasoon::Ultrasoon;

pub struct CocktailMachine {
    pumps: [OutputPin; 8],
    stop: bool,
    door: Switch,
    infra: Switch,
    ultra: Ultrasoon
}

impl CocktailMachine 
{
    pub fn new(p1: u8, p2: u8, p3: u8, p4: u8, p5: u8, p6: u8, p7: u8, p8: u8, door_pin: u8, door_pullup: bool, infra_pin: u8, infra_pullup: bool, ultra_trigger: u8, ultra_echo: u8) -> CocktailMachine 
    {
        CocktailMachine 
        {
            pumps: [
                Gpio::new().expect("cannot make new").get(p1).expect("cannot get pin").into_output(),
                Gpio::new().expect("cannot make new").get(p2).expect("cannot get pin").into_output(),
                Gpio::new().expect("cannot make new").get(p3).expect("cannot get pin").into_output(),
                Gpio::new().expect("cannot make new").get(p4).expect("cannot get pin").into_output(),
                Gpio::new().expect("cannot make new").get(p5).expect("cannot get pin").into_output(),
                Gpio::new().expect("cannot make new").get(p6).expect("cannot get pin").into_output(),
                Gpio::new().expect("cannot make new").get(p7).expect("cannot get pin").into_output(),
                Gpio::new().expect("cannot make new").get(p8).expect("cannot get pin").into_output(),
            ],
            stop: false,
            door: Switch::new(door_pin, CocktailMachine::callback_door, door_pullup),
            infra: Switch::new(infra_pin, CocktailMachine::callback_infra, infra_pullup),
            ultra: Ultrasoon::new(ultra_trigger, ultra_echo)
        }
    }

    pub fn make_cocktail(&mut self, code: &str) 
    {
        let steps = self.get_steps(code);
        for i in steps.iter() 
        {
            for j in i.iter() 
            {
                if !self.stop
                {
                    self.activate_pump_time_safe(j[0] - 1, j[1] as u64);
                }
            }
        }
    }

    //gaat als volgt:
    //     1Q3N2Q3N3Q3N4Q3N5Q3X6Q3N8Q2
    //     [1Q3N2Q3N3Q3N4Q3N5Q3,6Q3N8Q2]
    //     [[1Q3,2Q3,3Q3,4Q3,5Q3],[6Q3,8Q2]]
    //     [[[1,3],[2,3],[3,3],[4,3],[5,3]][[6,3],[8,2]]]
    fn get_steps(&self, code: &str) -> Vec<Vec<[u8; 2]>> {
        let c = &code.to_uppercase();
        let mut output: Vec<Vec<[u8; 2]>> = Vec::new();
        for i in c.split("X") {
            let mut step: Vec<[u8; 2]> = Vec::new();
            for j in i.split("N") {
                let mut drink = [0; 2];
                for (index, k) in j.split("Q").enumerate() {
                    drink[index] = k.parse().unwrap();
                }
                step.push(drink);
            }
            output.push(step);
        }
        output
    }

    fn activate_pump_time_safe(&mut self, pump: u8, time: u64)
    {
        let mut now = SystemTime::now();
        let mut el = 0;
        self.set_pump(pump, true);
        while el < time
        {
            if self.door.get_state() == rppal::gpio::Level::High || self.infra.get_state() == rppal::gpio::Level::High
            {
                println!("pauze");
                self.set_pump(pump, false);
                let waitingtime = self.wait_until_clear();
                now += Duration::from_millis(waitingtime as u64);
                self.set_pump(pump, true);
            }
            if self.ultra.get_avg_distance(20) <= 0.1
            {
                self.stop = true;
                println!("gestop - afstand: {}", self.ultra.get_avg_distance(20));
                break;
            }
            el = now.elapsed().expect("error").as_secs();
            sleep(Duration::from_millis(200));
        }
        self.set_pump(pump, false);
    }

    fn wait_until_clear(&self) -> u128
    {
        let now = SystemTime::now();
        while self.door.get_state() == rppal::gpio::Level::High || self.infra.get_state() == rppal::gpio::Level::High
        {
            sleep(Duration::from_millis(200));
        }
        now.elapsed().expect("error").as_millis()
    }

    fn set_pump(&mut self, pump: u8, state: bool) 
    {
        if state 
        {
            self.pumps[pump as usize].set_high();
        } 
        else 
        {
            self.pumps[pump as usize].set_low();
        }
    }

    fn callback_door(a: rppal::gpio::Level)
    {
        if a == rppal::gpio::Level::High
        {
            println!("Deur open");
        }
        else
        {
            println!("Deur gesloten");
        }
    }

    fn callback_infra(a: rppal::gpio::Level)
    {
        if a == rppal::gpio::Level::High
        {
            println!("Geen object gedetecteerd");
        }
        else
        {
            println!("Object gedetecteerd");
        }
    }
}