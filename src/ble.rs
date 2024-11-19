use anyhow::{anyhow, Result};
use esp32_nimble::{BLEAdvertisementData, BLEDevice, BLEScan};
use esp_idf_hal::task::{block_on, notification::Notifier};
use std::sync::{Arc, Mutex};

use crate::{
    button,
    clock::Timer,
    infra::{Poller, Switch},
    trigger::Trigger,
};

const SCAN_FREQ: u64 = 1;

enum State {
    Active,
    Inactive,
}

pub struct Advertiser<'a> {
    name: &'a str,
    state: State,
}

impl<'a> Advertiser<'a> {
    pub fn new(name: &'a str) -> Result<Self> {
        let ret = Self {
            name,
            state: State::Inactive,
        };
        ret.apply()?;

        Ok(ret)
    }

    fn apply(&self) -> Result<()> {
        let device = BLEDevice::take();
        let advertising = device.get_advertising();
        let name = match self.state {
            // TODO: This doesn't take into account the fact that multiple devices could be nearby.
            //       That could be handled with some kind of an ID mechanism...
            State::Active => format!("{}-Active", self.name),
            State::Inactive => format!("{}-Inactive", self.name),
        };

        advertising
            .lock()
            .set_data(BLEAdvertisementData::new().name(&name))?;
        advertising.lock().start()?;

        Ok(())
    }
}

impl Switch for Advertiser<'_> {
    fn toggle(&mut self) -> Result<()> {
        self.state = match self.state {
            State::Active => State::Inactive,
            State::Inactive => State::Active,
        };

        self.apply()
    }
}

pub struct Scanner<'a> {
    name: &'a str,
    notifier: Arc<Notifier>,
    timer: Timer<'a>,
    state: Arc<Mutex<button::State>>,
    device: &'a BLEDevice,
    scan: BLEScan,
}

impl<'a> Scanner<'a> {
    const WINDOW: i32 = 1000;

    pub fn new(
        name: &'a str,
        notifier: Arc<Notifier>,
        timer: Timer<'a>,
        state: Arc<Mutex<button::State>>,
    ) -> Result<Self> {
        let device = BLEDevice::take();
        let scan = BLEScan::new();

        Ok(Self {
            name,
            notifier,
            timer,
            state,
            device,
            scan,
        })
    }

    async fn do_scan(&mut self) -> Result<Option<Trigger>> {
        Ok(self
            .scan
            .start(self.device, Self::WINDOW, |_, data| {
                data.name().and_then(|name| {
                    if name == format!("{}-Active", self.name) {
                        Some(Trigger::DeviceFoundActive)
                    } else if name == format!("{}-Inactive", self.name) {
                        Some(Trigger::DeviceFoundInactive)
                    } else {
                        None
                    }
                })
            })
            .await?)
    }
}

impl Poller for Scanner<'_> {
    fn poll(&mut self) -> Result<()> {
        block_on(async {
            loop {
                let _ = self.timer.delay(SCAN_FREQ).await?;

                if let button::State::Off = *self
                    .state
                    .lock()
                    .map_err(|e| anyhow!("Mutex lock error: {:?}", e))?
                {
                    continue;
                }

                let trigger = if let Some(trigger) = self.do_scan().await? {
                    trigger
                } else {
                    Trigger::DeviceNotFound
                };

                unsafe {
                    self.notifier.notify_and_yield(trigger.try_into()?);
                }
            }

            Ok::<(), anyhow::Error>(())
        })?;

        Ok(())
    }
}
