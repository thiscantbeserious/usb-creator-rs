mod usb_writer;
mod utils;

use crate::usb_writer::UsbWriter;

#[cfg(target_os = "freebsd")]
use usb_writer::bsd::BSDUsbWriter as PlatformUsbWriter;
#[cfg(any(target_os = "linux"))]
use usb_writer::linux::LinuxUsbWriter as PlatformUsbWriter;
#[cfg(any(target_os = "macos"))]
use usb_writer::macos::MacOSUsbWriter as PlatformUsbWriter;
#[cfg(target_os = "windows")]
use usb_writer::windows::WindowsUsbWriter as PlatformUsbWriter;

fn main() {
    match PlatformUsbWriter::list_devices() {
        Ok(devices) => {
            println!("USB Devices:");
            for device in devices {
                println!("- {:?}", device);
            }
            // You could then open a device by its path or identifier
        }
        Err(e) => eprintln!("Error listing USB devices: {}", e),
    }
}