use anyhow::{anyhow, Result};
use esp_idf_hal::{
    gpio::{InputMode, InputPin, PinDriver},
    task::notification::Notifier,
};
use std::sync::{Arc, Mutex};

use crate::{
    infra::Poller,
    time::{sleep, yield_now},
    trigger::Trigger,
};

pub enum State {
    On,
    Off,
}

pub struct Button<'a, T, MODE>
where
    T: InputPin,
    MODE: InputMode,
{
    notifier: Arc<Notifier>,
    pin: PinDriver<'a, T, MODE>,
    state: Arc<Mutex<State>>,
}

impl<'a, T, MODE> Button<'a, T, MODE>
where
    T: InputPin,
    MODE: InputMode,
{
    pub fn new(
        notifier: Arc<Notifier>,
        pin: PinDriver<'a, T, MODE>,
        state: Arc<Mutex<State>>,
    ) -> Result<Self> {
        Ok(Self {
            notifier,
            pin,
            state,
        })
    }

    fn pressed(&self) -> Result<bool> {
        Ok(self.pin.is_low())
    }

    fn toggle_state(&self) -> Result<()> {
        let mut state = self
            .state
            .lock()
            .map_err(|e| anyhow!("Mutex lock error: {:?}", e))?;

        *state = match *state {
            State::On => State::Off,
            State::Off => State::On,
        };

        Ok(())
    }
}

impl<'a, T, MODE> Poller for Button<'_, T, MODE>
where
    T: InputPin,
    MODE: InputMode,
{
    fn poll(&mut self) -> Result<()> {
        // Using polling instead of interrupts for the button as on some boards
        // (e.g. M5Stack's Atom Lite) the interrupt pin of the button is too close
        // to the WiFi antenna which causes interference.

        loop {
            if self.pressed()? {
                unsafe {
                    self.notifier
                        .notify_and_yield(Trigger::ButtonPressed.try_into()?);
                }
                self.toggle_state()?;
                sleep(500);
            }
            yield_now();
        }
    }
}
