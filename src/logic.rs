use anyhow::Result;
use esp_idf_hal::{delay::BLOCK, task::notification::Notification};
use log::info;
use std::{fmt, num::NonZeroU32};

use crate::{
    ble::Advertiser,
    clock::Timer,
    color::{Rgb, GREEN, RED},
    infra::Switch,
    light::Led,
    trigger::Trigger,
};

macro_rules! func {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);

        match &name[..name.len() - 3].rfind(':') {
            Some(pos) => &name[pos + 1..name.len() - 3],
            None => &name[..name.len() - 3],
        }
    }};
}

#[derive(PartialEq)]
enum State {
    On,
    Off,
    ActiveDeviceNearby,
    InactiveDeviceNearby,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::On => write!(f, "On"),
            State::Off => write!(f, "Off"),
            State::ActiveDeviceNearby => write!(f, "ActiveDeviceNearby"),
            State::InactiveDeviceNearby => write!(f, "InactiveDeviceNearby"),
        }
    }
}

impl From<&State> for Rgb {
    fn from(state: &State) -> Self {
        match state {
            State::On => GREEN,
            State::Off => RED,
            State::ActiveDeviceNearby => GREEN,
            State::InactiveDeviceNearby => RED,
        }
    }
}

pub struct StateMachine<'a> {
    advertiser: Advertiser<'a>,
    led: Led<'a>,
    timer: Timer<'a>,
    notification: Notification,
    state: State,
}

impl<'a> StateMachine<'a> {
    pub fn new(
        advertiser: Advertiser<'a>,
        led: Led<'a>,
        timer: Timer<'a>,
        notification: Notification,
    ) -> Result<Self> {
        let state = State::Off;

        let mut led = led;
        led.set_color((&state).into())?;
        led.on()?;

        Ok(Self {
            advertiser,
            led,
            timer,
            notification,
            state,
        })
    }

    fn handle_button_pressed(&mut self) -> Result<()> {
        info!("{}", func!());

        self.state = match self.state {
            State::Off => State::On,
            _ => State::Off,
        };

        self.advertiser.toggle()
    }

    fn handle_timer_ticked(&mut self) -> Result<()> {
        info!("{}", func!());

        match self.state {
            State::ActiveDeviceNearby | State::InactiveDeviceNearby => {
                self.led.toggle()
            } // Blinking
            _ => Ok(()),
        }
    }

    fn handle_device_found_active(&mut self) -> Result<()> {
        info!("{}", func!());

        self.state = match self.state {
            State::Off => State::Off,
            _ => State::ActiveDeviceNearby,
        };

        Ok(())
    }

    fn handle_device_found_inactive(&mut self) -> Result<()> {
        info!("{}", func!());

        self.state = match self.state {
            State::Off => State::Off,
            _ => State::InactiveDeviceNearby,
        };

        Ok(())
    }

    fn handle_device_not_found(&mut self) -> Result<()> {
        info!("{}", func!());

        self.state = match self.state {
            State::Off => State::Off,
            _ => State::On,
        };

        Ok(())
    }

    fn handle_notification(&mut self, notification: NonZeroU32) -> Result<()> {
        let notification = notification.get();
        info!("{}: 0b{notification:b}, state: {}", func!(), self.state);

        if notification & u32::try_from(Trigger::ButtonPressed)? != 0 {
            self.handle_button_pressed()?;
        } else if notification & u32::try_from(Trigger::DeviceFoundActive)? != 0 {
            self.handle_device_found_active()?;
        } else if notification & u32::try_from(Trigger::DeviceFoundInactive)? != 0 {
            self.handle_device_found_inactive()?;
        } else if notification & u32::try_from(Trigger::DeviceNotFound)? != 0 {
            self.handle_device_not_found()?;
        } else if notification & u32::try_from(Trigger::TimerTicked)? != 0 {
            self.handle_timer_ticked()?;
        } else {
            Err(anyhow::Error::msg(
                "Invalid notification: 0b{notification:b}",
            ))?;
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            let notification = self.notification.wait(BLOCK); // FIXME should the whole notification mechanism be abstracted too?
            if let Some(notification) = notification {
                self.handle_notification(notification)?;

                self.led.set_color((&self.state).into())?;
                if self.state == State::On || self.state == State::Off {
                    self.timer.off()?;
                    self.led.on()?;
                } else {
                    self.timer.on()?;
                }
            }
        }
    }
}
