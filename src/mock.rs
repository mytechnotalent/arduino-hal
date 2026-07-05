//! Simulated AVR register file for host-side unit testing.
//!
//! **File:** `mock.rs`  
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

#![cfg(test)]

extern crate std;

// Import std::cell::RefCell
use std::cell::RefCell;

thread_local! {
    /// The backing store — 256 bytes, one per I/O register address.
    static REGS: RefCell<[u8; 256]> = const { RefCell::new([0; 256]) };
}

/// Clears all simulated registers back to zero.
pub fn reset() {
    REGS.with(|regs| *regs.borrow_mut() = [0; 256]);
}

/// Writes `val` to the simulated register at `addr`.
///
/// # Arguments
///
/// * `addr` - Device address.
/// * `val` - Value to use.
///
/// # Arguments
///
/// * `addr` - Device address.
/// * `val` - Value to use.
pub fn write_reg(addr: u16, val: u8) {
    REGS.with(|regs| {
        let mut r = regs.borrow_mut();
        r[addr as usize] = val;
    });
}

/// Reads the simulated register value at `addr`.
///
/// # Arguments
///
/// * `addr` - Device address.
///
/// # Returns
///
/// An 8-bit unsigned integer value.
///
/// # Arguments
///
/// * `addr` - Device address.
///
/// # Returns
///
/// An 8-bit unsigned integer value.
pub fn read_reg(addr: u16) -> u8 {
    REGS.with(|regs| {
        let mut r = regs.borrow_mut();
        if addr == 0x46 { r[0x46] = r[0x46].saturating_add(50); }
        if addr == 0xC0 { r[0xC0] |= 1 << 5; }
        if addr == 0x7A { r[0x7A] &= !(1 << 6); }
        r[addr as usize]
    })
}
