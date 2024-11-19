use anyhow::Result;
use esp_idf_hal::{task::notification::Notifier, timer::TimerDriver};
use std::sync::Arc;

use crate::{thread::failure, trigger::Trigger};

pub struct Timer<'a> {
    timer: TimerDriver<'a>,
}

impl<'a> Timer<'a> {
    pub fn new(timer: TimerDriver<'a>) -> Result<Self> {
        Ok(Self { timer })
    }

    pub fn configure_interrupt(
        &mut self,
        freq: u64,
        notifier: Arc<Notifier>,
    ) -> Result<()> {
        unsafe {
            self.timer.subscribe(move || {
                if let Ok(trigger) = Trigger::TimerTicked.try_into() {
                    notifier.notify_and_yield(trigger);
                } else {
                    failure();
                }
            })?;
        }

        self.timer.set_alarm(self.timer.tick_hz() / freq)?;
        self.timer.enable_interrupt()?;

        Ok(())
    }

    fn enable(&mut self, enable: bool) -> Result<()> {
        self.timer.enable(enable)?;
        self.timer.enable_alarm(enable)?;

        Ok(())
    }

    pub fn on(&mut self) -> Result<()> {
        self.enable(true)
    }

    pub fn off(&mut self) -> Result<()> {
        self.enable(false)
    }

    pub async fn delay(&mut self, freq: u64) -> Result<()> {
        self.timer.delay(self.timer.tick_hz() / freq).await?;

        Ok(())
    }
}
