//! LCD controller.
//!
//! **File:** `lcd.rs`  
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
// Import crate::timer::Timer0Real
use crate::timer::Timer0Real;

/// Writes one nibble (4 bits) to the LCD in 4-bit mode.
///
/// Manages the RS bit and the Enable pulse on PORTD, while the upper data
/// nibble is output on PB0–PB1 and the lower data nibble on PD6–PD7.
///
/// # Arguments
///
/// * `rs` - Register Select bit state.
/// * `n` - Nibble or number.
///
/// # Arguments
///
/// * `rs` - Register Select bit state.
/// * `n` - Nibble or number.
fn lcd_nibble(rs: bool, n: u8) {
    let p = (read_reg(0x2B) & 0x0F) | ((if rs { 1u8 } else { 0 }) << 4);
    write_reg(0x2B, p | ((n & 0x03) << 6) | (1 << 5));
    let b = (read_reg(0x25) & 0xFC) | ((n >> 2) & 0x03);
    write_reg(0x25, b);
    write_reg(0x2B, p | ((n & 0x03) << 6));
}

/// Sends a command byte to the LCD (RS = 0, two nibbles).
///
/// # Arguments
///
/// * `c` - Command byte.
///
/// # Arguments
///
/// * `c` - Command byte.
fn lcd_cmd(c: u8) {
    lcd_nibble(false, c >> 4);
    lcd_nibble(false, c & 0x0F);
}

/// Sends a data byte to the LCD (RS = 1, two nibbles).
///
/// # Arguments
///
/// * `d` - The `d` parameter.
///
/// # Arguments
///
/// * `d` - The `d` parameter.
fn lcd_data(d: u8) {
    lcd_nibble(true, d >> 4);
    lcd_nibble(true, d & 0x0F);
}

/// Blocks ~2 ms to allow the LCD to complete its internal operation.
fn lcd_busy() {
    let t = Timer0Real;
    t.delay_ms(2);
}

/// HD44780-based LCD1602 controller (4-bit mode).
pub struct LCD;
impl LCD {
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
        Some(LCD)
    }

    /// Initializes the LCD in 4-bit mode (2 lines, 5x8 font).
    pub fn init(&self) {
        let t = Timer0Real;
        t.delay_ms(50);
        lcd_nibble(false, 3);
        t.delay_ms(5);
        Self::init_pt2(&t);
    }

    /// Part 2 of LCD initialization
    ///
    /// # Arguments
    ///
    /// * `t` - The `t` parameter.
    ///
    /// # Arguments
    ///
    /// * `t` - The `t` parameter.
    fn init_pt2(t: &Timer0Real) {
        lcd_nibble(false, 3);
        t.delay_ms(1);
        lcd_nibble(false, 3);
        t.delay_ms(1);
        Self::init_pt3(t);
    }

    /// Part 3 of LCD initialization
    ///
    /// # Arguments
    ///
    /// * `t` - The `t` parameter.
    ///
    /// # Arguments
    ///
    /// * `t` - The `t` parameter.
    fn init_pt3(t: &Timer0Real) {
        lcd_nibble(false, 2); t.delay_ms(1); lcd_cmd(0x28); lcd_busy(); lcd_cmd(0x0C);
        lcd_busy();
        lcd_cmd(0x06);
        lcd_busy();
        lcd_cmd(0x01);
        lcd_busy();
    }

    /// Clears the display and returns the cursor home.
    pub fn clear_display(&self) {
        lcd_cmd(1);
        lcd_busy();
    }

    /// Writes a string at the current cursor position.
    ///
    /// # Arguments
    ///
    /// * `s` - The `s` parameter.
    ///
    /// # Arguments
    ///
    /// * `s` - The `s` parameter.
    pub fn print(&self, s: &str) {
        for c in s.bytes() {
            self.write_char(c as char);
        }
    }

    /// Writes a single character at the current cursor position.
    ///
    /// # Arguments
    ///
    /// * `c` - Command byte.
    ///
    /// # Arguments
    ///
    /// * `c` - Command byte.
    pub fn write_char(&self, c: char) {
        lcd_data(c as u8);
        lcd_busy();
    }

    /// Writes a `u16` as a decimal string starting at the current cursor position.
    ///
    /// # Arguments
    ///
    /// * `num` - The `num` parameter.
    ///
    /// # Arguments
    ///
    /// * `num` - The `num` parameter.
    pub fn print_number(&self, num: u16) {
        let (b, i) = Self::num_to_bytes(num);
        for c in &b[i..] {
            self.write_char(*c as char);
        }
    }

    /// Converts a number to bytes for LCD display
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
}
