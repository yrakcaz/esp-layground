use anyhow::{anyhow, Result};
use esp32_nimble::{BLEAdvertisementData, BLEDevice, BLEScan};
use esp_idf_hal::task::block_on;
use std::sync::{Arc, Mutex};

use crate::{
    button,
    clock::Timer,
    infra::{Poller, Switch},
    message::{Notifier, Trigger},
};

const SCAN_FREQ: u64 = 1;

/// Represents the state of the BLE advertiser.
///
/// # Variants
/// * `Active` - The advertiser is active.
/// * `Inactive` - The advertiser is inactive.
enum State {
    Active,
    Inactive,
}

/// Represents a BLE advertiser.
///
/// # Type Parameters
/// * `'a` - Lifetime of the advertiser.
pub struct Advertiser<'a> {
    name: &'a str,
    state: State,
}

impl<'a> Advertiser<'a> {
    /// Creates a new `Advertiser` instance.
    ///
    /// # Arguments
    /// * `name` - The name of the advertiser.
    ///
    /// # Errors
    /// Returns an error if the advertiser cannot be initialized.
    pub fn new(name: &'a str) -> Result<Self> {
        let ret = Self {
            name,
            state: State::Inactive,
        };
        ret.apply()?;

        Ok(ret)
    }

    /// Applies the current state to the BLE advertiser.
    ///
    /// # Errors
    /// Returns an error if the BLE device or advertising data cannot be configured.
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
    /// Toggles the state of the advertiser.
    ///
    /// # Errors
    /// Returns an error if the state cannot be toggled or applied.
    fn toggle(&mut self) -> Result<()> {
        self.state = match self.state {
            State::Active => State::Inactive,
            State::Inactive => State::Active,
        };

        self.apply()
    }
}

/// Represents a BLE scanner.
///
/// # Type Parameters
/// * `'a` - Lifetime of the scanner.
pub struct Scanner<'a> {
    name: &'a str,
    notifier: Notifier,
    timer: Timer<'a>,
    state: Arc<Mutex<button::State>>,
    device: &'a BLEDevice,
    scan: BLEScan,
}

impl<'a> Scanner<'a> {
    const WINDOW: i32 = 1000;

    /// Creates a new `Scanner` instance.
    ///
    /// # Arguments
    /// * `name` - The name of the scanner.
    /// * `notifier` - A notifier to send scan results.
    /// * `timer` - A timer for scan intervals.
    /// * `state` - Shared state of the scanner.
    ///
    /// # Errors
    /// Returns an error if the scanner cannot be initialized.
    pub fn new(
        name: &'a str,
        notifier: Notifier,
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

    /// Performs a BLE scan.
    ///
    /// # Errors
    /// Returns an error if the scan fails.
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
    /// Polls the BLE scanner for devices.
    ///
    /// This function continuously scans for BLE devices and notifies the results.
    ///
    /// # Errors
    /// Returns an error if the scan or notification fails.
    fn poll(&mut self) -> Result<!> {
        block_on(async {
            loop {
                self.timer.delay(SCAN_FREQ).await?;

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

                self.notifier.notify(trigger)?;
            }
        })
    }
}
