#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use nb::block;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut gpio_c = dp.GPIOC.split(&mut rcc.apb2);
    let mut led = gpio_c.pc13.into_push_pull_output(&mut gpio_c.crh);
    let mut timer = Timer::syst(cp.SYST, &clocks).start_count_down(1.hz());
    loop {
        block!(timer.wait()).unwrap();
        led.set_high().unwrap();
        block!(timer.wait()).unwrap();
        led.set_low().unwrap();
    }
}
