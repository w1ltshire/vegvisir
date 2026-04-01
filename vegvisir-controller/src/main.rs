#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]
#![no_std]
#![no_main]

mod drivers;
mod phy;

use defmt::info;
use crate::drivers::gps::GPSDriver;
use embassy_executor::Spawner;
use embassy_stm32::{bind_interrupts, interrupt, peripherals, spi, usart};
use embassy_stm32::gpio::{Level, Output, Pull, Speed};
use embassy_stm32::exti::{self, ExtiInput};
use embassy_stm32::time::khz;
use embassy_time::Delay;
use embedded_hal_bus::spi::ExclusiveDevice;
#[allow(unused_imports)]
use {defmt_rtt as _, panic_probe as _};
use lora_phy::iv::GenericSx127xInterfaceVariant;
use lora_phy::sx127x::{Sx1276, Sx127x};
use lora_phy::LoRa;
use lora_phy::{mod_params::*, sx127x};

bind_interrupts!(struct Irqs {
    USART1 => usart::BufferedInterruptHandler<peripherals::USART1>;
    EXTI0 => exti::InterruptHandler<interrupt::typelevel::EXTI0>;
});

const LORA_FREQUENCY_IN_HZ: u32 = 433_000_000; // warning: set this appropriately for the region

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut config = embassy_stm32::Config::default();
    config.rcc.hsi = true;
    config.rcc.sys = embassy_stm32::rcc::Sysclk::HSI;
    let p = embassy_stm32::init(Default::default());

    let nss = Output::new(p.PB2, Level::High, Speed::Low);
    let reset = Output::new(p.PB1, Level::High, Speed::Low);
    let irq = ExtiInput::new(p.PB0, p.EXTI0, Pull::Up, Irqs);

    let mut spi_cfg = spi::Config::default();
    spi_cfg.frequency = khz(200);
    let spi = spi::Spi::new(
        p.SPI1,
        p.PB3,
        p.PB5,
        p.PB4,
        p.DMA2_CH3,
        p.DMA2_CH2,
        spi_cfg,
    );

    let spi = ExclusiveDevice::new(spi, nss, Delay).unwrap();

    let config = sx127x::Config {
        chip: Sx1276,
        tcxo_used: true,
        rx_boost: false,
        tx_boost: true,
    };
    let iv = GenericSx127xInterfaceVariant::new(reset, irq, None, None).unwrap();
    let mut lora = LoRa::new(Sx127x::new(spi, iv, config), false, Delay).await.unwrap();

    let mdltn_params = {
        match lora.create_modulation_params(
            SpreadingFactor::_10,
            Bandwidth::_250KHz,
            CodingRate::_4_8,
            LORA_FREQUENCY_IN_HZ,
        ) {
            Ok(mp) => mp,
            Err(err) => {
                info!("Radio error = {}", err);
                return;
            }
        }
    };

    let mut tx_pkt_params = {
        match lora.create_tx_packet_params(4, false, true, false, &mdltn_params) {
            Ok(pp) => pp,
            Err(err) => {
                info!("Radio error = {}", err);
                return;
            }
        }
    };

    let buffer = [0x01u8, 0x02u8, 0x03u8];

    match lora
        .prepare_for_tx(&mdltn_params, &mut tx_pkt_params, 20, &buffer)
        .await
    {
        Ok(()) => {}
        Err(err) => {
            info!("Radio error = {}", err);
            return;
        }
    };

    match lora.tx().await {
        Ok(()) => {
            info!("TX DONE");
        }
        Err(err) => {
            info!("Radio error = {}", err);
            return;
        }
    };

    match lora.sleep(false).await {
        Ok(()) => info!("Sleep successful"),
        Err(err) => info!("Sleep unsuccessful = {}", err),
    }

    // TODO: improve error handling (send over lora? do some freaky stuff like blink an led or buzz?)
    GPSDriver::new(p.USART1, p.PA10, p.PA9, 9600).spawn(&spawner).unwrap();
}
