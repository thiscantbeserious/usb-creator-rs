// src/unix/usb_writer.rs
use super::*;

use std::process::Command;
use std::str;

use byte_unit::Byte;
use serde_json;

pub struct BSDUsbWriter;

impl UsbWriter for BSDUsbWriter {
    fn list_devices() -> Result<Vec<UsbDisk>, UsbWriterError> {
        let mut devices = Vec::new();
        Ok(devices)
    }

    fn open_device(device_path: &str) -> Result<(), UsbWriterError> {
        use std::fs::OpenOptions;

        match OpenOptions::new().write(true).open(device_path) {
            Ok(_) => Ok(()),
            Err(e) => Err(UsbWriterError::OpenDeviceError(e.to_string())),
        }
    }
}
