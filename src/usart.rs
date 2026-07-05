//! USART peripheral.
//!
//! **File:** `usart.rs`  
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
#[allow(unused_macros)]

/// Macro for generating documented enums.
macro_rules! doc_enum {
    ($(#[$m:meta])* pub enum $n:ident { $($v:ident),+ $(,)? }) => {
        $(#[$m])*
        pub enum $n { $( #[doc = stringify!($v)] $v, )+ }
    };
}

/// USART0 operating mode selection.
pub enum Mode {
    /// Double-speed asynchronous mode (U2X = 1).
    DoubleSpeedAsynchronous,
}

/// Universal Synchronous/Asynchronous Receiver/Transmitter 0.
pub struct USART0;
impl USART0 {
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
        Some(USART0)
    }

    /// Enables the USART0 peripheral clock by clearing the relevant PRR bit.
    pub fn start(&self) {
        write_reg(0x64, read_reg(0x64) & !(1 << 1));
    }

    /// Calculates and writes the baud-rate register for the requested baud.
    ///
    /// Uses the double-speed formula: `UBRR = F_CPU / (8 × baud) - 1`.
    ///
    /// # Arguments
    ///
    /// * `baud` - The `baud` parameter.
    /// * `_m` - The `_m` parameter.
    ///
    /// # Arguments
    ///
    /// * `baud` - The `baud` parameter.
    /// * `m` - The `m` parameter.
    pub fn set_baud_rate(&self, baud: u32, _m: Mode) {
        let u = (16_000_000 / (8 * baud)) as u16 - 1;
        write_reg(0xC5, (u >> 8) as u8);
        write_reg(0xC4, u as u8);
        write_reg(0xC0, 1 << 1);
    }

    /// Configures 8N1 frame format (8 data bits, no parity, 1 stop bit).
    pub fn set_frame_format(&self) {
        write_reg(0xC2, 0x06);
    }

    /// Enables both the transmitter and the receiver.
    pub fn enable_tx_rx(&self) {
        write_reg(0xC1, (1 << 4) | (1 << 3));
    }

    /// Transmits a single byte over USART0, blocking until the data register is empty.
    ///
    /// # Arguments
    ///
    /// * `c` - Command byte.
    ///
    /// # Arguments
    ///
    /// * `c` - Command byte.
    pub fn transmit_char(&self, c: char) {
        while read_reg(0xC0) & (1 << 5) == 0 {}
        write_reg(0xC6, c as u8);
    }

    /// Prints a string slice over USART0.
    ///
    /// # Arguments
    ///
    /// * `s` - The `s` parameter.
    ///
    /// # Arguments
    ///
    /// * `s` - The `s` parameter.
    pub fn print_string(&self, s: &str) {
        for c in s.bytes() {
            self.transmit_char(c as char);
        }
    }

    /// Prints a `u16` as a decimal string over USART0.
    ///
    /// # Arguments
    ///
    /// * `num` - The `num` parameter.
    ///
    /// # Arguments
    ///
    /// * `num` - The `num` parameter.
    pub fn print_num(&self, num: u16) {
        let (b, i) = Self::num_to_bytes(num);
        for c in &b[i..] {
            self.transmit_char(*c as char);
        }
    }

    /// Converts a number to bytes for USART transmission
    ///
    /// # Arguments
    ///
    /// * `n` - Nibble or number.
    ///
    /// # Returns
    ///
    /// A value of type `([u8; 5], usize)`.
    ///
    /// # Arguments
    ///
    /// * `n` - Nibble or number.
    ///
    /// # Returns
    ///
    /// A value of type `([u8; 5], usize)`.
    fn num_to_bytes(mut n: u16) -> ([u8; 5], usize) {
        let mut b = [0u8; 5]; let mut i = 5; loop { i -= 1; b[i] = (n % 10) as u8 + b'0'; n /= 10;
            if n == 0 {
                break;
            }
        }
        (b, i)
    }

    /// Reads one byte from USART0 (non-blocking, returns 0 when no data).
    ///
    /// # Returns
    ///
    /// An 8-bit unsigned integer value.
    ///
    /// # Returns
    ///
    /// An 8-bit unsigned integer value.
    pub fn usart_receive(&self) -> u8 {
        if read_reg(0xC0) & (1 << 7) == 0 {
            return 0;
        }
        read_reg(0xC6)
    }
}
