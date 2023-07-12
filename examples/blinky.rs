#![no_main]
#![no_std]

use embedded_hal::digital::v2::*;
use msp430_rt::entry;
use msp430fr247x_hal::{gpio::Batch, pmm::Pmm, watchdog::Wdt};
extern crate panic_msp430;

// Greem onboard LED should blink at a steady period.
// Red onboard LED should go on when P2.3 button is pressed
#[entry]
fn main() -> ! {
    let periph = msp430fr247x::Peripherals::take().unwrap();
    let _wdt = Wdt::constrain(periph.WDT_A);

    let pmm = Pmm::new(periph.PMM);
    let p1 = Batch::new(periph.P1).split(&pmm);
    let p2 = Batch::new(periph.P2)
        .config_pin3(|p| p.pullup())
        .split(&pmm);
    let p5 = Batch::new(periph.P5)
        .config_pin1(|p| p.to_output())
        .split(&pmm);

    let mut p1_0 = p1.pin0.to_output();
    let p2_3 = p2.pin3;
    let mut p5_1 = p5.pin1;

    // pmm.locklpm5.write(|w| w.locklpm5.clear_bit());

    loop {
        p1_0.toggle().ok();

        for _ in 0..5000 {
            if p2_3.is_high().unwrap() {
                p5_1.set_low().ok();
            } else {
                p5_1.set_high().ok();
            }
        }
    }
}

// The compiler will emit calls to the abort() compiler intrinsic if debug assertions are
// enabled (default for dev profile). MSP430 does not actually have meaningful abort() support
// so for now, we create our own in each application where debug assertions are present.
#[no_mangle]
extern "C" fn abort() -> ! {
    panic!();
}
