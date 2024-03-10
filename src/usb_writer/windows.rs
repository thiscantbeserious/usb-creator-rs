use super::*;

// https://codentium.com/guides/windows-dev/windows-drivers-in-rust-io-controls/

use ::windows::Win32::Storage::FileSystem::{
    FindFirstVolumeW, 
    FindNextVolumeW, 
    GetVolumePathNamesForVolumeNameW,
    CreateFileW, FILE_SHARE_READ, OPEN_EXISTING,
};
use ::windows::Win32::System::Ioctl::{
    IOCTL_DISK_GET_DRIVE_GEOMETRY,
    IOCTL_STORAGE_QUERY_PROPERTY, 
    STORAGE_PROPERTY_QUERY
};

use ::windows::Win32::System::IO:: {
    DeviceIoControl
};

pub struct WindowsUsbWriter;

impl UsbWriter for WindowsUsbWriter {
    fn list_devices() -> Result<Vec<UsbDisk>, UsbWriterError> {
        // Placeholder: Implement logic to list USB devices for Windows
        Ok(vec![])
    }

    fn open_device(device_path: &str) -> Result<(), UsbWriterError> {
        // Placeholder: Use `windows` crate as previously described
        Ok(())
    }
}
