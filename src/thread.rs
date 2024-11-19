use esp_idf_hal::reset::restart;
use std::thread;

use crate::time::sleep;

pub fn failure() {
    // This program should run forever, until the device is powered off.
    // If something goes wrong and the program dies, we wait for a second and
    // then restart the device.
    sleep(1000);
    restart();
}

pub struct ExitGuard;

impl Drop for ExitGuard {
    fn drop(&mut self) {
        failure();
    }
}

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
