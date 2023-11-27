Current issues that need fixing:
    - the buzzer works but led doesnt? could be because the loop doesnt encode 24 bits for each colour properly and instead always updates 1 by 1
    - when setting pwm; dp.TIM4.pwm_hz[0] is pinS and not pin, therefore difficulties setting 1 pin arise, might also be whats causing led pwm not to work
    - pwm led for loop is currently very unoptimised and takes up a lot of memory


ToDo:
    - make pwm without pwm.rs use written dma func and not syst/spi clocks
    - dont need transfer??
    - generate 2 square waves, logic1/0 and reset delay


Reminders:
    dma:
        - arr - auto reload register - these are the TIMs which have channels; responsible for interrupts - https://ziblog.ru/2011/01/15/stm32-chast-8-ndash-taymeryi-obshhego-naznacheniya-preryivaniya.html
        - pre-scalar - cuts the frequency of timer ticks - TIM1 & TIM8, while 2-5 are general-purpose timers; others are basic
        - stm32 metapac - crate for working with dma stream mappings, interrupts, etc - https://crates.io/crates/stm32-metapac

    pwm:
        - cce??

    ws2812-pwm-dma crate which this was initially based of doesnt currently work because of crate conflicts and bugs (bivtec 1.0.1 -> funty -1.1.0 but  funy is 2.0.0 and doesnt let earlier version get added (still doesnt fix the issue))

    stm32f3xx-hal has a ::pwm, can study further what theyve done (does seem to use sys clock thou)