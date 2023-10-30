#![no_std]
#![no_main]



pub mod a;
pub mod dma_ccr_timer;
use crate::a::*;
use crate::dma_ccr_timer::*;



// use core::mem::{self, size_of};
// use bitvec::prelude::{BitArray, Msb0};
use stm32f4xx_hal::{timer::{Pins,CCR, PwmExt,Pwm, Ch}, dma::{MemoryToPeripheral, traits::{PeriAddress, Stream, Channel, DMASet}, ChannelX}, rcc::Clocks};
use smart_leds_trait::{SmartLedsWrite, RGB8};
use embedded_hal::PwmPin;
use embedded_dma::Word;
use fugit::ExtU32;


use cortex_m_rt::entry;
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{prelude::*, pac::Peripherals, rcc::RccExt, timer::TimerExt};
use {cortex_m_rt as _, panic_probe as _};



/// ccr - compare (trigger an external event after a predetermined amount of time has expired)/ capture(measure the duration of an event) register









#[entry]
fn main() -> ! {


    rtt_init_print!();

    let led_buf={
        static mut led_buf:[u16;24*8+8]=[0;24*8+8];
        unsafe{&mut led_buf};
    };

    let dp = Peripherals::take().unwrap();

    let gpiob=dp.GPIOB.split();
    let dma1=dp.DMA1.split();

    let ws_pin=gpiob.pb7.into_alternate();

    let mut rcc = dp.RCC.constrain();
    let mut clocks = rcc.cfgr.freeze();
    let mut delay = dp.TIM2.delay_ms(&clocks);

    let mut ws=Ws2812Pwm::new(dp.TIM3,ws_pin,dma1.5,led_buf,&clocks);

    ws.write((0..=8).map(|_|RGB8::new(255,255,255)));





    // let cp = cortex_m::Peripherals::take().unwrap();

    // let gpiob = dp.GPIOB.split();
    // let dma1 = dp.DMA1.split();

    // let led = gpiob.pb7.into_alternate();

    // let mut led = gpiob.pb7.into_push_pull_output();


    // rprintln!("hw");

    // loop {
    //     led.set_high();
    //     delay.delay_ms(400_u32);
    //     led.set_low();
    // }
}