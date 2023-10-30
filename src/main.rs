#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{prelude::*, pac::Peripherals, rcc::RccExt, timer::TimerExt};
use {cortex_m_rt as _, panic_probe as _};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = Peripherals::take().unwrap();

    let cp = cortex_m::Peripherals::take().unwrap();

    let gc = dp.GPIOC.split();
    let mut led = gc.pc13.into_push_pull_output();

    let mut rcc = dp.RCC.constrain();
    let mut clocks = rcc.cfgr.freeze();

    let mut delay = dp.TIM2.delay_ms(&clocks);

    rprintln!("hw");
    loop {
        led.set_high();
        delay.delay_ms(400_u32);
        led.set_low();
    }
}
