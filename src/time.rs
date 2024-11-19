use esp_idf_hal::delay::FreeRtos;

/// Delays execution for a specified number of milliseconds.
///
/// # Arguments
/// * `ms` - The number of milliseconds to delay.
pub fn sleep(ms: u32) {
    FreeRtos::delay_ms(ms);
}

/// Yields the current thread for a short duration.
///
/// This function is useful for cooperative multitasking.
pub fn yield_now() {
    sleep(10);
}
