//! Build script to configure linker memory regions for the AVR target.
//!
//! **File:** `build.rs`  
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

/// The main entry point.
fn main() {
    let target = std::env::var("TARGET").unwrap_or_default();
    if target.contains("avr") {
        println!("cargo:rustc-link-arg=-Wl,--defsym=__TEXT_REGION_LENGTH__=32K");
        println!("cargo:rustc-link-arg=-Wl,--defsym=__DATA_REGION_LENGTH__=0x800");
        println!("cargo:rustc-link-arg=-Wl,--defsym=__EEPROM_REGION_LENGTH__=1K");
    }
}
// Core logic

#[cfg(test)]
mod tests {
    // Import all parent module items
    use super::*;
}
