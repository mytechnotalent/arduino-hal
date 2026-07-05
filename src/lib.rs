#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]

//! Register-level HAL for the ATmega328P (Arduino Uno R3).
//!
//! Provides safe wrappers around AVR I/O registers for GPIO (PortB/C/D),
//! USART0, Timer0, Timer1, ADC, and HD44780-based LCD1602.
//!
//! Contains exactly two `unsafe` blocks (in `write_reg` and `read_reg`).
//! All public functions and methods are safe wrappers.
//!
//! **File:** `lib.rs`  
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

pub(crate) mod io;
#[cfg(test)]
pub(crate) mod mock;

pub mod adc;
pub mod board;
pub mod gpio;
pub mod lcd;
pub mod timer;
pub mod usart;

#[cfg(test)]
mod tests;
