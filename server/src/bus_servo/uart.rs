use std::error::Error;
use std::time::Duration;
use std::fmt;
use rppal::uart::{Parity, Uart};


#[derive(Debug)]
struct UartError(String);

impl fmt::Display for UartError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Uart Error: {}", self.0)
    }
}

impl Error for UartError {}

pub fn read_uart() -> Result<u8, Box<dyn Error>> {
    let mut uart = Uart::new(115_200, Parity::None, 8, 1)?;
    uart.set_read_mode(1, Duration::default())?;

    let mut buffer = [0u8; 1];

    if uart.read(&mut buffer)? > 0 {
        return Ok(buffer[0]);
    } else {
        return Err(Box::new(UartError("Can't read from uart".into())));
      
    }
}

pub fn read_uart_terminal() -> Result<(), Box<dyn Error>> {
    let mut uart = Uart::new(115_200, Parity::None, 8, 1)?;
    uart.set_read_mode(1, Duration::default())?;

    let mut buffer = [0u8; 1];
    loop {
        if uart.read(&mut buffer)? > 0 {
            println!("Received byte: {}", buffer[0]);
        }
    }
}
