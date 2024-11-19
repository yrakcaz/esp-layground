use anyhow::anyhow;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::{convert::TryFrom, num::NonZeroU32};

#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum Trigger {
    ButtonPressed = 1 << 0,
    TimerTicked = 1 << 1,
    DeviceFoundActive = 1 << 2,
    DeviceFoundInactive = 1 << 3,
    DeviceNotFound = 1 << 4,
}

impl TryFrom<NonZeroU32> for Trigger {
    type Error = anyhow::Error;

    fn try_from(val: NonZeroU32) -> Result<Self, Self::Error> {
        Trigger::try_from(val.get())
            .map_err(|_| anyhow!("Invalid trigger value: {}", val))
    }
}

impl TryFrom<Trigger> for NonZeroU32 {
    type Error = anyhow::Error;

    fn try_from(trigger: Trigger) -> Result<Self, Self::Error> {
        NonZeroU32::new(trigger.into())
            .ok_or_else(|| anyhow!("Invalid value for NonZeroU32"))
    }
}
