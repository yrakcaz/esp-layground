use esp_idf_hal::reset::restart;
use std::thread;

use crate::time::sleep;

/// Handles program failure by restarting the device.
///
/// This function waits for a second and then restarts the device if the program encounters an error.
pub fn failure() {
    // This program should run forever, until the device is powered off.
    // If something goes wrong and the program dies, we wait for a second and
    // then restart the device.
    sleep(1000);
    restart();
}

/// A guard that ensures the program restarts on thread exit.
///
/// When the thread exits, the `Drop` implementation of this guard will call the `failure` function.
pub struct ExitGuard;

impl Drop for ExitGuard {
    /// Ensures the program restarts when the thread exits.
    fn drop(&mut self) {
        failure();
    }
}

/// Spawns a new thread with a failure guard.
///
/// # Arguments
/// * `f` - A closure to execute in the new thread.
///
/// # Returns
/// A `JoinHandle` for the spawned thread.
///
/// # Type Parameters
/// * `F` - The type of the closure.
/// * `T` - The return type of the closure.
pub fn spawn<F, T>(f: F) -> thread::JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    thread::spawn(move || {
        let _guard = ExitGuard;
        f()
    })
}
