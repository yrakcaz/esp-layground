use anyhow::Result;

/// A trait representing a poller that performs periodic tasks.
///
/// # Errors
/// This trait's `poll` method returns an error if the polling operation fails.
pub trait Poller {
    /// Polls for periodic tasks.
    ///
    /// # Errors
    /// Returns an error if the polling operation fails.
    fn poll(&mut self) -> Result<!>;
}

/// A trait representing a switch that can toggle its state.
///
/// # Errors
/// This trait's `toggle` method returns an error if the toggle operation fails.
pub trait Switch {
    /// Toggles the state of the switch.
    ///
    /// # Errors
    /// Returns an error if the toggle operation fails.
    fn toggle(&mut self) -> Result<()>;
}
