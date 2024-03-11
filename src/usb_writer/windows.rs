use super::*;

// https://codentium.com/guides/windows-dev/windows-drivers-in-rust-io-controls/

use ::windows::Win32::Foundation::{
    CloseHandle, 
    ERROR_NO_MORE_FILES, 
    GENERIC_READ, 
    GENERIC_WRITE, 
    HANDLE
};

use ::windows::Win32::Storage::FileSystem::{
    CreateFileW, 
    FindFirstVolumeW, 
    FindNextVolumeW, 
    FindVolumeClose, 
    GetVolumePathNamesForVolumeNameW, 
    FILE_ATTRIBUTE_NORMAL, 
    FILE_SHARE_READ, 
    FILE_SHARE_WRITE, 
    OPEN_EXISTING
};
use ::windows::Win32::System::Ioctl::{
    IOCTL_DISK_GET_DRIVE_GEOMETRY, 
    IOCTL_STORAGE_QUERY_PROPERTY
};

use ::windows::Win32::System::IO::DeviceIoControl;

use ::windows::core::Error;
use ::windows::core::HSTRING;

pub struct WindowsUsbWriter {
    handle: HANDLE,
}

pub struct WindowsVolumeInfo {
    volume_name: String,
    volume_path: String,
    volume_size: u64,
}

impl WindowsUsbWriter {
    /// Will return all volume names on the system (unfiltered)
    fn get_volume_names() -> Result<Vec<String>, Error> {
        let mut volume_names = Vec::new();
        let mut volume_name_buf: [u16; 1024] = [0; 1024];
        unsafe {
            match FindFirstVolumeW(&mut volume_name_buf) {
                Ok(handle) => {
                    if handle.is_invalid() {
                        return Err(Error::from_win32());
                    }
                    loop {
                        // Push the current volume name into the vector.
                        // let current_volume_name = String::from_utf16_lossy(&volume_name_buf)
                        //     .trim_end_matches('\u{0}')
                        //     .to_string();
                        let current_volume_name = HSTRING::from_wide(&volume_name_buf).unwrap().to_string();

                        volume_names.push(current_volume_name);
                        if let Err(e) = FindNextVolumeW(handle, &mut volume_name_buf) {
                            if e.code() == ERROR_NO_MORE_FILES.into() {
                                break;
                            }
                            let _ = FindVolumeClose(handle);
                            return Err(e.into());
                        }
                    }
                    let _ = FindVolumeClose(handle);
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }

        Ok(volume_names)
    }

    /// Will return the specific volume info for the given volume name
    /// Including metadata such as disk size
    fn query_volume_info(volume_name: String) -> Result<WindowsVolumeInfo, Error> {
        
        unsafe {
            match CreateFileW( 
                &HSTRING::from(&volume_name),
                GENERIC_READ.0 as u32,
                FILE_SHARE_READ,
                None,
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                None
            ) {
                Ok(volume_handle) => {
                    if volume_handle.is_invalid() {
                        return Err(Error::from_win32());
                    }
                    let volume_info = WindowsVolumeInfo {
                        volume_name,
                        volume_path: String::new(),
                        volume_size: 0,
                    };
                    let _ = CloseHandle(volume_handle);
                    Ok(volume_info)

                },
                Err(e) => {
                    return Err(e.into());
                }
            }

            
        }
    }
}

impl UsbWriter for WindowsUsbWriter {
    fn list_devices() -> Result<Vec<UsbDisk>, UsbWriterError> {
        let volume_names = WindowsUsbWriter::get_volume_names();
        Ok(vec![])
    }

    fn open_device(device_path: &str) -> Result<(), UsbWriterError> {
        // Placeholder: Use `windows` crate as previously described
        Ok(())
    }
}
