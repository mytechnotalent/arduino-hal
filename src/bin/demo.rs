//! Demo binary for the Elegoo Uno R3.
//!
//! **File:** `demo.rs`  
//! **Author:** Kevin Thomas  
//! **Date:** 2026  
//!
//! MIT License
//!
//! Copyright (c) 2026 Kevin Thomas
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in
//! all copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.

#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![allow(clippy::empty_loop)]

// Import arduino_hal::board::board_init
use arduino_hal::board::board_init;
// Import arduino_hal::gpio::PinB
use arduino_hal::gpio::PinB;
// Import arduino_hal::gpio::PortB
use arduino_hal::gpio::PortB;
// Import arduino_hal::timer::Timer0Real
use arduino_hal::timer::Timer0Real;
// Import arduino_hal::usart::Mode
use arduino_hal::usart::Mode;
// Import arduino_hal::usart::USART0
use arduino_hal::usart::USART0;

/// Initializes the board peripherals.
///
/// # Returns
///
/// A tuple containing instances of PortB, Timer0Real, and USART0.
pub fn init_peripherals() -> (PortB, Timer0Real, USART0) {
    board_init();
    let pb = PortB::take().unwrap();
    let t0 = Timer0Real::take().unwrap();
    let u0 = USART0::take().unwrap();
    (pb, t0, u0)
}

/// Configures the initialized peripherals.
///
/// # Arguments
///
/// * `pb` - Reference to the PortB peripheral.
/// * `t0` - Reference to the Timer0Real peripheral.
/// * `u0` - Reference to the USART0 peripheral.
pub fn configure(pb: &PortB, t0: &Timer0Real, u0: &USART0) {
    pb.set_output(PinB::PB5);
    t0.start();
    u0.start();
    u0.set_baud_rate(9600, Mode::DoubleSpeedAsynchronous);
    u0.set_frame_format();
    u0.enable_tx_rx();
    t0.delay_ms(500);
}

/// Performs a single demo step.
///
/// # Arguments
///
/// * `pb` - Reference to the PortB peripheral.
/// * `t0` - Reference to the Timer0Real peripheral.
/// * `u0` - Reference to the USART0 peripheral.
pub fn step(pb: &PortB, t0: &Timer0Real, u0: &USART0) {
    pb.toggle(PinB::PB5);
    u0.print_string("Hi\r\n");
    t0.delay_ms(500);
}

/// Executes the main demo loop.
///
/// # Arguments
///
/// * `it` - The number of iterations to run, where zero implies an infinite loop.
pub fn run_demo(mut it: u16) {
    let (pb, t0, u0) = init_peripherals();
    configure(&pb, &t0, &u0);
    while it != 1 {
        step(&pb, &t0, &u0);
        it = it.saturating_sub(1);
    }
}

/// The main entry point for the demo program.
///
/// # Returns
///
/// A value of type `!`.
#[cfg_attr(not(test), no_mangle)]
#[allow(dead_code)]
pub extern "C" fn main() -> ! {
    run_demo(0);
    loop {}
}

/// The panic handler, diverging infinitely upon panic.
///
/// # Arguments
///
/// * `_info` - The panic information structure.
///
/// # Returns
///
/// A value of type `!`.
#[cfg(not(test))]
#[panic_handler]
#[allow(dead_code)]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
