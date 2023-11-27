#![no_std]
#![no_main]

/// additional imports
/// bitvec = "1.0.1"
/// embedded-dma = "0.2.0"
/// fugit = "0.3.7"
/// smart-leds-trait = "0.2.1"


use cortex_m_rt::entry;
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{prelude::*, pac::{Peripherals, self, dma1, rcc::apb2lpenr::{ADC1LPEN_A, ADC1LPEN_R}, crc::cr::RESET_AW}, rcc::RccExt, timer::{TimerExt, Channel, Channel1, Channel2, Channel3}, i2c::DutyCycle};
use {cortex_m_rt as _, panic_probe as _};

use stm32f4xx_hal::dma::Transfer;

pub mod dma;
pub mod pwm;
use dma::DmaCcrTimer;
use pwm::*;


/// ccr - compare (trigger an external event after a predetermined amount of time has expired)/ capture(measure the duration of an event) register



#[entry]
fn main() -> ! {

    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();

    let gpiob=dp.GPIOB.split();
/*
    //let buzz = gpiob.pb7.into_alternate();
   
    let channels = (Channel2::new(gpiob.pb7), Channel3::new(gpiob.pb8));


    let mut pwm = dp.TIM4.pwm_hz(channels, 2000.Hz(), &clocks);

    let max_duty = pwm.get_max_duty();
    pwm.set_duty(Channel::C2, max_duty / 2);


    let mut delay = dp.TIM2.delay_ms(&clocks);
*/

    /// another block for when using pwm.rs
    let led_buf={
        static mut led_buf:[u16;24*8+8]=[0;24*8+8];
        unsafe{&mut led_buf};
    };

    let dma1=dp.DMA1.split();
    
    let ws_pin=gpiob.pb7.into_alternate();
    
    // uses stream channel 2 and Timer channel 4
    let mut ws=Ws2812Pwm::new(dp.TIM4,ws_pin,dma1.2,led_buf,&clocks);
    
    ws.write((0..=8).map(|_|RGB8::new(255,255,255)));

        loop {
        cortex_m::asm::nop();
    }


/*
    /// frequencies of highs, lows & reset
    let bits =[
        ('h', 2500000.Hz()), // T0H
        ('l', 1176471.Hz()), // T0L
        ('H', 1250000.Hz()), // T1H
        ('L', 2222222.Hz()), // T1L
    ];

    /// single logic 1 & 0 encoded
    let z = [
        ('h', 1),
        ('l', 1),
    ]; // logic 0 

    let o = [
        ('H', 1),
        ('L', 1),
    ]; // logic 1


    /// single led colour
    let green = [
        ('o', 8),
        ('z', 8),
        ('z', 8),
    ];

    let red = [
        ('z', 8),
        ('o', 8),
        ('z', 8),
    ];

    let blue = [
        ('z', 8),
        ('z', 8),
        ('o', 8),
    ];

    /// here encoded to LED
    let colours = [
        ('g', 1),
        ('r', 1),
        ('b', 2),
        ('g', 1),
        ('r', 1),
        ('b', 2),
        (' ', 4),
    ];

    let rst = 55_u32;
    let duty_cycle = 2_u32;

    loop {

        for col in colours {
            for bit in bits {
                for zs in z {
                    for os in o {
                        if col.0 == 'g' {
                            for gr in green {
                                if gr.0 == 'o' {
                                    if os.0 == bit.0 {
                                        pwm.set_period(bit.1);
                                        pwm.enable(Channel::C2);
                                    }
                                    delay.delay_us(gr.1 * duty_cycle);
                                } else if gr.0 == 'z' {
                                    if zs.0 == bit.0 {
                                        pwm.set_period(bit.1);
                                        pwm.enable(Channel::C2);
                                    }
                                    delay.delay_us(gr.1 * duty_cycle);
                                }
                            }
                            delay.delay_us(col.1 * duty_cycle * 24);    
                        } else if col.0 == 'r' {
                            for re in red {
                                if re.0 == 'o' {
                                    if os.0 == bit.0 {
                                        pwm.set_period(bit.1);
                                        pwm.enable(Channel::C2);
                                    }
                                    delay.delay_us(re.1 * duty_cycle);
                                } else if re.0 == 'z' {
                                    if zs.0 == bit.0 {
                                        pwm.set_period(bit.1);
                                        pwm.enable(Channel::C2);
                                    }
                                    delay.delay_us(re.1 * duty_cycle);
                                }
                            }
                            delay.delay_us(col.1 * duty_cycle * 24); 
                        } else if col.0 == 'b' {
                            for bl in blue {
                                if bl.0 == 'o' {
                                    if os.0 == bit.0 {
                                        pwm.set_period(bit.1);
                                        pwm.enable(Channel::C2);
                                    }
                                    delay.delay_us(bl.1 * duty_cycle);
                                } else if bl.0 == 'z' {
                                    if zs.0 == bit.0 {
                                        pwm.set_period(bit.1);
                                        pwm.enable(Channel::C2);
                                    }
                                    delay.delay_us(bl.1 * duty_cycle);
                                }
                            }
                            delay.delay_us(col.1 * duty_cycle * 24); 
                        } else if col.0 == ' ' {
                            pwm.disable(Channel::C2);
                            delay.delay_us(rst);
                        }
                    }
                }
            }
            // pwm.disable(Channel::C2);
            // delay.delay_ms(rst);
        }
    }
*/


/*
let tones = [
    ('c', 261.Hz()),
    ('d', 294.Hz()),
    ('e', 329.Hz()),
    ('f', 349.Hz()),
    ('g', 392.Hz()),
    ('a', 440.Hz()),
    ('b', 493.Hz()),
];


let tune = [
    ('c', 1),
    ('c', 1),
    ('g', 1),
    ('g', 1),
    ('a', 1),
    ('a', 1),
    ('g', 2),
    ('f', 1),
    ('f', 1),
    ('e', 1),
    ('e', 1),
    ('d', 1),
    ('d', 1),
    ('c', 2),
    (' ', 4),
];

let tempo = 300_u32;

loop {
    for note in tune {
        for tone in tones {
            if tone.0 == note.0 {
                pwm.set_period(tone.1);
                pwm.enable(Channel::C2);
                delay.delay_ms(note.1 * tempo);
            } else if note.0 == ' ' {
                pwm.disable(Channel::C2);
                delay.delay_ms(tempo);
            }
        }
        pwm.disable(Channel::C2);
        delay.delay_ms(tempo / 2);
    }
}

*/






    // if let Some(dp) = Peripherals::take() {

    //     ///clock
    //     let mut rcc = dp.RCC.constrain();
    //     let mut clocks = rcc.cfgr.freeze();
    //     let mut delay = dp.TIM2.delay_ms(&clocks);
 
    //     /// set pin and dma
    //     let gpiob=dp.GPIOB.split();
    //     let gpioa = dp.GPIOA.split();
    //     let channels = (Channel2::new(gpiob.pb7), Channel3::new(gpiob.pb8));


    //     /// set pin behavior
    //     // let mut led=gpiob.pb7.into_push_pull_output();
    //     // let wub=gpiob.pb7.into_alternate();

    //     ///channel 4 and timer chan 2
    //     let pwm = dp.TIM4.pwm_hz(channels, 1.Hz(), &clocks).split();
    //     let (mut ch2, _ch3) = pwm;
    //     let max_duty = ch2.get_max_duty();
    //     ch2.set_duty(max_duty / 2);
    //     ch2.enable();


    //     // reset leds cuz no clue which one works when otherwise
    //     // let mut led=gpiob.pb8.into_push_pull_output();

    //     // loop {
    //     //     led.toggle();
    //     //     delay.delay_us(80_u32);
    //     // }
    // }
    // // rtt_init_print!();
    // // rprintln!("hw");
    // loop {
    //     cortex_m::asm::nop();
    // }

}