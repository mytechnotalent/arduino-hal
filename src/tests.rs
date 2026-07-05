//! Unit tests.
//!
//! **File:** `tests.rs`  
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

#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {

    // Import dependencies from crate::adc
    use crate::adc::{adc_mux_bits, adc_prescaler_bits, vref_bits};
    // Import dependencies from crate::io
    use crate::io::{read_reg, write_reg};
    // Import crate::mock
    use crate::mock;
    // Import crate::timer::prescaler_bits
    use crate::timer::prescaler_bits;
    // Import crate::adc::*
    use crate::adc::*;
    // Import crate::board::*
    use crate::board::*;
    // Import crate::gpio::*
    use crate::gpio::*;
    // Import crate::lcd::*
    use crate::lcd::*;
    // Import crate::timer::*
    use crate::timer::*;
    // Import crate::usart::*
    use crate::usart::*;

    /// Executes the reset operation.
    fn reset() {
        mock::reset();
    }

    /// Executes the test write read round trip operation.
    #[test]
    fn test_write_read_round_trip() {
        reset();
        write_reg(0x25, 0xAB);
        assert_eq!(read_reg(0x25), 0xAB);
    }

    /// Executes the test write read multiple regs operation.
    #[test]
    fn test_write_read_multiple_regs() {
        reset();
        write_reg(0x25, 0x01);
        write_reg(0x24, 0x02);
        write_reg(0x23, 0x04);
        assert_eq!(read_reg(0x25), 0x01);
        assert_eq!(read_reg(0x24), 0x02);
        assert_eq!(read_reg(0x23), 0x04);
    }

    /// Executes the test write reg overwrites operation.
    #[test]
    fn test_write_reg_overwrites() {
        reset();
        write_reg(0x25, 0x01);
        write_reg(0x25, 0xFF);
        assert_eq!(read_reg(0x25), 0xFF);
    }

    /// Executes the test pinb has correct variants operation.
    #[test]
    fn test_pinb_has_correct_variants() {
        assert_eq!(PinB::PB0 as u8, 0);
        assert_eq!(PinB::PB5 as u8, 5);
    }

    /// Executes the test pinc has correct variants operation.
    #[test]
    fn test_pinc_has_correct_variants() {
        assert_eq!(PinC::PC0 as u8, 0);
        assert_eq!(PinC::PC5 as u8, 5);
    }

    /// Executes the test pind has correct variants operation.
    #[test]
    fn test_pind_has_correct_variants() {
        assert_eq!(PinD::PD0 as u8, 0);
        assert_eq!(PinD::PD7 as u8, 7);
    }

    /// Executes the test pin enums are clone copy operation.
    #[test]
    fn test_pin_enums_are_clone_copy() {
        let a = PinB::PB0;
        let _b = a;
        let _c = a;
        assert_eq!(a as u8, 0);
    }

    /// Executes the test portb take operation.
    #[test]
    fn test_portb_take() {
        reset();
        let p = PortB::take();
        assert!(p.is_some());
    }

    /// Executes the test portb set output sets ddrb bit operation.
    #[test]
    fn test_portb_set_output_sets_ddrb_bit() {
        reset();
        let p = PortB::take().unwrap();
        p.set_output(PinB::PB5);
        assert_eq!(read_reg(0x24), 1 << 5);
    }

    /// Executes the test portb set output preserves other bits operation.
    #[test]
    fn test_portb_set_output_preserves_other_bits() {
        reset();
        write_reg(0x24, 0x01);
        let p = PortB::take().unwrap();
        p.set_output(PinB::PB5);
        assert_eq!(read_reg(0x24), 0x01 | (1 << 5));
    }

    /// Executes the test portb set input clears ddrb bit operation.
    #[test]
    fn test_portb_set_input_clears_ddrb_bit() {
        reset();
        write_reg(0x24, 0xFF);
        let p = PortB::take().unwrap();
        p.set_input(PinB::PB5, false);
        assert_eq!(read_reg(0x24), !(1 << 5));
    }

    /// Executes the test portb set input with pullup sets port bit operation.
    #[test]
    fn test_portb_set_input_with_pullup_sets_port_bit() {
        reset();
        let p = PortB::take().unwrap();
        p.set_input(PinB::PB3, true);
        assert_eq!(read_reg(0x25), 1 << 3);
    }

    /// Executes the test portb set input without pullup operation.
    #[test]
    fn test_portb_set_input_without_pullup() {
        reset();
        write_reg(0x25, 0xFF);
        let p = PortB::take().unwrap();
        p.set_input(PinB::PB3, false);
        assert_eq!(read_reg(0x25), 0xFF);
    }

    /// Executes the test portb set high sets port bit operation.
    #[test]
    fn test_portb_set_high_sets_port_bit() {
        reset();
        let p = PortB::take().unwrap();
        p.set_high(PinB::PB2);
        assert_eq!(read_reg(0x25), 1 << 2);
    }

    /// Executes the test portb set low clears port bit operation.
    #[test]
    fn test_portb_set_low_clears_port_bit() {
        reset();
        write_reg(0x25, 0xFF);
        let p = PortB::take().unwrap();
        p.set_low(PinB::PB2);
        assert_eq!(read_reg(0x25), !(1 << 2));
    }

    /// Executes the test portb is high returns true when bit set operation.
    #[test]
    fn test_portb_is_high_returns_true_when_bit_set() {
        reset();
        write_reg(0x23, 1 << 4);
        let p = PortB::take().unwrap();
        assert!(p.is_high(PinB::PB4));
    }

    /// Executes the test portb is high returns false when bit clear operation.
    #[test]
    fn test_portb_is_high_returns_false_when_bit_clear() {
        reset();
        let p = PortB::take().unwrap();
        assert!(!p.is_high(PinB::PB4));
    }

    /// Executes the test portb toggle writes to pin reg operation.
    #[test]
    fn test_portb_toggle_writes_to_pin_reg() {
        reset();
        let p = PortB::take().unwrap();
        p.toggle(PinB::PB1);
        assert_eq!(read_reg(0x23), 1 << 1);
    }

    /// Executes the test portc take operation.
    #[test]
    fn test_portc_take() {
        reset();
        let p = PortC::take();
        assert!(p.is_some());
    }

    /// Executes the test portc set output sets ddrc operation.
    #[test]
    fn test_portc_set_output_sets_ddrc() {
        reset();
        let p = PortC::take().unwrap();
        p.set_output(PinC::PC0);
        assert_eq!(read_reg(0x27), 1 << 0);
    }

    /// Executes the test portc set input clears ddrc operation.
    #[test]
    fn test_portc_set_input_clears_ddrc() {
        reset();
        write_reg(0x27, 0xFF);
        let p = PortC::take().unwrap();
        p.set_input(PinC::PC2, false);
        assert_eq!(read_reg(0x27), !(1 << 2));
    }

    /// Executes the test portc set input with pullup operation.
    #[test]
    fn test_portc_set_input_with_pullup() {
        reset();
        let p = PortC::take().unwrap();
        p.set_input(PinC::PC3, true);
        assert_eq!(read_reg(0x28), 1 << 3);
    }

    /// Executes the test portc set high operation.
    #[test]
    fn test_portc_set_high() {
        reset();
        let p = PortC::take().unwrap();
        p.set_high(PinC::PC4);
        assert_eq!(read_reg(0x28), 1 << 4);
    }

    /// Executes the test portc set low operation.
    #[test]
    fn test_portc_set_low() {
        reset();
        write_reg(0x28, 0xFF);
        let p = PortC::take().unwrap();
        p.set_low(PinC::PC4);
        assert_eq!(read_reg(0x28), !(1 << 4));
    }

    /// Executes the test portc is high operation.
    #[test]
    fn test_portc_is_high() {
        reset();
        write_reg(0x26, 1 << 5);
        let p = PortC::take().unwrap();
        assert!(p.is_high(PinC::PC5));
    }

    /// Executes the test portc toggle operation.
    #[test]
    fn test_portc_toggle() {
        reset();
        let p = PortC::take().unwrap();
        p.toggle(PinC::PC1);
        assert_eq!(read_reg(0x26), 1 << 1);
    }

    /// Executes the test portd take operation.
    #[test]
    fn test_portd_take() {
        reset();
        let p = PortD::take();
        assert!(p.is_some());
    }

    /// Executes the test portd set output sets ddrd operation.
    #[test]
    fn test_portd_set_output_sets_ddrd() {
        reset();
        let p = PortD::take().unwrap();
        p.set_output(PinD::PD6);
        assert_eq!(read_reg(0x2A), 1 << 6);
    }

    /// Executes the test portd set input clears ddrd operation.
    #[test]
    fn test_portd_set_input_clears_ddrd() {
        reset();
        write_reg(0x2A, 0xFF);
        let p = PortD::take().unwrap();
        p.set_input(PinD::PD7, false);
        assert_eq!(read_reg(0x2A), !(1 << 7));
    }

    /// Executes the test portd set input with pullup operation.
    #[test]
    fn test_portd_set_input_with_pullup() {
        reset();
        let p = PortD::take().unwrap();
        p.set_input(PinD::PD4, true);
        assert_eq!(read_reg(0x2B), 1 << 4);
    }

    /// Executes the test portd set high operation.
    #[test]
    fn test_portd_set_high() {
        reset();
        let p = PortD::take().unwrap();
        p.set_high(PinD::PD2);
        assert_eq!(read_reg(0x2B), 1 << 2);
    }

    /// Executes the test portd set low operation.
    #[test]
    fn test_portd_set_low() {
        reset();
        write_reg(0x2B, 0xFF);
        let p = PortD::take().unwrap();
        p.set_low(PinD::PD2);
        assert_eq!(read_reg(0x2B), !(1 << 2));
    }

    /// Executes the test portd is high operation.
    #[test]
    fn test_portd_is_high() {
        reset();
        write_reg(0x29, 1 << 3);
        let p = PortD::take().unwrap();
        assert!(p.is_high(PinD::PD3));
    }

    /// Executes the test portd toggle operation.
    #[test]
    fn test_portd_toggle() {
        reset();
        let p = PortD::take().unwrap();
        p.toggle(PinD::PD0);
        assert_eq!(read_reg(0x29), 1 << 0);
    }

    /// Executes the test mode variant operation.
    #[test]
    fn test_mode_variant() {
        let _ = Mode::DoubleSpeedAsynchronous;
    }

    /// Executes the test usart0 take operation.
    #[test]
    fn test_usart0_take() {
        reset();
        let u = USART0::take();
        assert!(u.is_some());
    }

    /// Executes the test usart0 start clears prr bit operation.
    #[test]
    fn test_usart0_start_clears_prr_bit() {
        reset();
        write_reg(0x64, 0xFF);
        USART0::take().unwrap().start();
        assert_eq!(read_reg(0x64), !(1 << 1));
    }

    /// Executes the test usart0 set baud rate writes ubrr operation.
    #[test]
    fn test_usart0_set_baud_rate_writes_ubrr() {
        reset();
        USART0::take()
            .unwrap()
            .set_baud_rate(9600, Mode::DoubleSpeedAsynchronous);
        let u = (16000000 / (8 * 9600)) as u16 - 1;
        assert_eq!(read_reg(0xC4), u as u8);
        assert_eq!(read_reg(0xC5), (u >> 8) as u8);
        assert_eq!(read_reg(0xC0), (1 << 1) | (1 << 5));
    }

    /// Executes the test usart0 set baud rate high baud operation.
    #[test]
    fn test_usart0_set_baud_rate_high_baud() {
        reset();
        USART0::take()
            .unwrap()
            .set_baud_rate(115200, Mode::DoubleSpeedAsynchronous);
        let u = (16000000 / (8 * 115200)) as u16 - 1;
        assert_eq!(read_reg(0xC4), u as u8);
    }

    /// Executes the test usart0 set frame format operation.
    #[test]
    fn test_usart0_set_frame_format() {
        reset();
        USART0::take().unwrap().set_frame_format();
        assert_eq!(read_reg(0xC2), 0x06);
    }

    /// Executes the test usart0 enable tx rx operation.
    #[test]
    fn test_usart0_enable_tx_rx() {
        reset();
        USART0::take().unwrap().enable_tx_rx();
        assert_eq!(read_reg(0xC1), (1 << 4) | (1 << 3));
    }

    /// Executes the test usart0 transmit char writes to udr operation.
    #[test]
    fn test_usart0_transmit_char_writes_to_udr() {
        reset();
        write_reg(0xC0, 1 << 5);
        USART0::take().unwrap().transmit_char('A');
        assert_eq!(read_reg(0xC6), b'A');
    }

    /// Executes the test usart0 print string writes all chars operation.
    #[test]
    fn test_usart0_print_string_writes_all_chars() {
        reset();
        write_reg(0xC0, 0xFF);
        USART0::take().unwrap().print_string("Hi");
        assert_eq!(read_reg(0xC6), b'i');
    }

    /// Executes the test usart0 print num zero operation.
    #[test]
    fn test_usart0_print_num_zero() {
        reset();
        write_reg(0xC0, 0xFF);
        USART0::take().unwrap().print_num(0);
        assert_eq!(read_reg(0xC6), b'0');
    }

    /// Executes the test usart0 print num multi digit operation.
    #[test]
    fn test_usart0_print_num_multi_digit() {
        reset();
        write_reg(0xC0, 0xFF);
        USART0::take().unwrap().print_num(42);
        assert_eq!(read_reg(0xC6), b'2');
    }

    /// Executes the test usart0 receive returns zero when no data operation.
    #[test]
    fn test_usart0_receive_returns_zero_when_no_data() {
        reset();
        assert_eq!(USART0::take().unwrap().usart_receive(), 0);
    }

    /// Executes the test usart0 receive returns data when available operation.
    #[test]
    fn test_usart0_receive_returns_data_when_available() {
        reset();
        write_reg(0xC0, 1 << 7);
        write_reg(0xC6, 0x55);
        assert_eq!(USART0::take().unwrap().usart_receive(), 0x55);
    }

    /// Executes the test usart0 transmit waits for udre operation.
    #[test]
    fn test_usart0_transmit_waits_for_udre() {
        reset();
        write_reg(0xC0, 0);
        write_reg(0xC0, 1 << 5);
        USART0::take().unwrap().transmit_char('Z');
        assert_eq!(read_reg(0xC6), b'Z');
    }

    /// Executes the test prescaler bits values operation.
    #[test]
    fn test_prescaler_bits_values() {
        assert_eq!(prescaler_bits(&Prescaler::Prescaler8), 0x02);
        assert_eq!(prescaler_bits(&Prescaler::Prescaler64), 0x03);
        assert_eq!(prescaler_bits(&Prescaler::Prescaler256), 0x04);
        assert_eq!(prescaler_bits(&Prescaler::Prescaler1024), 0x05);
    }

    /// Executes the test timer0 take operation.
    #[test]
    fn test_timer0_take() {
        reset();
        let t = Timer0Real::take();
        assert!(t.is_some());
    }

    /// Executes the test timer0 start clears prr bit operation.
    #[test]
    fn test_timer0_start_clears_prr_bit() {
        reset();
        write_reg(0x64, 0xFF);
        Timer0Real::take().unwrap().start();
        assert_eq!(read_reg(0x64), !(1 << 4));
    }

    /// Executes the test timer0 delay ms sets tccr0b operation.
    #[test]
    fn test_timer0_delay_ms_sets_tccr0b() {
        reset();
        Timer0Real::take().unwrap().delay_ms(5);
        assert_eq!(read_reg(0x45), 0);
    }

    /// Executes the test timer0 delay ms stops timer operation.
    #[test]
    fn test_timer0_delay_ms_stops_timer() {
        reset();
        Timer0Real::take().unwrap().delay_ms(1);
        assert_eq!(read_reg(0x45), 0);
    }

    /// Executes the test timer0 set normal mode operation.
    #[test]
    fn test_timer0_set_normal_mode() {
        reset();
        Timer0Real::take().unwrap().set_normal_mode();
        assert_eq!(read_reg(0x44), 0x00);
    }

    /// Executes the test timer0 set ctc mode operation.
    #[test]
    fn test_timer0_set_ctc_mode() {
        reset();
        Timer0Real::take().unwrap().set_ctc_mode();
        assert_eq!(read_reg(0x44), 0x02);
    }

    /// Executes the test timer0 set fast pwm operation.
    #[test]
    fn test_timer0_set_fast_pwm() {
        reset();
        Timer0Real::take().unwrap().set_fast_pwm(128);
        assert_eq!(read_reg(0x44), 0x03);
        assert_eq!(read_reg(0x48), 128);
    }

    /// Executes the test timer0 set fast pwm top truncated operation.
    #[test]
    fn test_timer0_set_fast_pwm_top_truncated() {
        reset();
        Timer0Real::take().unwrap().set_fast_pwm(300);
        assert_eq!(read_reg(0x48), (300 & 0xFF) as u8);
    }

    /// Executes the test timer1 take operation.
    #[test]
    fn test_timer1_take() {
        reset();
        let t = Timer1Real::take();
        assert!(t.is_some());
    }

    /// Executes the test timer1 start clears prr bit operation.
    #[test]
    fn test_timer1_start_clears_prr_bit() {
        reset();
        write_reg(0x64, 0xFF);
        Timer1Real::take().unwrap().start();
        assert_eq!(read_reg(0x64), !(1 << 3));
    }

    /// Executes the test timer1 set normal mode operation.
    #[test]
    fn test_timer1_set_normal_mode() {
        reset();
        Timer1Real::take().unwrap().set_normal_mode();
        assert_eq!(read_reg(0x80), 0x00);
        assert_eq!(read_reg(0x81), 0x00);
    }

    /// Executes the test timer1 set ctc mode operation.
    #[test]
    fn test_timer1_set_ctc_mode() {
        reset();
        Timer1Real::take().unwrap().set_ctc_mode();
        assert_eq!(read_reg(0x81), 0x08);
    }

    /// Executes the test timer1 set fast pwm operation.
    #[test]
    fn test_timer1_set_fast_pwm() {
        reset();
        Timer1Real::take().unwrap().set_fast_pwm(1999);
        assert_eq!(read_reg(0x80), 0x82);
        assert_eq!(read_reg(0x81), 0x18);
        assert_eq!(((read_reg(0x87) as u16) << 8) | read_reg(0x86) as u16, 1999);
    }

    /// Executes the test timer1 set prescaler operation.
    #[test]
    fn test_timer1_set_prescaler() {
        reset();
        write_reg(0x81, 0xFF);
        Timer1Real::take()
            .unwrap()
            .set_prescaler(Prescaler::Prescaler64);
        assert_eq!(read_reg(0x81) & 0x07, 0x03);
    }

    /// Executes the test timer1 set top value operation.
    #[test]
    fn test_timer1_set_top_value() {
        reset();
        Timer1Real::take().unwrap().set_top_value(9999);
        assert_eq!(((read_reg(0x89) as u16) << 8) | read_reg(0x88) as u16, 9999);
    }

    /// Executes the test timer1 wait for match clears flag operation.
    #[test]
    fn test_timer1_wait_for_match_clears_flag() {
        reset();
        write_reg(0x36, 1 << 1);
        Timer1Real::take().unwrap().wait_for_match();
        assert_eq!(read_reg(0x36) & (1 << 1), 1 << 1); // Mock doesn't clear flag
    }

    /// Executes the test timer1 wait clears overflow flag operation.
    #[test]
    fn test_timer1_wait_clears_overflow_flag() {
        reset();
        write_reg(0x36, 1 << 0);
        Timer1Real::take().unwrap().wait();
        assert_eq!(read_reg(0x36) & (1 << 0), 1 << 0); // Mock doesn't clear flag
    }

    /// Executes the test timer1 set duty cycle operation.
    #[test]
    fn test_timer1_set_duty_cycle() {
        reset();
        Timer1Real::take().unwrap().set_duty_cycle(1500);
        assert_eq!(((read_reg(0x89) as u16) << 8) | read_reg(0x88) as u16, 1500);
    }

    /// Executes the test input channel variants operation.
    #[test]
    fn test_input_channel_variants() {
        assert_eq!(InputChannel::AnalogPin0 as u8, 0);
        assert_eq!(InputChannel::AnalogPin5 as u8, 5);
    }

    /// Executes the test adc prescaler variants operation.
    #[test]
    fn test_adc_prescaler_variants() {
        assert_eq!(adc_prescaler_bits(&ADCprescaler::Prescaler2), 1);
        assert_eq!(adc_prescaler_bits(&ADCprescaler::Prescaler128), 7);
    }

    /// Executes the test adc mux bits values operation.
    #[test]
    fn test_adc_mux_bits_values() {
        assert_eq!(adc_mux_bits(&InputChannel::AnalogPin0), 0);
        assert_eq!(adc_mux_bits(&InputChannel::AnalogPin3), 3);
    }

    /// Executes the test vref bits values operation.
    #[test]
    fn test_vref_bits_values() {
        assert_eq!(vref_bits(&VoltageReference::Default), 0);
        assert_eq!(vref_bits(&VoltageReference::AVcc), 1);
        assert_eq!(vref_bits(&VoltageReference::Internal), 3);
        assert_eq!(vref_bits(&VoltageReference::External), 0);
    }

    /// Executes the test adc take operation.
    #[test]
    fn test_adc_take() {
        reset();
        let a = ADC::take();
        assert!(a.is_some());
    }

    /// Executes the test adc start resets adcsra operation.
    #[test]
    fn test_adc_start_resets_adcsra() {
        reset();
        write_reg(0x7A, 0xFF);
        ADC::take().unwrap().start();
        assert_eq!(read_reg(0x7A), 0);
    }

    /// Executes the test adc select input channel operation.
    #[test]
    fn test_adc_select_input_channel() {
        reset();
        ADC::take()
            .unwrap()
            .select_input_channel(InputChannel::AnalogPin3);
        assert_eq!(read_reg(0x7C) & 0x07, 3);
    }

    /// Executes the test adc select input channel preserves refs operation.
    #[test]
    fn test_adc_select_input_channel_preserves_refs() {
        reset();
        write_reg(0x7C, 0x80);
        ADC::take()
            .unwrap()
            .select_input_channel(InputChannel::AnalogPin1);
        assert_eq!(read_reg(0x7C) & 0xE0, 0x80);
        assert_eq!(read_reg(0x7C) & 0x07, 1);
    }

    /// Executes the test adc set reference avcc operation.
    #[test]
    fn test_adc_set_reference_avcc() {
        reset();
        ADC::take().unwrap().set_reference(VoltageReference::AVcc);
        assert_eq!(read_reg(0x7C) >> 6, 1);
    }

    /// Executes the test adc set reference internal operation.
    #[test]
    fn test_adc_set_reference_internal() {
        reset();
        ADC::take()
            .unwrap()
            .set_reference(VoltageReference::Internal);
        assert_eq!(read_reg(0x7C) >> 6, 3);
    }

    /// Executes the test adc set reference preserves mux operation.
    #[test]
    fn test_adc_set_reference_preserves_mux() {
        reset();
        write_reg(0x7C, 0x05);
        ADC::take().unwrap().set_reference(VoltageReference::AVcc);
        assert_eq!(read_reg(0x7C) & 0x07, 5);
    }

    /// Executes the test adc start conversion returns result operation.
    #[test]
    fn test_adc_start_conversion_returns_result() {
        reset();
        write_reg(0x7A, 1 << 6);
        write_reg(0x78, 0x34);
        write_reg(0x79, 0x12);
        let r = ADC::take()
            .unwrap()
            .start_conversion(ADCprescaler::Prescaler64);
        assert_eq!(r, 0x1234);
    }

    /// Executes the test adc start conversion sets prescaler operation.
    #[test]
    fn test_adc_start_conversion_sets_prescaler() {
        reset();
        write_reg(0x7A, 1 << 6);
        ADC::take()
            .unwrap()
            .start_conversion(ADCprescaler::Prescaler128);
        assert_eq!(read_reg(0x7A) & 0x07, 7);
    }

    /// Executes the test lcd take operation.
    #[test]
    fn test_lcd_take() {
        reset();
        let l = LCD::take();
        assert!(l.is_some());
    }

    /// Executes the test lcd clear display writes cmd operation.
    #[test]
    fn test_lcd_clear_display_writes_cmd() {
        reset();
        LCD::take().unwrap().clear_display();
        let portb = read_reg(0x25);
        assert_eq!(portb & 0x03, 0);
    }

    /// Executes the test lcd write char writes data operation.
    #[test]
    fn test_lcd_write_char_writes_data() {
        reset();
        LCD::take().unwrap().write_char('A');
        let portb = read_reg(0x25);
        assert_eq!(portb & 0x03, (b'A' >> 2) & 0x03);
    }

    /// Executes the test lcd print writes all chars operation.
    #[test]
    fn test_lcd_print_writes_all_chars() {
        reset();
        LCD::take().unwrap().print("AB");
        let portb = read_reg(0x25);
        assert_eq!(portb & 0x03, (b'B' >> 2) & 0x03);
    }

    /// Executes the test lcd print number zero operation.
    #[test]
    fn test_lcd_print_number_zero() {
        reset();
        LCD::take().unwrap().print_number(0);
        let portb = read_reg(0x25);
        assert_eq!(portb & 0x03, (b'0' >> 2) & 0x03);
    }

    /// Executes the test lcd print number multi operation.
    #[test]
    fn test_lcd_print_number_multi() {
        reset();
        LCD::take().unwrap().print_number(42);
        let portb = read_reg(0x25);
        assert_eq!(portb & 0x03, (b'2' >> 2) & 0x03);
    }

    /// Executes the test board init disables watchdog operation.
    #[test]
    fn test_board_init_disables_watchdog() {
        reset();
        write_reg(0x60, 0xFF);
        board_init();
        assert_eq!(read_reg(0x60), 0);
    }
}
