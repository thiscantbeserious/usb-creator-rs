// src/usb_writer/macos.rs
use super::*;

use std::process::{Command, Output};
use std::str;

use byte_unit::Byte;
use plist::Value;

pub struct MacOSUsbWriter;

impl MacOSUsbWriter {

    /// On MacOS we'll use the `diskutil` command to query the disk information.
    /// Since that command returns a plist, we can parse it with `plist` crate.
    /// It also includes metadata such as disk size. 
    /// We limit it to external devices via it's external flag.
    fn cmd_diskutil_plist() -> Result<Output, UsbWriterError> {
        let output = Command::new("diskutil")
            .arg("list")
            .arg("-plist")
            .arg("external")
            .output()
            .map_err(|_| {
                UsbWriterError::CommandExecutionError("Failed to execute diskutil".to_string())
            })?;
        if !output.status.success() {
            return Err(UsbWriterError::CommandExecutionError(
                "diskutil execution failed".into(),
            ));
        }
        Ok(output)
    }
}

impl UsbWriter for MacOSUsbWriter {
    fn list_devices() -> Result<Vec<UsbDisk>, UsbWriterError> {
        let output = MacOSUsbWriter::cmd_diskutil_plist()?;
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
                let size = Byte::parse_str(
                    disk.get("Size")
                        .and_then(Value::as_string)
                        .unwrap_or_default(),
                    true,
                )
                .map(|b| b.as_u64())
                .unwrap_or(0);
                let volume_name = disk
                    .get("VolumeName")
                    .and_then(Value::as_string)
                    .map(|s| s.to_owned());

                devices.push(UsbDisk {
                    identifier: format!("/dev/{}", device_identifier),
                    size: size,
                    name: volume_name.clone(), // Here assuming volume_name as name for simplification
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
