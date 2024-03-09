// src/usb_writer.rs
pub mod bsd;
pub mod linux;
pub mod macos;
pub mod windows;

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum UsbWriterError {
    CommandExecutionError(String),
    ParseError(String),
    OpenDeviceError(String),
    // Other error variants here
}

impl fmt::Display for UsbWriterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UsbWriterError::CommandExecutionError(ref err) => {
                write!(f, "Command execution error: {}", err)
            }
            UsbWriterError::ParseError(ref err) => write!(f, "Parse error: {}", err),
            UsbWriterError::OpenDeviceError(ref err) => write!(f, "Open error: {}", err),
            // Handle other variants here
        }
    }
}

impl Error for UsbWriterError {}

#[derive(Debug)]
pub struct UsbDisk {
    pub name: String,
    pub vendor: String,
    pub model: String,
    pub mountpoint: Option<String>,
    pub size: u64,
    pub blocktype: String,
}

pub trait UsbWriter {
    fn list_devices() -> Result<Vec<UsbDisk>, UsbWriterError>;
    fn open_device(device_path: &str) -> Result<(), UsbWriterError>;
}
