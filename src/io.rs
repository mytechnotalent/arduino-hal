//! Low-level register access.
//!
//! **File:** `io.rs`  
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

/// Writes one byte to a memory-mapped I/O register.
///
/// This is one of exactly two `unsafe` blocks in the crate.
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
#[cfg(not(test))]
pub(crate) fn write_reg(addr: u16, val: u8) {
    unsafe { core::ptr::write_volatile(addr as *mut u8, val) }
}

/// Reads one byte from a memory-mapped I/O register.
///
/// This is one of exactly two `unsafe` blocks in the crate.
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
#[cfg(not(test))]
pub(crate) fn read_reg(addr: u16) -> u8 {
    unsafe { core::ptr::read_volatile(addr as *const u8) }
}

/// Test-mode `write_reg`: delegates to the mock register array.
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
#[cfg(test)]
pub(crate) fn write_reg(addr: u16, val: u8) {
    crate::mock::write_reg(addr, val);
}

/// Reads a value from a hardware register.
///
/// # Arguments
///
/// * `addr` - Device address.
///
/// # Returns
///
/// An 8-bit unsigned integer value.
#[cfg(test)]
pub(crate) fn read_reg(addr: u16) -> u8 {
    crate::mock::read_reg(addr)
}
