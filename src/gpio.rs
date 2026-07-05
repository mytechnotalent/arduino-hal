//! General Purpose I/O.
//!
//! **File:** `gpio.rs`  
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

/// Macro for generating pin enums.
macro_rules! pin_enum {
    ($n:ident, $($v:ident),+) => {
        #[repr(u8)]
        #[derive(Clone, Copy)]
        #[doc = concat!("Pin identifiers for ", stringify!($n), ".")]
        pub enum $n { $( #[doc = stringify!($v)] $v, )+ }
    };
}
pin_enum!(PinB, PB0, PB1, PB2, PB3, PB4, PB5);
pin_enum!(PinC, PC0, PC1, PC2, PC3, PC4, PC5);
pin_enum!(PinD, PD0, PD1, PD2, PD3, PD4, PD5, PD6, PD7);

#[allow(unused_macros)]
/// Macro for generating port structures (currently unused but preserved).
macro_rules! port {
    ($n:ident, $p:ty, $pin:expr, $ddr:expr, $port:expr, $doc:expr) => {
        #[doc = $doc]
        pub struct $n;
        impl $n {
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
                Some($n)
            }

            /// Configures a pin as a push-pull output.
            ///
            /// # Arguments
            ///
            /// * `pin` - GPIO pin.
            ///
            /// # Arguments
            ///
            /// * `pin` - GPIO pin.
            pub fn set_output(&self, pin: $p) {
                write_reg($ddr, read_reg($ddr) | (1 << pin as u8));
            }

            /// Configures a pin as an input, optionally with pull-up.
            ///
            /// # Arguments
            ///
            /// * `pin` - GPIO pin.
            /// * `pullup` - The `pullup` parameter.
            ///
            /// # Arguments
            ///
            /// * `pin` - GPIO pin.
            /// * `pullup` - The `pullup` parameter.
            pub fn set_input(&self, pin: $p, pullup: bool) {
                write_reg($ddr, read_reg($ddr) & !(1 << pin as u8));
                if pullup {
                    write_reg($port, read_reg($port) | (1 << pin as u8));
                }
            }

            /// Drives the pin high.
            ///
            /// # Arguments
            ///
            /// * `pin` - GPIO pin.
            ///
            /// # Arguments
            ///
            /// * `pin` - GPIO pin.
            pub fn set_high(&self, pin: $p) {
                write_reg($port, read_reg($port) | (1 << pin as u8));
            }

            /// Drives the pin low.
            ///
            /// # Arguments
            ///
            /// * `pin` - GPIO pin.
            ///
            /// # Arguments
            ///
            /// * `pin` - GPIO pin.
            pub fn set_low(&self, pin: $p) {
                write_reg($port, read_reg($port) & !(1 << pin as u8));
            }

            /// Reads the pin state (true = high).
            ///
            /// # Arguments
            ///
            /// * `pin` - GPIO pin.
            ///
            /// # Returns
            ///
            /// `true` if successful or set, `false` otherwise.
            ///
            /// # Arguments
            ///
            /// * `pin` - GPIO pin.
            ///
            /// # Returns
            ///
            /// `true` if successful or set, `false` otherwise.
            pub fn is_high(&self, pin: $p) -> bool {
                read_reg($pin) & (1 << pin as u8) != 0
            }

            /// Toggles the pin output state.
            ///
            /// # Arguments
            ///
            /// * `pin` - GPIO pin.
            ///
            /// # Arguments
            ///
            /// * `pin` - GPIO pin.
            pub fn toggle(&self, pin: $p) {
                write_reg($pin, 1 << pin as u8);
            }
        }
    };
}

/// Port B (digital pins 8–13 on Arduino).
pub struct PortB;
impl PortB {
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
        Some(PortB)
    }

    /// Configures a pin as a push-pull output.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    pub fn set_output(&self, pin: PinB) {
        write_reg(0x24, read_reg(0x24) | (1 << pin as u8));
    }

    /// Configures a pin as an input, optionally with pull-up.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    /// * `pullup` - The `pullup` parameter.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    /// * `pullup` - The `pullup` parameter.
    pub fn set_input(&self, pin: PinB, pullup: bool) {
        write_reg(0x24, read_reg(0x24) & !(1 << pin as u8));
        if pullup {
            write_reg(0x25, read_reg(0x25) | (1 << pin as u8));
        }
    }

    /// Drives the pin high.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    pub fn set_high(&self, pin: PinB) {
        write_reg(0x25, read_reg(0x25) | (1 << pin as u8));
    }

    /// Drives the pin low.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    pub fn set_low(&self, pin: PinB) {
        write_reg(0x25, read_reg(0x25) & !(1 << pin as u8));
    }

    /// Reads the pin state (true = high).
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    ///
    /// # Returns
    ///
    /// `true` if successful or set, `false` otherwise.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    ///
    /// # Returns
    ///
    /// `true` if successful or set, `false` otherwise.
    pub fn is_high(&self, pin: PinB) -> bool {
        read_reg(0x23) & (1 << pin as u8) != 0
    }

    /// Toggles the pin output state.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    pub fn toggle(&self, pin: PinB) {
        write_reg(0x23, 1 << pin as u8);
    }
}

/// Port C (analog input pins A0–A5 on Arduino).
pub struct PortC;
impl PortC {
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
        Some(PortC)
    }

    /// Configures a pin as a push-pull output.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    pub fn set_output(&self, pin: PinC) {
        write_reg(0x27, read_reg(0x27) | (1 << pin as u8));
    }

    /// Configures a pin as an input, optionally with pull-up.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    /// * `pullup` - The `pullup` parameter.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    /// * `pullup` - The `pullup` parameter.
    pub fn set_input(&self, pin: PinC, pullup: bool) {
        write_reg(0x27, read_reg(0x27) & !(1 << pin as u8));
        if pullup {
            write_reg(0x28, read_reg(0x28) | (1 << pin as u8));
        }
    }

    /// Drives the pin high.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    pub fn set_high(&self, pin: PinC) {
        write_reg(0x28, read_reg(0x28) | (1 << pin as u8));
    }

    /// Drives the pin low.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    pub fn set_low(&self, pin: PinC) {
        write_reg(0x28, read_reg(0x28) & !(1 << pin as u8));
    }

    /// Reads the pin state (true = high).
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    ///
    /// # Returns
    ///
    /// `true` if successful or set, `false` otherwise.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    ///
    /// # Returns
    ///
    /// `true` if successful or set, `false` otherwise.
    pub fn is_high(&self, pin: PinC) -> bool {
        read_reg(0x26) & (1 << pin as u8) != 0
    }

    /// Toggles the pin output state.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    pub fn toggle(&self, pin: PinC) {
        write_reg(0x26, 1 << pin as u8);
    }
}

/// Port D (digital pins 0–7 on Arduino).
pub struct PortD;
impl PortD {
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
        Some(PortD)
    }

    /// Configures a pin as a push-pull output.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    pub fn set_output(&self, pin: PinD) {
        write_reg(0x2A, read_reg(0x2A) | (1 << pin as u8));
    }

    /// Configures a pin as an input, optionally with pull-up.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    /// * `pullup` - The `pullup` parameter.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    /// * `pullup` - The `pullup` parameter.
    pub fn set_input(&self, pin: PinD, pullup: bool) {
        write_reg(0x2A, read_reg(0x2A) & !(1 << pin as u8));
        if pullup {
            write_reg(0x2B, read_reg(0x2B) | (1 << pin as u8));
        }
    }

    /// Drives the pin high.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    pub fn set_high(&self, pin: PinD) {
        write_reg(0x2B, read_reg(0x2B) | (1 << pin as u8));
    }

    /// Drives the pin low.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    pub fn set_low(&self, pin: PinD) {
        write_reg(0x2B, read_reg(0x2B) & !(1 << pin as u8));
    }

    /// Reads the pin state (true = high).
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    ///
    /// # Returns
    ///
    /// `true` if successful or set, `false` otherwise.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    ///
    /// # Returns
    ///
    /// `true` if successful or set, `false` otherwise.
    pub fn is_high(&self, pin: PinD) -> bool {
        read_reg(0x29) & (1 << pin as u8) != 0
    }

    /// Toggles the pin output state.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    ///
    /// # Arguments
    ///
    /// * `pin` - GPIO pin.
    pub fn toggle(&self, pin: PinD) {
        write_reg(0x29, 1 << pin as u8);
    }
}
