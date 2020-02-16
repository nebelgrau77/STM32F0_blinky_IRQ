Blinking an LED with 2 timers and IRQs.
Modified example from https://github.com/stm32-rs/stm32f0xx-hal/blob/master/examples/blinky_timer_irq.rs 
ported to the cheap STMF030F4P6 board. 
Two timers used instead of one, to blink two LEDs at different frequency.

Possible use case: one timer to update number of seconds elapsed, other one to update a display, etc.
