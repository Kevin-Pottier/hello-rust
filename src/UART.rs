#![allow(dead_code)]

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum UartError {
    InitError,
    WriteError,
    ReadError,
    NoError,
    NbError,
}

#[derive(Copy, Clone, Eq, Debug)]
pub enum UartBaudRate {
    Baud9600 = 9600,
    Baud19200 = 19200,
    Baud38400 = 38400,
    Baud57600 = 57600,
    Baud115200 = 115200,
}

impl PartialEq for UartBaudRate {
    fn eq(&self, other: &Self) -> bool {
        *self as u32 == *other as u32
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum UartParity {
    None,
    Even,
    Odd,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum UartStopBits {
    One,
    Two,
}

#[derive(Clone, Debug, Eq)]
pub struct UartConfig {
    pub baud_rate: UartBaudRate,
    pub parity: UartParity,
    pub stop_bits: UartStopBits,
}

impl PartialEq for UartConfig {
    fn eq(&self, other: &Self) -> bool {
        self.baud_rate as u8 == other.baud_rate as u8
            && self.parity as u8 == other.parity as u8
            && self.stop_bits as u8 == other.stop_bits as u8
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Uart {
    pub config: UartConfig,
    pub is_initialized: bool,
    pub tx_buffer: Vec<u8>,
    pub rx_buffer: Vec<u8>,
}

pub fn uart_init(config: UartConfig) -> Result<Uart, UartError> {
    // Simulate initialization logic
    if config.baud_rate == UartBaudRate::Baud9600 {
        Ok(Uart {
            config,
            is_initialized: true,
            tx_buffer: Vec::new(),
            rx_buffer: Vec::new(),
        })
    } else {
        Err(UartError::InitError)
    }
}
