//! Timer peripherals.
//!
//! **File:** `timer.rs`  
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

// Import dependencies from crate::io
use crate::io::{read_reg, write_reg};

/// Timer clock prescaler selection.
pub enum Prescaler {
    /// Divide clock by 8.
    Prescaler8,
    /// Divide clock by 64.
    Prescaler64,
    /// Divide clock by 256.
    Prescaler256,
    /// Divide clock by 1024.
    Prescaler1024,
}

/// Returns the TCCRxB CS[2:0] bit pattern for the given prescaler.
///
/// # Arguments
///
/// * `p` - GPIO pin.
///
/// # Returns
///
/// An 8-bit unsigned integer value.
///
/// # Arguments
///
/// * `p` - GPIO pin.
///
/// # Returns
///
/// An 8-bit unsigned integer value.
pub(crate) fn prescaler_bits(p: &Prescaler) -> u8 {
    match p {
        Prescaler::Prescaler8 => 0x02,
        Prescaler::Prescaler64 => 0x03,
        Prescaler::Prescaler256 => 0x04,
        Prescaler::Prescaler1024 => 0x05,
    }
}

/// Timer0 (8-bit) controller.
pub struct Timer0Real;
impl Timer0Real {
    /// Returns a singleton instance.
    ///
    /// # Returns
    ///
    /// A new instance of the struct.
    ///
    /// # Returns
    ///
    /// A new instance of the struct.
    pub fn take() -> Option<Self> {
        Some(Timer0Real)
    }

    /// Enables the Timer0 peripheral clock by clearing the relevant PRR bit.
    pub fn start(&self) {
        write_reg(0x64, read_reg(0x64) & !(1 << 4));
    }

    /// Blocks for approximately `ms` milliseconds using Timer0 with prescaler 64.
    ///
    /// # Arguments
    ///
    /// * `ms` - The `ms` parameter.
    ///
    /// # Arguments
    ///
    /// * `ms` - The `ms` parameter.
    pub fn delay_ms(&self, ms: u16) {
        write_reg(0x45, 0x03);
        for _ in 0..ms {
            write_reg(0x46, 0);
            while read_reg(0x46) < 250 {}
        }
        write_reg(0x45, 0);
    }

    /// Selects normal mode (no waveform generation).
    pub fn set_normal_mode(&self) {
        write_reg(0x44, 0x00);
    }

    /// Selects CTC (Clear Timer on Compare Match) mode.
    pub fn set_ctc_mode(&self) {
        write_reg(0x44, 0x02);
    }

    /// Selects Fast PWM mode with the given top value.
    ///
    /// # Arguments
    ///
    /// * `top` - The `top` parameter.
    ///
    /// # Arguments
    ///
    /// * `top` - The `top` parameter.
    pub fn set_fast_pwm(&self, top: u16) {
        write_reg(0x44, 0x03);
        write_reg(0x48, top as u8);
    }
}

/// Timer1 (16-bit) controller.
pub struct Timer1Real;
impl Timer1Real {
    /// Returns a singleton instance.
    ///
    /// # Returns
    ///
    /// A new instance of the struct.
    ///
    /// # Returns
    ///
    /// A new instance of the struct.
    pub fn take() -> Option<Self> {
        Some(Timer1Real)
    }

    /// Enables the Timer1 peripheral clock by clearing the relevant PRR bit.
    pub fn start(&self) {
        write_reg(0x64, read_reg(0x64) & !(1 << 3));
    }

    /// Selects normal mode (no waveform generation).
    pub fn set_normal_mode(&self) {
        write_reg(0x80, 0x00);
        write_reg(0x81, 0x00);
    }

    /// Selects CTC (Clear Timer on Compare Match) mode (WGM13:0 = 4).
    pub fn set_ctc_mode(&self) {
        write_reg(0x80, 0x00);
        write_reg(0x81, 0x08);
    }

    /// Selects Fast PWM mode with ICR1 as the top value (WGM13:0 = 14).
    ///
    /// # Arguments
    ///
    /// * `top` - The `top` parameter.
    ///
    /// # Arguments
    ///
    /// * `top` - The `top` parameter.
    pub fn set_fast_pwm(&self, top: u16) {
        write_reg(0x80, 0x82);
        write_reg(0x81, 0x18);
        write_reg(0x87, (top >> 8) as u8);
        write_reg(0x86, top as u8);
    }

    /// Sets the clock prescaler (CS[2:0] bits of TCCR1B).
    ///
    /// # Arguments
    ///
    /// * `p` - GPIO pin.
    ///
    /// # Arguments
    ///
    /// * `p` - GPIO pin.
    pub fn set_prescaler(&self, p: Prescaler) {
        write_reg(0x81, (read_reg(0x81) & 0xF8) | prescaler_bits(&p));
    }

    /// Sets the OCR1A top value (for CTC mode).
    ///
    /// # Arguments
    ///
    /// * `top` - The `top` parameter.
    ///
    /// # Arguments
    ///
    /// * `top` - The `top` parameter.
    pub fn set_top_value(&self, top: u16) {
        write_reg(0x89, (top >> 8) as u8);
        write_reg(0x88, top as u8);
    }

    /// Blocks until an OCR1A compare match occurs.
    pub fn wait_for_match(&self) {
        while read_reg(0x36) & (1 << 1) == 0 {}
        write_reg(0x36, 1 << 1);
    }

    /// Blocks until Timer1 overflow occurs.
    pub fn wait(&self) {
        while read_reg(0x36) & (1 << 0) == 0 {}
        write_reg(0x36, 1 << 0);
    }

    /// Sets the PWM duty cycle via OCR1A.
    ///
    /// # Arguments
    ///
    /// * `duty` - The `duty` parameter.
    ///
    /// # Arguments
    ///
    /// * `duty` - The `duty` parameter.
    pub fn set_duty_cycle(&self, duty: u16) {
        write_reg(0x89, (duty >> 8) as u8);
        write_reg(0x88, duty as u8);
    }
}
