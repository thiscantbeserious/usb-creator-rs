// src/unix/usb_writer.rs
use super::*;

use std::process::Command;
use std::str;
use plist::{Value, Error, from_bytes}; // Import the `Value` type and `from_bytes` function

pub struct MacOSUsbWriter;

impl UsbWriter for MacOSUsbWriter {
    fn list_devices() -> Result<Vec<UsbDisk>, UsbWriterError> {
        let output = Command::new("diskutil")
            .arg("list")
            .arg("external")
            .arg("-plist")
            .output()
            .map_err(|_| {
                UsbWriterError::CommandExecutionError("Failed to execute diskutil".to_string())
            })?;

        if !output.status.success() {
            return Err(UsbWriterError::CommandExecutionError(
                "diskutil execution failed".into(),
            ));
        }

        let plist: Value = plist::from_bytes(&output.stdout)
            .map_err(|e| UsbWriterError::ParseError(e.to_string()))?;

        let disk_list = plist
            .as_dictionary()
            .ok_or_else(|| UsbWriterError::ParseError("Invalid plist format".into()))?
            .get("AllDisksAndPartitions")
            .and_then(plist::Value::as_array)
            .ok_or_else(|| UsbWriterError::ParseError("Missing AllDisksAndPartitions".into()))?;

        let mut devices: Vec<UsbDisk> = Vec::new();

        for disk_value in disk_list {
            if let Some(disk) = disk_value.as_dictionary() {
                let device_identifier = disk
                    .get("DeviceIdentifier")
                    .and_then(Value::as_string)
                    .unwrap_or_default();
                let size = disk
                    .get("Size")
                    .and_then(Value::as_unsigned_integer)
                    .unwrap_or_default();
                let volume_name = disk
                    .get("VolumeName")
                    .and_then(Value::as_string)
                    .map(|s| s.to_owned());

                let mount_point = disk
                    .get("MountPoint")
                    .and_then(Value::as_string)
                    .map(|s| s.to_owned());

                devices.push(UsbDisk {
                    path: format!("/dev/{}", device_identifier),
                    size: size,
                    name: volume_name.clone(), // Here assuming volume_name as name for simplification
                                               // Further fields like `vendor` and `usbtype` require additional logic not directly available from diskutil
                });
            }
        }

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
