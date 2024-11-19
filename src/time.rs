use esp_idf_hal::delay::FreeRtos;

pub fn sleep(ms: u32) {
    FreeRtos::delay_ms(ms);
}

pub fn yield_now() {
    sleep(10);
}
