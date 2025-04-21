use anyhow::Result;
use esp_idf_hal::timer::TimerDriver;

use crate::{
    message::{Notifier, Trigger},
    thread::failure,
};

/// Represents a timer that can be used for various operations.
///
/// # Type Parameters
/// * `'a` - Lifetime of the timer.
pub struct Timer<'a> {
    timer: TimerDriver<'a>,
}

impl<'a> Timer<'a> {
    /// Creates a new `Timer` instance.
    ///
    /// # Arguments
    /// * `timer` - A timer driver instance.
    ///
    /// # Errors
    /// Returns an error if the timer cannot be initialized.
    pub fn new(timer: TimerDriver<'a>) -> Result<Self> {
        Ok(Self { timer })
    }

    /// Configures the timer interrupt.
    ///
    /// # Arguments
    /// * `freq` - Frequency of the timer interrupt.
    /// * `notifier` - A notifier to send timer tick events.
    ///
    /// # Errors
    /// Returns an error if the interrupt cannot be configured.
    pub fn configure_interrupt(
        &mut self,
        freq: u64,
        notifier: Notifier,
    ) -> Result<()> {
        unsafe {
            self.timer.subscribe(move || {
                notifier
                    .notify(Trigger::TimerTicked)
                    .unwrap_or_else(|_| failure());
            })?;
        }

        self.timer.set_alarm(self.timer.tick_hz() / freq)?;
        self.timer.enable_interrupt()?;

        Ok(())
    }

    /// Enables or disables the timer.
    ///
    /// # Arguments
    /// * `enable` - `true` to enable the timer, `false` to disable it.
    ///
    /// # Errors
    /// Returns an error if the timer cannot be enabled or disabled.
    fn enable(&mut self, enable: bool) -> Result<()> {
        self.timer.enable(enable)?;
        self.timer.enable_alarm(enable)?;

        Ok(())
    }

    /// Turns on the timer.
    ///
    /// # Errors
    /// Returns an error if the timer cannot be turned on.
    pub fn on(&mut self) -> Result<()> {
        self.enable(true)
    }

    /// Turns off the timer.
    ///
    /// # Errors
    /// Returns an error if the timer cannot be turned off.
    pub fn off(&mut self) -> Result<()> {
        self.enable(false)
    }

    /// Delays execution for a specified frequency.
    ///
    /// # Arguments
    /// * `freq` - Frequency for the delay.
    ///
    /// # Errors
    /// Returns an error if the delay cannot be performed.
    pub async fn delay(&mut self, freq: u64) -> Result<()> {
        self.timer.delay(self.timer.tick_hz() / freq).await?;

        Ok(())
    }
}
