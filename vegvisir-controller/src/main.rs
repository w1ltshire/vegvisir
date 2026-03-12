#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]
#![no_std]
#![no_main]

mod drivers;
mod phy;

use crate::drivers::gps::GPSDriver;
use embassy_executor::Spawner;
use embassy_stm32::{bind_interrupts, peripherals, usart};
#[allow(unused_imports)]
use {defmt_rtt as _, panic_probe as _};
use crate::drivers::Driver;

bind_interrupts!(struct Irqs {
    USART1 => usart::BufferedInterruptHandler<peripherals::USART1>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let gps_driver = GPSDriver::init(p);
    let _ = gps_driver.spawn(&spawner);
}
