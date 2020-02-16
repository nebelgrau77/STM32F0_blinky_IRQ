// blinking built-in LED on an STM32F030F4P6 board
// with 2 timers and interrupts

#![no_main]
#![no_std]

use panic_halt as _;

use stm32f0xx_hal as hal;

use crate::hal::{
    gpio::*,
    prelude::*,
    stm32::{interrupt, Interrupt, Peripherals, TIM3, TIM14},
    time::Hertz,
    timers::*,
};

use cortex_m_rt::entry;

use core::cell::RefCell;
use cortex_m::{interrupt::Mutex, peripheral::Peripherals as c_m_Peripherals};

// A type definition for the GPIO pin to be used for our LED
// Using built-in LED on PA4
type LEDPIN1 = gpioa::PA4<Output<PushPull>>;
type LEDPIN2 = gpioa::PA0<Output<PushPull>>;

// Make LED pin globally available
static GLED1: Mutex<RefCell<Option<LEDPIN1>>> = Mutex::new(RefCell::new(None));
static GLED2: Mutex<RefCell<Option<LEDPIN2>>> = Mutex::new(RefCell::new(None));

// Make timer interrupt registers globally available
static GINT1: Mutex<RefCell<Option<Timer<TIM3>>>> = Mutex::new(RefCell::new(None));
static GINT2: Mutex<RefCell<Option<Timer<TIM14>>>> = Mutex::new(RefCell::new(None));

// Define an interupt handler, i.e. function to call when interrupt occurs. Here if our external
// interrupt trips when the timer timed out
#[interrupt]
fn TIM3() {
    static mut LED1: Option<LEDPIN1> = None;
    static mut INT1: Option<Timer<TIM3>> = None;

    let led1 = LED1.get_or_insert_with(|| {
        cortex_m::interrupt::free(|cs| {
            // Move LED pin here, leaving a None in its place
            GLED1.borrow(cs).replace(None).unwrap()
        })
    });

    let int1 = INT1.get_or_insert_with(|| {
        cortex_m::interrupt::free(|cs| {
            // Move LED pin here, leaving a None in its place
            GINT1.borrow(cs).replace(None).unwrap()
        })
    });

    led1.toggle().ok();
    int1.wait().ok();
}

#[interrupt]
fn TIM14() {
    static mut LED2: Option<LEDPIN2> = None;
    static mut INT2: Option<Timer<TIM14>> = None;

    let led2 = LED2.get_or_insert_with(|| {
        cortex_m::interrupt::free(|cs| {
            // Move LED pin here, leaving a None in its place
            GLED2.borrow(cs).replace(None).unwrap()
        })
    });

    let int2 = INT2.get_or_insert_with(|| {
        cortex_m::interrupt::free(|cs| {
            // Move LED pin here, leaving a None in its place
            GINT2.borrow(cs).replace(None).unwrap()
        })
    });

    led2.toggle().ok();
    int2.wait().ok();
}


#[entry]
fn main() -> ! {
    if let (Some(mut p), Some(cp)) = (Peripherals::take(), c_m_Peripherals::take()) {
        cortex_m::interrupt::free(move |cs| {
            let mut rcc = p.RCC.configure().sysclk(8.mhz()).freeze(&mut p.FLASH);


            let gpioa = p.GPIOA.split(&mut rcc);

            // (Re-)configure PA4 as output
            let led1 = gpioa.pa4.into_push_pull_output(cs);

            // Move the pin into our global storage
            *GLED1.borrow(cs).borrow_mut() = Some(led1);


            // (Re-)configure PA0 as output
            let led2 = gpioa.pa0.into_push_pull_output(cs);

            // Move the pin into our global storage
            *GLED2.borrow(cs).borrow_mut() = Some(led2);

            // Set up a timer expiring after 100ms
            let mut timer1 = Timer::tim3(p.TIM3, Hertz(5), &mut rcc);

            // Set up a timer expiring after 500ms
            let mut timer2 = Timer::tim14(p.TIM14, Hertz(2), &mut rcc);

            // Generate an interrupt when the timer expires
            timer1.listen(Event::TimeOut);

            // Generate an interrupt when the timer expires
            timer2.listen(Event::TimeOut);

            // Move the timer into our global storage
            *GINT1.borrow(cs).borrow_mut() = Some(timer1);

            // Move the timer into our global storage
            *GINT2.borrow(cs).borrow_mut() = Some(timer2);

            // Enable TIM3 IRQ, set prio 1 and clear any pending IRQs
            // Enable TIM14 IRQ, set prio 2 and clear any pending IRQs
            let mut nvic = cp.NVIC;
            unsafe {
                nvic.set_priority(Interrupt::TIM3, 1);
                cortex_m::peripheral::NVIC::unmask(Interrupt::TIM3);

                nvic.set_priority(Interrupt::TIM14, 2);
                cortex_m::peripheral::NVIC::unmask(Interrupt::TIM14);
            }
            cortex_m::peripheral::NVIC::unpend(Interrupt::TIM3);
            cortex_m::peripheral::NVIC::unpend(Interrupt::TIM14);
            
            
            

        });
    }

    loop {
        continue;
    }
}