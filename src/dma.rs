use core::marker::PhantomData;
use stm32f4xx_hal::pac;
//use stm32f4xx_hal::dma::Transfer;
use stm32f4xx_hal::dma::{MemoryToPeripheral, traits::{PeriAddress, Stream}};

/// ccr - compare (trigger an external event after a predetermined amount of time has expired)/ capture(measure the duration of an event) register
/// stm32f411 uses timers 1-5 for ch4 and channels 1,2,4
/// pin b7 uses tim4 ch 2
/// pin b8 uses tim 4 ch 3
/// 
/// 
/// 


pub trait DmaCcrTimer<const TIM_CHAN:u8>{
    fn enable_dma();
    fn disable_dma();
}

macro_rules! with_dma_int{
    ($TIM:ty, CH4)=>{
        impl<const TIM_CHAN:u8> DmaCcrTimer<TIM_CHAN> for $TIM{
            fn enable_dma(){
                let tim=unsafe{ &*<$TIM>::ptr() };
                match TIM_CHAN{
                    0=>{
                        tim.dier.modify(|_,w| w.cc1de().set_bit());
                    },
                    1=>{
                        tim.dier.modify(|_,w| w.cc2de().set_bit());
                    },
                    2=>{
                        tim.dier.modify(|_,w| w.cc3de().set_bit());
                    },
                    3=>{
                        tim.dier.modify(|_,w| w.cc4de().set_bit());
                    },
                    _=>unimplemented!()
                }
            }
            fn disable_dma(){
                let tim=unsafe{ &*<$TIM>::ptr() };
                match TIM_CHAN{
                    0=>{
                        tim.dier.modify(|_,w| w.cc1de().clear_bit());
                    },
                    1=>{
                        tim.dier.modify(|_,w| w.cc2de().clear_bit());
                    },
                    2=>{
                        tim.dier.modify(|_,w| w.cc3de().clear_bit());
                    },
                    3=>{
                        tim.dier.modify(|_,w| w.cc4de().clear_bit());
                    },
                    _=>unimplemented!()
                }
            }
        }
    };

        ($TIM:ty, CH2)=>{
        impl<const TIM_CHAN:u8> DmaCcrTimer<TIM_CHAN> for $TIM{
            fn enable_dma(){
                let tim=unsafe{ &*<$TIM>::ptr() };
                match TIM_CHAN{
                    0=>{
                        tim.dier.modify(|_,w| w.cc1de().set_bit());
                    },
                    1=>{
                        tim.dier.modify(|_,w| w.cc2de().set_bit());
                    },
                    _=>unimplemented!()
                }
            }
            fn disable_dma(){
                let tim=unsafe{ &*<$TIM>::ptr() };
                match TIM_CHAN{
                    0=>{
                        tim.dier.modify(|_,w| w.cc1de().clear_bit());
                    },
                    1=>{
                        tim.dier.modify(|_,w| w.cc2de().clear_bit());
                    },
                    _=>unimplemented!()
                }
            }
        }
    };
}

macro_rules! with_dma {
    ($($TIM:ty: $T:ident,)+) => {
        $(with_dma_int!{$TIM, $T})+
    };
}

with_dma!(
    pac::TIM1: CH4,
    pac::TIM5: CH4,
    pac::TIM4: CH2,
);



// dont really need below as can import ...hal::dma, here for later need

// impl<STREAM, const CHANNEL: u8, PERIPHERAL, BUF>
//     Transfer<STREAM, CHANNEL, PERIPHERAL, PeripheralToMemory, BUF>
// where
//     STREAM: Stream,
//     ChannelX<CHANNEL>: Channel,
//     PERIPHERAL: PeriAddress + DMASet<STREAM, CHANNEL, PeripheralToMemory> + SafePeripheralRead,
//     BUF: WriteBuffer<Word = <PERIPHERAL as PeriAddress>::MemSize>,
// {
//     /// Access the owned peripheral for reading
//     pub fn peripheral(&self) -> &PERIPHERAL {
//         &self.peripheral
//     }
// }







