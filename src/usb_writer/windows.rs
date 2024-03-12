// src/usb_writer/windows.rs
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
    PropertyStandardQuery, StorageDeviceProperty, IOCTL_DISK_GET_DRIVE_GEOMETRY, IOCTL_STORAGE_QUERY_PROPERTY, STORAGE_DEVICE_DESCRIPTOR, STORAGE_PROPERTY_QUERY
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

    /// Will Query the Controlcode of the Volume with the given handle
    fn query_io_controlcode<T, U>(
        handle: HANDLE, 
        controlcode: u32, 
        input_buffer: Option<&T>, 
        output_buffer: Option<&U>
    ) -> Result<u32, Error> { // Returns bytes_returned or an error
        let mut bytes_returned: u32 = 0;
        unsafe {
            DeviceIoControl(
                handle,
                controlcode,
                input_buffer.map_or(None, |b| Some(b as *const _ as *mut _)), // Cast input_buffer to pointer
                input_buffer.map_or(0, |_| std::mem::size_of::<T>() as u32), // Input buffer size
                output_buffer.map_or(None, |b| Some(b as *const _ as *mut _)), // Cast output_buffer to pointer
                output_buffer.map_or(0, |_| std::mem::size_of::<U>() as u32), // Output buffer size
                Some(&mut bytes_returned as *mut _),
                None, // Overlapped not used
            )?;
            // TODO: THIS SHOULD NOT WORK PROPERLY
        }
        Ok(bytes_returned)
    }

    /// Will return the specific volume info for the given volume name
    /// Including metadata such as disk size
    fn query_volume_info(volume_name: String) -> Result<WindowsVolumeInfo, Error> {
        
        unsafe {


            // Open our handle to the volume.
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

                    let disk_removable_query:STORAGE_PROPERTY_QUERY = STORAGE_PROPERTY_QUERY {
                        PropertyId: StorageDeviceProperty,
                        QueryType: PropertyStandardQuery, 
                        AdditionalParameters: [0]
                    };

                    let mut disk_removable_buf: STORAGE_DEVICE_DESCRIPTOR = std::mem::zeroed(); 

                    WindowsUsbWriter::query_io_controlcode(
                        volume_handle, 
                        IOCTL_STORAGE_QUERY_PROPERTY, 
                        Some(&disk_removable_query),
                        Some(&mut disk_removable_buf)
                    )?;
                    
                    let volume_info = WindowsVolumeInfo {
                        volume_name,
                        volume_path: String::new(),
                        volume_size: 0,
                    };


                    // Make sure we close our handle to the volume.
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
        match WindowsUsbWriter::get_volume_names() {
            Ok(volume_list) => {
                for volume_name in volume_list {
                    let result = WindowsUsbWriter::query_volume_info(volume_name);
                }
            },
            Err(e) => {
                return Err(UsbWriterError::ListDevicesError(e.to_string()));
            }
        }
        Ok(vec![])
    }

    fn open_device(device_path: &str) -> Result<(), UsbWriterError> {
        Ok(())
    }
}
