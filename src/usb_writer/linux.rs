// src/unix/usb_writer.rs
use super::*;

use std::process::Command;
use std::str;

use byte_unit::Byte;
use serde_json;

pub struct LinuxUsbWriter;

impl UsbWriter for LinuxUsbWriter {
    fn list_devices() -> Result<Vec<UsbDisk>, UsbWriterError> {
        let output = Command::new("sh")
            .arg("-c")
            .arg("lsblk -J -o NAME,VENDOR,RM,MODEL,MOUNTPOINT,SIZE,TYPE")
            .output()
            .map_err(|_| {
                UsbWriterError::CommandExecutionError("Failed to execute lsblk".to_string())
            })?;

        let output_str: &str = str::from_utf8(&output.stdout)
            .map_err(|e| UsbWriterError::ParseError(e.to_string()))?;

        let mut devices = Vec::new();

        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(output_str) {
            if let Some(blocks) = parsed["blockdevices"].as_array() {
                for blk in blocks {
                    let blocktype = blk["type"].as_str().unwrap_or_default().trim().to_string();
                    let removable = blk["rm"].as_bool().unwrap_or_default();
                    if blocktype != "disk" || !removable  { //only work with removable disks to make sure we don't write to the root partition
                        continue;
                    }
                    let path = blk["name"].as_str().unwrap_or_default().to_string();
                    let vendor = blk["vendor"]
                        .as_str()
                        .unwrap_or_default()
                        .trim()
                        .to_string();
                    let model = blk["model"].as_str().unwrap_or_default().trim().to_string();
                    let size = Byte::parse_str(
                        &blk["size"].as_str().unwrap_or_default().replace(",", "."),
                        true,
                    )
                    .map(|b| b.as_u64())
                    .unwrap_or(0);
                
                    let name = if !vendor.is_empty() || !model.is_empty() {
                        Some(format!("{}{}", vendor, model).trim().to_string())
                    } else {
                        None
                    };

                    devices.push(UsbDisk {
                        path: format!("/dev/{}", path),
                        size,
                        name
                    });
                }
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
