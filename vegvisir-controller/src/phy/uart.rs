/// Create a [`embassy_stm32::usart::BufferedUart`] instance
///
/// # Example
/// ```rust,no_run
/// let mut config = Config::default();
/// config.baudrate = 9600;
///
/// let uart: BufferedUart = buffered_uart!(
///         p,
///         p.USART1,
///         config,
///         PA10,
///         PA9,
///         1024,
///         32
/// );
/// ```
#[macro_export]
macro_rules! buffered_uart {
    ($uart:expr, $config:expr, $rx_pin:expr, $tx_pin:expr, $rx_size:expr, $tx_size:expr) => {{
	    static RX_BUF: StaticCell<[u8; $rx_size]> = StaticCell::new();
        static TX_BUF: StaticCell<[u8; $tx_size]> = StaticCell::new();

		let rx_buf: &'static mut [u8; $rx_size] = RX_BUF.init([0; $rx_size]);
		let tx_buf: &'static mut [u8; $tx_size] = TX_BUF.init([0; $tx_size]);

        BufferedUart::new(
            $uart,
            $rx_pin,
            $tx_pin,
            tx_buf,
            rx_buf,
            Irqs,
            $config,
        )
        .unwrap()
    }}
}