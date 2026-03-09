#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::usart::{BufferedUart, Config};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use embedded_io_async::BufRead;
use nmea::Nmea;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USART1 => usart::BufferedInterruptHandler<peripherals::USART1>;
});

macro_rules! buffered_uart {
    ($p:expr, $uart:expr, $config:expr, $rx_pin:ident, $tx_pin:ident, $rx_buf:expr, $tx_buf:expr) => {{
        BufferedUart::new(
            $uart,
            $p.$rx_pin,
            $p.$tx_pin,
            $tx_buf,
            $rx_buf,
            Irqs,
            $config,
        )
        .unwrap()
    }}
}

async fn read_line<U: BufRead + Unpin>(
    uart: &mut U,
    line_buf: &mut heapless::String<512>,
) -> Result<(), ()> {
    line_buf.clear();

    loop {
        let buf = uart.fill_buf().await.map_err(|_| ())?;
        if let Some(pos) = buf.iter().position(|&b| b == b'\n') {
            line_buf
                .push_str(core::str::from_utf8(&buf[..=pos]).map_err(|_| ())?)
                .map_err(|_| ())?;
            uart.consume(pos + 1);
            break;
        } else {
            line_buf
                .push_str(core::str::from_utf8(buf).map_err(|_| ())?)
                .map_err(|_| ())?;
            let n = buf.len();
            uart.consume(n);
        }
    }
    Ok(())
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let mut config = Config::default();
    config.baudrate = 9600;

    let mut rx_buf = [0u8; 1024];
    let mut tx_buf = [0u8; 32];
    let mut uart = buffered_uart!(p, p.USART1, config, PA10, PA9, &mut rx_buf, &mut tx_buf);

    let mut line = heapless::String::<512>::new();

    loop {
        if read_line(&mut uart, &mut line).await.is_err() {
            error!("UART read error");
            break;
        }

        let sentence = line.trim_end_matches(&['\r', '\n'][..]);
        let mut nmea = Nmea::default();
        if nmea.parse(sentence).is_ok() {
            info!("{:?}", nmea);
        } else {
            info!("raw: {}", sentence);
        }
    }
}