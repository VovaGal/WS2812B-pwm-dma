use stm32f4xx_hal::pac;


/// ccr - compare (trigger an external event after a predetermined amount of time has expired)/ capture(measure the duration of an event) register
/// stm32f411 uses timers 1-5 for ch4 and channels 1,2,4
/// pin b7 uses tim4 ch 2
/// pin b8 uses tim 4 ch 3


pub trait DmaCcrTimer<const TIM_CHAN:u8>{
    fn enable_dma();
    fn disable_dma();
}

macro_rules! with_dma_int{
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
    pac::TIM4: CH2
);