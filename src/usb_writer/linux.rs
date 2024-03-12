// src/unix/usb_writer.rs
use super::*;

use std::process::{Command, Output};
use std::str;

use byte_unit::Byte;
use serde_json;

pub struct LinuxUsbWriter;

// TODO example json for Mocking
// $lsblk -J -o NAME,VENDOR,RM,MODEL,MOUNTPOINT,SIZE,TYPE
// {
//    "blockdevices": [
//       {
//          "name": "loop0",
//          "vendor": null,
//          "rm": false,
//          "model": null,
//          "mountpoint": "/snap/bare/5",
//          "size": "4K",
//          "type": "loop"
//       },{
//          "name": "loop1",
//          "vendor": null,
//          "rm": false,
//          "model": null,
//          "mountpoint": "/snap/core18/2812",
//          "size": "55,7M",
//          "type": "loop"
//       },{
//          "name": "loop2",
//          "vendor": null,
//          "rm": false,
//          "model": null,
//          "mountpoint": "/snap/core22/1122",
//          "size": "74,2M",
//          "type": "loop"
//       },{
//          "name": "loop3",
//          "vendor": null,
//          "rm": false,
//          "model": null,
//          "mountpoint": "/snap/gnome-3-28-1804/198",
//          "size": "164,8M",
//          "type": "loop"
//       },{
//          "name": "loop4",
//          "vendor": null,
//          "rm": false,
//          "model": null,
//          "mountpoint": "/snap/gnome-42-2204/141",
//          "size": "497M",
//          "type": "loop"
//       },{
//          "name": "loop5",
//          "vendor": null,
//          "rm": false,
//          "model": null,
//          "mountpoint": "/snap/gtk-common-themes/1535",
//          "size": "91,7M",
//          "type": "loop"
//       },{
//          "name": "loop6",
//          "vendor": null,
//          "rm": false,
//          "model": null,
//          "mountpoint": "/snap/snap-store/959",
//          "size": "12,3M",
//          "type": "loop"
//       },{
//          "name": "loop7",
//          "vendor": null,
//          "rm": false,
//          "model": null,
//          "mountpoint": "/snap/snapd/20671",
//          "size": "40,4M",
//          "type": "loop"
//       },{
//          "name": "loop8",
//          "vendor": null,
//          "rm": false,
//          "model": null,
//          "mountpoint": "/snap/snapd/21184",
//          "size": "39,1M",
//          "type": "loop"
//       },{
//          "name": "loop9",
//          "vendor": null,
//          "rm": false,
//          "model": null,
//          "mountpoint": "/snap/snap-store/1113",
//          "size": "12,9M",
//          "type": "loop"
//       },{
//          "name": "loop10",
//          "vendor": null,
//          "rm": false,
//          "model": null,
//          "mountpoint": "/snap/core20/2182",
//          "size": "63,9M",
//          "type": "loop"
//       },{
//          "name": "loop11",
//          "vendor": null,
//          "rm": false,
//          "model": null,
//          "mountpoint": "/snap/flutter/145",
//          "size": "206,3M",
//          "type": "loop"
//       },{
//          "name": "sda",
//          "vendor": "Generic-",
//          "rm": true,
//          "model": "SD/MMC",
//          "mountpoint": null,
//          "size": "0B",
//          "type": "disk"
//       },{
//          "name": "sdb",
//          "vendor": "Phison  ",
//          "rm": true,
//          "model": "USB DISK 50X",
//          "mountpoint": null,
//          "size": "58,9G",
//          "type": "disk",
//          "children": [
//             {
//                "name": "sdb1",
//                "vendor": null,
//                "rm": true,
//                "model": null,
//                "mountpoint": "/media/doh/CCCOMA_X64FRE_DE-DE_DV9",
//                "size": "58,9G",
//                "type": "part"
//             },{
//                "name": "sdb2",
//                "vendor": null,
//                "rm": true,
//                "model": null,
//                "mountpoint": "/media/doh/UEFI_NTFS",
//                "size": "1M",
//                "type": "part"
//             }
//          ]
//       },{
//          "name": "zram0",
//          "vendor": null,
//          "rm": false,
//          "model": null,
//          "mountpoint": "[SWAP]",
//          "size": "16G",
//          "type": "disk"
//       },{
//          "name": "nvme0n1",
//          "vendor": null,
//          "rm": false,
//          "model": "Samsung SSD 970 EVO Plus 2TB",
//          "mountpoint": null,
//          "size": "1,8T",
//          "type": "disk",
//          "children": [
//             {
//                "name": "nvme0n1p1",
//                "vendor": null,
//                "rm": false,
//                "model": null,
//                "mountpoint": "/boot/efi",
//                "size": "1022M",
//                "type": "part"
//             },{
//                "name": "nvme0n1p2",
//                "vendor": null,
//                "rm": false,
//                "model": null,
//                "mountpoint": "/recovery",
//                "size": "4G",
//                "type": "part"
//             },{
//                "name": "nvme0n1p3",
//                "vendor": null,
//                "rm": false,
//                "model": null,
//                "mountpoint": null,
//                "size": "1,8T",
//                "type": "part",
//                "children": [
//                   {
//                      "name": "cryptdata",
//                      "vendor": null,
//                      "rm": false,
//                      "model": null,
//                      "mountpoint": null,
//                      "size": "1,8T",
//                      "type": "crypt",
//                      "children": [
//                         {
//                            "name": "data-root",
//                            "vendor": null,
//                            "rm": false,
//                            "model": null,
//                            "mountpoint": "/home",
//                            "size": "1,8T",
//                            "type": "lvm"
//                         }
//                      ]
//                   }
//                ]
//             },{
//                "name": "nvme0n1p4",
//                "vendor": null,
//                "rm": false,
//                "model": null,
//                "mountpoint": null,
//                "size": "4G",
//                "type": "part",
//                "children": [
//                   {
//                      "name": "cryptswap",
//                      "vendor": null,
//                      "rm": false,
//                      "model": null,
//                      "mountpoint": "[SWAP]",
//                      "size": "4G",
//                      "type": "crypt"
//                   }
//                ]
//             }
//          ]
//       }
//    ]
// }

impl LinuxUsbWriter {

    /// We'll use `lsblk`in linux because that standardizes the output
    /// and can return the data in JSON-Format - so that we can parse it with serde_json.
    /// This way we don't have to manually iterate `/sys/block` and `/sys/bus/usb/devices`.
    fn cmd_lsblk_json() -> Result<Output, UsbWriterError> {
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
        Ok(output)
    }
}

impl UsbWriter for LinuxUsbWriter {
    fn list_devices() -> Result<Vec<UsbDisk>, UsbWriterError> {
        let lsblk_cmd_output = LinuxUsbWriter::cmd_lsblk_json()?;
        let lsblk_cmd_output_str = String::from_utf8(lsblk_cmd_output.stdout)
            .map_err(|e| UsbWriterError::ParseError(e.to_string()))?;

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
