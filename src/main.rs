#![no_std]
#![no_main]

/// additional imports
/// bitvec = "1.0.1"
/// embedded-dma = "0.2.0"
/// fugit = "0.3.7"
/// smart-leds-trait = "0.2.1"



// pub mod a;
// pub mod dma_ccr_timer;
// use crate::a::*;
// use crate::dma_ccr_timer::*;



use cortex_m_rt::entry;
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{prelude::*, pac::{Peripherals, dma1}, rcc::RccExt, timer::{TimerExt, Channel1, Channel2, Channel3}};
use {cortex_m_rt as _, panic_probe as _};

// mod dma;
// use dma::DmaCcrTimer;


/// ccr - compare (trigger an external event after a predetermined amount of time has expired)/ capture(measure the duration of an event) register


#[entry]
fn main() -> ! {

    if let Some(dp) = Peripherals::take() {

        ///clock
        let mut rcc = dp.RCC.constrain();
        let mut clocks = rcc.cfgr.freeze();
        let mut delay = dp.TIM2.delay_ms(&clocks);

        /// set pin and dma
        let gpiob=dp.GPIOB.split();
        // let dma1=dp.DMA1.split();
        let gpioa = dp.GPIOA.split();
        let channels = (Channel2::new(gpiob.pb7), Channel3::new(gpiob.pb8));


        /// set pin behavior
        // let mut led=gpiob.pb7.into_push_pull_output();
        // let wub=gpiob.pb7.into_alternate();

        ///channel 4 and timer chan 2
        let pwm = dp.TIM4.pwm_hz(channels, 1.Hz(), &clocks).split();
        let (mut ch2, _ch3) = pwm;
        let max_duty = ch2.get_max_duty();
        ch2.set_duty(max_duty / 2);
        ch2.enable();


        // reset leds cuz no clue which one works when otherwise
        // let mut led=gpiob.pb8.into_push_pull_output();

        // loop {
        //     led.toggle();
        //     delay.delay_us(80_u32);
        // }
    }

    // rtt_init_print!();
    // rprintln!("hw");
    
    loop {
        cortex_m::asm::nop();
    }

}