# ESP Playground

This project demonstrates the basic functionalities of the ESP32 microcontroller, specifically on the M5Stack Atom Lite development board. It serves as a playground for experimenting with various features of the ESP32, including GPIO, BLE, timers, and LEDs.

## Overview

The example in `main.rs` implements a simple state machine that integrates the following components:

- **Button Input**: A button is used to toggle the system state between "on" and "off."
- **BLE Scanner and Advertiser**: The system scans for nearby BLE devices and advertises its own state.
- **LED Control**: An LED is used to visually indicate the system state, with different colors and blinking patterns.
- **Timers**: Timers are used for periodic tasks, such as blinking the LED and scanning for BLE devices.

## How It Works

1. The button toggles the system state between "on" and "off."
2. When the system is "on," the BLE scanner searches for nearby devices, and the LED blinks to indicate activity.
3. The BLE advertiser broadcasts the system's state.
4. A state machine coordinates the interactions between these components.

This example demonstrates how to use the ESP-IDF framework with Rust to build embedded applications for the ESP32 platform.