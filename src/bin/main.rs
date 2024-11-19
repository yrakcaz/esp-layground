use anyhow::Result;
use esp_idf_hal::{
    gpio::PinDriver,
    prelude::Peripherals,
    rmt::{config::TransmitConfig, TxRmtDriver},
    task::notification::Notification,
    timer::{TimerConfig, TimerDriver},
};
use esp_idf_svc::log::EspLogger;
use std::sync::{Arc, Mutex};

use esp_layground::{
    ble::{Advertiser, Scanner},
    button::{Button, State},
    clock::Timer,
    infra::Poller,
    light::{Led, BLINK_FREQ},
    logic::StateMachine,
    thread::{spawn, ExitGuard},
};

const NAME: &str = "ESPlayground";

fn main() -> Result<()> {
    // FIXME main is a mess and should be reorganized + specifically naming of variables especially around timers and ble.

    // main() should never return. Restart the device if it does.
    let _guard = ExitGuard;

    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_hal::sys::link_patches();

    EspLogger::initialize_default();

    let notification = Notification::new();
    let button_notifier = notification.notifier();
    let ble_notifier = notification.notifier();
    let timer_notifier = notification.notifier();

    let peripherals = Peripherals::take()?;

    let channel_peripheral = peripherals.rmt.channel0;
    let led_peripheral = peripherals.pins.gpio27;
    let timer_peripheral = peripherals.timer00;
    let ble_timer_peripheral = peripherals.timer01;
    let button_peripheral = peripherals.pins.gpio39;

    let tx_rmt_cfg = TransmitConfig::new().clock_divider(1);
    let tx_rmt_driver =
        TxRmtDriver::new(channel_peripheral, led_peripheral, &tx_rmt_cfg)?;

    let timer_cfg = TimerConfig::new().auto_reload(true);
    let timer_driver = TimerDriver::new(timer_peripheral, &timer_cfg)?;
    let mut timer = Timer::new(timer_driver)?;
    timer.configure_interrupt(BLINK_FREQ, timer_notifier)?;

    // The two inputs to the state machine are the button and the BLE scanner.
    // These inputs are polled in separate threads. However, BLE scanning should
    // not run if the whole system is off. Consequently, the button also needs
    // to be an input to the BLE scanner. This cannot be done using the general
    // notification mechanism because it can have only one listener. Hence, we
    // use a shared state between the button and the BLE scanner.
    let button_state = Arc::new(Mutex::new(State::Off));

    let pin = PinDriver::input(button_peripheral)?;
    let mut button = Button::new(button_notifier, pin, Arc::clone(&button_state))?;
    spawn(move || button.poll());

    let ble_timer_driver = TimerDriver::new(ble_timer_peripheral, &timer_cfg)?;
    let ble_timer = Timer::new(ble_timer_driver)?;
    let mut scanner =
        Scanner::new(NAME, ble_notifier, ble_timer, Arc::clone(&button_state))?;
    spawn(move || scanner.poll());

    let advertiser = Advertiser::new(NAME)?;
    let led = Led::new(tx_rmt_driver)?;
    let mut sm = StateMachine::new(advertiser, led, timer, notification)?;
    sm.run()
}
