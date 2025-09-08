#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use rand::Rng;
use std::io;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum UART_Error {
    UART_INIT_ERROR,
    UART_WRITE_ERROR,
    UART_READ_ERROR,
    NO_ERROR,
    NB_ERROR,
}

#[derive(Copy, Clone, Eq, Debug)]
pub enum UART_BaudRate {
    Baud9600 = 9600,
    Baud19200 = 19200,
    Baud38400 = 38400,
    Baud57600 = 57600,
    Baud115200 = 115200,
}

impl PartialEq for UART_BaudRate {
    fn eq(&self, other: &Self) -> bool {
        *self as u32 == *other as u32
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum UART_Parity {
    None,
    Even,
    Odd,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum UART_StopBits {
    One,
    Two,
}

#[derive(Clone, Debug, Eq)]
pub struct UART_Config {
    pub baud_rate: UART_BaudRate,
    pub parity: UART_Parity,
    pub stop_bits: UART_StopBits,
}

impl PartialEq for UART_Config {
    fn eq(&self, other: &Self) -> bool {
        self.baud_rate as u8 == other.baud_rate as u8
            && self.parity as u8 == other.parity as u8
            && self.stop_bits as u8 == other.stop_bits as u8
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UART {
    pub config: UART_Config,
    pub is_initialized: bool,
    pub TX_buffer: Vec<u8>,
    pub RX_buffer: Vec<u8>,
}

pub fn UART_init(config: UART_Config) -> Result<UART, UART_Error> {
    // Simulate initialization logic
    if config.baud_rate == UART_BaudRate::Baud9600 {
        Ok(UART {
            config,
            is_initialized: true,
            TX_buffer: Vec::new(),
            RX_buffer: Vec::new(),
        })
    } else {
        Err(UART_Error::UART_INIT_ERROR)
    }
}
