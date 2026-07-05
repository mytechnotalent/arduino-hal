//! Analog-to-Digital Converter.
//!
//! **File:** `adc.rs`  
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

/// ADC input channel selection.
pub enum InputChannel {
    /// Analog pin A0 (PC0).
    AnalogPin0,
    /// Analog pin A1 (PC1).
    AnalogPin1,
    /// Analog pin A2 (PC2).
    AnalogPin2,
    /// Analog pin A3 (PC3).
    AnalogPin3,
    /// Analog pin A4 (PC4).
    AnalogPin4,
    /// Analog pin A5 (PC5).
    AnalogPin5,
}

/// ADC voltage reference selection.
pub enum VoltageReference {
    /// Default (AREF pin, internal Vref turned off).
    Default,
    /// AVcc with external capacitor at AREF pin.
    AVcc,
    /// Internal 1.1 V reference.
    Internal,
    /// External reference applied to AREF pin.
    External,
}

/// ADC clock prescaler selection.
pub enum ADCprescaler {
    /// Divide clock by 2.
    Prescaler2,
    /// Divide clock by 4.
    Prescaler4,
    /// Divide clock by 8.
    Prescaler8,
    /// Divide clock by 16.
    Prescaler16,
    /// Divide clock by 32.
    Prescaler32,
    /// Divide clock by 64.
    Prescaler64,
    /// Divide clock by 128.
    Prescaler128,
}

/// Returns the ADPS[2:0] bit pattern for the given ADC prescaler.
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
pub(crate) fn adc_prescaler_bits(p: &ADCprescaler) -> u8 {
    match p { ADCprescaler::Prescaler2 => 1, ADCprescaler::Prescaler4 => 2, ADCprescaler::Prescaler8 => 3,
        ADCprescaler::Prescaler16 => 4,
        ADCprescaler::Prescaler32 => 5,
        ADCprescaler::Prescaler64 => 6,
        ADCprescaler::Prescaler128 => 7,
    }
}

/// Returns the MUX[2:0] bit pattern for the given input channel.
///
/// # Arguments
///
/// * `ch` - The `ch` parameter.
///
/// # Returns
///
/// An 8-bit unsigned integer value.
///
/// # Arguments
///
/// * `ch` - The `ch` parameter.
///
/// # Returns
///
/// An 8-bit unsigned integer value.
pub(crate) fn adc_mux_bits(ch: &InputChannel) -> u8 {
    match ch { InputChannel::AnalogPin0 => 0, InputChannel::AnalogPin1 => 1,
        InputChannel::AnalogPin2 => 2,
        InputChannel::AnalogPin3 => 3,
        InputChannel::AnalogPin4 => 4,
        InputChannel::AnalogPin5 => 5,
    }
}

/// Returns the REFS[1:0] bit pattern for the given voltage reference.
///
/// # Arguments
///
/// * `r` - The `r` parameter.
///
/// # Returns
///
/// An 8-bit unsigned integer value.
///
/// # Arguments
///
/// * `r` - The `r` parameter.
///
/// # Returns
///
/// An 8-bit unsigned integer value.
pub(crate) fn vref_bits(r: &VoltageReference) -> u8 {
    match r {
        VoltageReference::AVcc => 1,
        VoltageReference::Internal => 3,
        _ => 0,
    }
}

/// Analog-to-Digital Converter controller.
pub struct ADC;
impl ADC {
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
        Some(ADC)
    }

    /// Resets the ADC control register (ADCSRA).
    pub fn start(&self) {
        write_reg(0x7A, 0);
    }

    /// Selects the analog input channel via the ADMUX MUX bits.
    ///
    /// # Arguments
    ///
    /// * `ch` - The `ch` parameter.
    ///
    /// # Arguments
    ///
    /// * `ch` - The `ch` parameter.
    pub fn select_input_channel(&self, ch: InputChannel) {
        write_reg(0x7C, (read_reg(0x7C) & 0xE0) | adc_mux_bits(&ch));
    }

    /// Selects the voltage reference via the ADMUX REFS bits.
    ///
    /// # Arguments
    ///
    /// * `r` - The `r` parameter.
    ///
    /// # Arguments
    ///
    /// * `r` - The `r` parameter.
    pub fn set_reference(&self, r: VoltageReference) {
        write_reg(0x7C, (read_reg(0x7C) & 0x3F) | (vref_bits(&r) << 6));
    }

    /// Starts a conversion and returns the 10-bit result (blocking).
    ///
    /// # Arguments
    ///
    /// * `p` - GPIO pin.
    ///
    /// # Returns
    ///
    /// A 16-bit unsigned integer value.
    ///
    /// # Arguments
    ///
    /// * `p` - GPIO pin.
    ///
    /// # Returns
    ///
    /// A 16-bit unsigned integer value.
    pub fn start_conversion(&self, p: ADCprescaler) -> u16 {
        write_reg(0x7A, (1 << 7) | adc_prescaler_bits(&p) | (1 << 6));
        while read_reg(0x7A) & (1 << 6) != 0 {}
        (read_reg(0x79) as u16) << 8 | read_reg(0x78) as u16
    }
}
