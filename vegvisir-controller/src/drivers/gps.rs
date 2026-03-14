use crate::Irqs;
use defmt::error;
use embassy_executor::Spawner;
use embassy_stm32::interrupt::typelevel::Binding;
use embassy_stm32::Peri;
use embassy_stm32::usart::{BufferedInterruptHandler, BufferedUart, Config, Error, Instance, RxPin, TxPin};
use embedded_io_async::{BufRead, ErrorType};
use nmea::Nmea;
use crate::buffered_uart;
use crate::drivers::DriverResult;

/// NMEA Line type for convenience.
///
/// NMEA messages have a maximum length of 82 characters, including the $ or ! starting character and the ending \<LF>.
type NmeaLine = heapless::String<82>;

/// Structure representing GPS driver
pub struct GPSDriver {
    /// UART instance
    uart: BufferedUart<'static>
}

impl GPSDriver {
    /// Create a new instance of [`GPSDriver`]
    pub fn new<T: Instance>(
        usart: Peri<'static, T>,
        rx: Peri<'static, impl RxPin<T>>,
        tx: Peri<'static, impl TxPin<T>>,
        baudrate: u32
    ) -> Self where Irqs: Binding<<T as Instance>::Interrupt, BufferedInterruptHandler<T>> {
        let mut config = Config::default();
        config.baudrate = baudrate;

        let uart: BufferedUart = buffered_uart!(
			usart,
			config,
			rx,
			tx,
			1024,
			1
		);

        Self {
            uart
        }
    }

    /// Spawn the GPS driver task, consuming `self`
    pub fn spawn(self, spawner: &Spawner) -> DriverResult<()> {
        Ok(spawner.spawn(gps(self.uart))?)
    }
}

/// The GPS driver task itself.
///
/// Read line into a buffer with length of 82 and parse it.
#[embassy_executor::task]
async fn gps(mut uart: BufferedUart<'static>) {
    let mut line = NmeaLine::new();

    loop {
        if let Err(e) = read_line(&mut uart, &mut line).await {
            error!("UART read error: {}", e);
            break;
        }

        let sentence = line.trim_end_matches(&['\r', '\n'][..]);
        let mut nmea = Nmea::default();
        let _ = nmea.parse(sentence);
        //info!("lat:{}, lon:{}, fix:{:?}", nmea.latitude, nmea.longitude, nmea.fix_type);
    }
}

/// Read a single line from the UART
async fn read_line<U: BufRead + Unpin>(
    uart: &mut U,
    line_buf: &mut NmeaLine,
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