use anyhow::Result;
use esp_idf_hal::rmt::{FixedLengthSignal, PinState, Pulse, TxRmtDriver};
use std::time::Duration;

use crate::{
    color::{Rgb, BLACK},
    infra::Switch,
};

pub const BLINK_FREQ: u64 = 3;

/// Sends an RGB color value to a `NeoPixel` LED using the RMT peripheral.
///
/// # Arguments
///
/// * `rgb` - An `Rgb` struct containing the red, green, and blue color values.
/// * `tx` - A mutable reference to a `TxRmtDriver` used to transmit the signal.
///
/// # Returns
///
/// * `Result<()>` - Returns `Ok(())` if the operation was successful, or an `anyhow::Error` if an error occurred.
///
/// # Errors
///
/// This function will return an error if:
///
/// * There is an issue with the RMT driver, such as failing to retrieve the counter clock frequency.
/// * There is an issue creating the pulses with the specified durations.
/// * There is an issue setting the signal pulses.
/// * There is an issue starting the transmission.
fn neopixel(rgb: &Rgb, tx: &mut TxRmtDriver) -> Result<()> {
    let color: u32 = rgb.into();
    let ticks_hz = tx.counter_clock()?;
    let (t0_high, t0_low, t1_high, t1_low) = (
        Pulse::new_with_duration(
            ticks_hz,
            PinState::High,
            &Duration::from_nanos(350),
        )?,
        Pulse::new_with_duration(
            ticks_hz,
            PinState::Low,
            &Duration::from_nanos(800),
        )?,
        Pulse::new_with_duration(
            ticks_hz,
            PinState::High,
            &Duration::from_nanos(700),
        )?,
        Pulse::new_with_duration(
            ticks_hz,
            PinState::Low,
            &Duration::from_nanos(600),
        )?,
    );
    let mut signal = FixedLengthSignal::<24>::new();
    for i in (0..24).rev() {
        let p = 2_u32.pow(i);
        let bit: bool = p & color != 0;
        let (high_pulse, low_pulse) = if bit {
            (t1_high, t1_low)
        } else {
            (t0_high, t0_low)
        };
        signal.set(23 - i as usize, &(high_pulse, low_pulse))?;
    }
    tx.start_blocking(&signal)?;
    Ok(())
}

pub enum State {
    On,
    Off,
}

pub struct Led<'a> {
    color: Rgb,
    state: State,
    tx_rmt: TxRmtDriver<'a>,
}

impl<'a> Led<'a> {
    pub fn new(tx_rmt: TxRmtDriver<'a>) -> Result<Self> {
        let mut ret = Self {
            tx_rmt,
            color: BLACK,
            state: State::Off,
        };
        ret.apply()?;

        Ok(ret)
    }

    fn apply(&mut self) -> Result<()> {
        match self.state {
            State::On => neopixel(&self.color, &mut self.tx_rmt),
            State::Off => neopixel(&BLACK, &mut self.tx_rmt),
        }
    }

    pub fn set_color(&mut self, color: Rgb) -> Result<()> {
        self.color = color;

        self.apply()
    }

    pub fn on(&mut self) -> Result<()> {
        self.state = State::On;

        self.apply()
    }

    pub fn off(&mut self) -> Result<()> {
        self.state = State::Off;

        self.apply()
    }
}

impl Switch for Led<'_> {
    fn toggle(&mut self) -> Result<()> {
        match self.state {
            State::On => self.off(),
            State::Off => self.on(),
        }
    }
}
