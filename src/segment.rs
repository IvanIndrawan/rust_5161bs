use cortex_m::delay::Delay;
use embedded_hal::digital::v2::OutputPin;
use rp_pico::hal::gpio::{DynPinId, FunctionSio, Pin, PullDown, SioOutput};


pub trait Demo {
    fn demo(&mut self, num: u8);
    fn display(&mut self, num: u8);
    fn demo_blink(&mut self, num: u8);
}

pub struct Segment<'d> {
    segment_pins: [Pin<DynPinId, FunctionSio<SioOutput>, PullDown>; 8],
    delay: &'d mut Delay,
}

impl <'d>Segment<'d> {
    pub fn new(segment_pins: [Pin<DynPinId, FunctionSio<SioOutput>, PullDown>; 8], delay: &'d mut Delay) -> Self {
        Self { segment_pins, delay }
    }

    pub fn clear(&mut self) {
        self.segment_pins.iter_mut().for_each(|x| {
            x.set_high().unwrap();
        });
    }

    pub fn display_all(&mut self) {
        self.segment_pins.iter_mut().for_each(|x| {
            x.set_low().unwrap();
        });
    }
}

impl <'d>Demo for Segment<'d> {
    fn demo_blink(&mut self, num: u8) {
        self.clear();
        for _ in 0..num {
            self.display_all();
            self.delay.delay_ms(200);
            self.clear();
            self.delay.delay_ms(200);
        }
    }


    fn demo(&mut self, num: u8) {
        self.clear();
        for _j in 0..num {
            for i in 0..6 {
                self.segment_pins[i].set_low().unwrap();
                self.delay.delay_ms(50);
                self.segment_pins[i].set_high().unwrap();
            }
        }
    }

    fn display(&mut self, num: u8) {
        self.clear();
        let to_turn = match (num) {
            0 => [0, 1, 2, 3, 4, 5, 9],
            1 => [1, 2, 9, 9, 9, 9, 9],
            2 => [0, 1, 6, 4, 3, 9, 9],
            3 => [0, 1, 6, 2, 3, 9, 9],
            4 => [1, 2, 5, 6, 9, 9, 9],
            5 => [0, 2, 3, 5, 6, 9, 9],
            6 => [0, 2, 3, 4, 5, 6, 9],
            7 => [0, 1, 2, 9, 9, 9, 9],
            8 => [0, 1, 2, 3, 4, 5, 6],
            9 => [0, 1, 2, 5, 6, 9, 9],
            _ => [9, 9, 9, 9, 9, 9, 9]
        };
        to_turn.iter().for_each(|i| {
            if *i < 9 {
                self.segment_pins[*i as usize].set_low().unwrap();
            }
        });
        self.delay.delay_ms(500);
    }
}