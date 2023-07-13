#![no_main]
#![no_std]

use embedded_hal::prelude::*;
use msp430_rt::entry;
use msp430fr247x_hal::{
    clock::{ClockConfig, DcoclkFreqSel, MclkDiv, SmclkDiv},
    fram::Fram,
    gpio::Batch,
    pmm::Pmm,
    pwm::{Pwm, PwmParts7, PwmPeriph, TimerConfig},
    watchdog::Wdt,
};
use panic_msp430 as _;

enum Turns {
    PWM1,
    PWM2,
    // PWM3,
    END
}

struct Turn(Turns);

impl Turn {
    fn new() -> Turn
    {
        Turn {
            0: Turns::PWM1
        }
    }

    fn next(&mut self)
    {
        match self.0 {
            Turns::PWM1 => {
                self.0 = Turns::PWM2;
            }
            Turns::PWM2 => {
                self.0 = Turns::END
            }
            // Self::PWM3 => {
            //     self = Self::END
            // }
            Turns::END => {
                self.0 = Turns::PWM1
            }
        }
    }
}

// P6.4 LED should be bright, P6.3 LED should be dim
#[entry]
fn main() -> ! {
    let periph = msp430fr247x::Peripherals::take().unwrap();

    let mut fram = Fram::new(periph.FRCTL);
    Wdt::constrain(periph.WDT_A);

    let pmm = Pmm::new(periph.PMM);
    let p4 = Batch::new(periph.P4).split(&pmm);
    let p5 = Batch::new(periph.P5).split(&pmm);

    let (smclk, _aclk) = ClockConfig::new(periph.CS)
        .mclk_dcoclk(DcoclkFreqSel::_1MHz, MclkDiv::_1)
        .smclk_on(SmclkDiv::_1)
        .aclk_vloclk()
        .freeze(&mut fram);

    let pwm = PwmParts7::new(periph.TB0, TimerConfig::smclk(&smclk), 10000);
    let mut pwm1 = pwm.pwm1.init(p4.pin7.to_output().to_alternate2()); // Blue
    let mut pwm2 = pwm.pwm2.init(p5.pin0.to_output().to_alternate2()); // Green
    // let mut pwm3 = pwm.pwm4.init(p5.pin2.to_output().to_alternate2()); // Red

    let mut turn = Turn::new();
    let mut dc:u16 = 0;
    let mut up:bool = true;

    config_pwm(&mut pwm1, 0);
    config_pwm(&mut pwm2, 0);

    loop {
        match turn.0 {
            Turns::PWM1 => pwm1.set_duty(dc),
            Turns::PWM2 => pwm2.set_duty(dc),
            // 2 => config_pwm(&mut pwm3, dc),
            Turns::END => {
                    turn.0 = Turns::PWM1;
                }
            }

        match up {
            true => {
                dc += 1;
                if dc >= 10000 {
                    up = false;
                }
            },
            false => {
                dc -= 1;
                if dc == 0 {
                    up = true;
                    turn.next();
                }
            }
        }
        
    }
}

fn config_pwm<T: PwmPeriph<C>, C>(pwm: &mut Pwm<T, C>, duty: u16) {
    pwm.enable();
    pwm.set_duty(duty);
}

// The compiler will emit calls to the abort() compiler intrinsic if debug assertions are
// enabled (default for dev profile). MSP430 does not actually have meaningful abort() support
// so for now, we create our own in each application where debug assertions are present.
#[no_mangle]
extern "C" fn abort() -> ! {
    panic!();
}
