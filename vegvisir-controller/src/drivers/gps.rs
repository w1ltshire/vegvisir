use crate::Irqs;
use defmt::{error, info};
use embassy_executor::Spawner;
use embassy_stm32::Peripherals;
use embassy_stm32::usart::{BufferedUart, Config, Error};
use embedded_io_async::{BufRead, ErrorType};
use nmea::Nmea;
use static_cell::StaticCell;
use crate::buffered_uart;
use crate::drivers::DriverResult;

/// GPS reading task
#[embassy_executor::task]
async fn run(mut uart: BufferedUart<'static>) {
	let mut line = heapless::String::<82>::new();

	loop {
		if read_line(&mut uart, &mut line).await.is_err() {
			error!("UART read error");
			break;
		}

		let sentence = line.trim_end_matches(&['\r', '\n'][..]);
		let mut nmea = Nmea::default();
		let _ = nmea.parse(sentence);
		info!("{:?}", nmea);
	}
}

/// Spawn GPS task
pub fn spawn(p: Peripherals, spawner: Spawner) {
	let mut config = Config::default();
	config.baudrate = 9600;

	let uart: BufferedUart = buffered_uart!(
		p,
		p.USART1,
		config,
		PA10,
		PA9,
		1024,
		1
	);

	spawner.spawn(run(uart)).unwrap();
}

/// Read a single line from the UART
async fn read_line<U: BufRead + Unpin>(
	uart: &mut U,
	line_buf: &mut heapless::String<82>,
) -> DriverResult<()> where Error: From<<U as ErrorType>::Error> {
	line_buf.clear();

	loop {
		let buf = uart
			.fill_buf()
			.await
			.map_err(|e| {
				<<U as ErrorType>::Error as Into<Error>>::into(e)
			})?;
		if let Some(pos) = buf.iter().position(|&b| b == b'\n') {
			line_buf
				.push_str(core::str::from_utf8(&buf[..=pos])?)?;
			uart.consume(pos + 1);
			break;
		} else {
			line_buf
				.push_str(core::str::from_utf8(buf)?)?;
			let n = buf.len();
			uart.consume(n);
		}
	}
	Ok(())
}