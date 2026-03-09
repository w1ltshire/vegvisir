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

    let mut tx_buf = [0u8; 32];
    let mut rx_buf = [0u8; 1024];
    let mut uart = BufferedUart::new(
        p.USART1,
        p.PA10, // rx on the board, tx on the gps module (white on bn-180)
        p.PA9,  // tx on the board, rx on the gps module (green on bn-180)
        &mut tx_buf,
        &mut rx_buf,
        Irqs,
        config,
    )
        .unwrap();

    let mut line = heapless::String::<512>::new();

    loop {
        if read_line(&mut uart, &mut line).await.is_err() {
            error!("UART read error");
            continue;
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