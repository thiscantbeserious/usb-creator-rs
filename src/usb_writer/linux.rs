// src/usb_writer/linux.rs
use super::*;

use std::process::{Command, Output};
use std::str;

use byte_unit::Byte;
use serde_json;

#[cfg(test)]
use mockall::automock;

pub struct LinuxUsbWriter;

#[cfg_attr(test, automock)]
impl LinuxUsbWriter {

    /// We'll use `lsblk`in linux because that standardizes the output
    /// and can return the data in JSON-Format - so that we can parse it with serde_json.
    /// This way we don't have to manually iterate `/sys/block` and `/sys/bus/usb/devices`.
    fn cmd_lsblk_json() -> Result<String, UsbWriterError> {
        let output = Command::new("sh")
            .arg("-c")
            .arg("lsblk -J -o NAME,VENDOR,RM,MODEL,MOUNTPOINT,SIZE,TYPE")
            .output()
            .map_err(|_| {
                UsbWriterError::CommandExecutionError("Failed to execute lsblk".to_string())
            })?;
        if !output.status.success() {
            return Err(UsbWriterError::CommandExecutionError(
                "lsblk execution failed".into(),
            ));
        }    
        let output_str = String::from_utf8(output.stdout)
        .map_err(|e| UsbWriterError::ParseError(e.to_string()))?;

        Ok(output_str)
    }
}

impl UsbWriter for LinuxUsbWriter {
    fn list_devices() -> Result<Vec<UsbDisk>, UsbWriterError> {
        let lsblk_cmd_output_str = LinuxUsbWriter::cmd_lsblk_json()?;

        let mut devices = Vec::new();

        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&lsblk_cmd_output_str) {
            if let Some(blocks) = parsed["blockdevices"].as_array() {
                for blk in blocks {
                    let blocktype = blk["type"].as_str().unwrap_or_default().trim().to_string();
                    let removable = blk["rm"].as_bool().unwrap_or_default();
                    
                    //only work with removable disks to make sure we don't write to the root partition
                    if blocktype != "disk" || !removable  { 
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

                    if size == 0 {
                        continue;
                    }
                
                    let name = if !vendor.is_empty() || !model.is_empty() {
                        Some(format!("{}{}", vendor, model).trim().to_string())
                    } else {
                        None
                    };

                    devices.push(UsbDisk {
                        identifier: format!("/dev/{}", path),
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


// -----------------------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref TEST_CASE1_DATA: String = test::load_fixture_as_string("linux/test_case1.data");
    }

    #[test]
    fn test_cmd_lsblk_json() {

        let lsblk_output = MockLinuxUsbWriter::cmd_lsblk_json_context();
        lsblk_output
            .expect()
            .returning(|| Ok(TEST_CASE1_DATA.clone()));
    }
}
      