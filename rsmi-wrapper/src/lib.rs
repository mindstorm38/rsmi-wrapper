pub mod error;
pub mod flags;

use std::ffi::{OsStr, CString, CStr};
use std::mem::ManuallyDrop;

// Here we re-export the sys crate.
pub use rsmi_wrapper_sys as sys;
use rsmi_wrapper_sys::RsmiLib;

// Local uses
use crate::error::{RsmiError, rsmi_try, rsmi_sym};
use crate::flags::InitFlags;


#[cfg(not(target_os = "linux"))]
compile_error!("ROCm SMI wrapper is only supported on linux");

#[cfg(target_os = "linux")]
const LIB_PATH: &str = "librocm_smi64.so";


/// Safe wrapper around ROCm SMI library, documented at
/// [here](https://raw.githubusercontent.com/RadeonOpenCompute/rocm_smi_lib/master/rocm_smi/docs/ROCm_SMI_Manual.pdf).
pub struct Rsmi {
    lib: ManuallyDrop<RsmiLib>
}

impl Rsmi {

    pub fn init_with_path_and_flags<P: AsRef<OsStr>>(path: P, flags: InitFlags) -> Result<Self, RsmiError> {
        let lib = unsafe {
            let lib = RsmiLib::new(path)?;
            let sym = rsmi_sym(&lib.rsmi_init)?;
            rsmi_try(sym(flags.bits()))?;
            ManuallyDrop::new(lib)
        };
        Ok(Rsmi { lib })
    }

    pub fn init_with_flags(flags: InitFlags) -> Result<Self, RsmiError> {
        Self::init_with_path_and_flags(LIB_PATH, flags)
    }

    pub fn init_with_path<P: AsRef<OsStr>>(path: P) -> Result<Self, RsmiError> {
        Self::init_with_path_and_flags(path, InitFlags::empty())
    }

    /// Same as drop, but you can handle errors from `rsmi_shut_down`.
    pub fn shutdown(mut self) -> Result<(), RsmiError> {
        
        unsafe {
            let sym = rsmi_sym(&self.lib.rsmi_shut_down)?;
            rsmi_try(sym())?;
        }

        unsafe {
            // Drop the lib here because we forget 'self'.
            ManuallyDrop::drop(&mut self.lib);
        }

        // We forget 'self' to avoid running destructor of lib.
        std::mem::forget(self);

        Ok(())

    }

    /// Get the number of devices that have monitor information.
    /// 
    /// The number of devices which have monitors is returned. Monitors are 
    /// referenced by the index which can be between 0 and `count - 1`.
    #[doc(alias = "rsmi_num_monitor_devices")]
    pub fn monitor_devices_count(&self) -> Result<u32, RsmiError> {
        let sym = rsmi_sym(&self.lib.rsmi_num_monitor_devices)?;
        let mut count = 0;
        unsafe { 
            rsmi_try(sym(&mut count))?; 
        }
        Ok(count)
    }

    /// Get the device id associated with the device with provided device index.
    #[doc(alias = "rsmi_dev_id_get")]
    pub fn get_device_id(&self, device_index: u32) -> Result<u16, RsmiError> {
        let sym = rsmi_sym(&self.lib.rsmi_dev_id_get)?;
        let mut count = 0;
        unsafe { 
            rsmi_try(sym(device_index, &mut count))?; 
        }
        Ok(count)
    }

    /// Get the SKU for a desired device associated with the device with provided device index.
    #[doc(alias = "rsmi_dev_sku_get")]
    pub fn get_device_sku(&self, device_index: u32) -> Result<i8, RsmiError> {
        let sym = rsmi_sym(&self.lib.rsmi_dev_sku_get)?;
        let mut sku = 0;
        unsafe { 
            rsmi_try(sym(device_index, &mut sku))?; 
        }
        Ok(sku)
    }

    /// Get the device vendor id associated with the device with provided device index
    #[doc(alias = "rsmi_dev_vendor_id_get")]
    pub fn get_device_vendor_id(&self, device_index: u32) -> Result<u16, RsmiError> {
        let sym = rsmi_sym(&self.lib.rsmi_dev_vendor_id_get)?;
        let mut vendor_id = 0;
        unsafe { 
            rsmi_try(sym(device_index, &mut vendor_id))?; 
        }
        Ok(vendor_id)
    }

    fn get_device_string<S: From<u16>>(&self, device_index: u32, sym: unsafe extern "C" fn(u32, *mut i8, S) -> u32) -> Result<String, RsmiError> {
        const BUFFER_LEN: u16 = 256;
        let mut buffer = [0i8; BUFFER_LEN as usize];
        unsafe {
            rsmi_try(sym(device_index, buffer.as_mut_ptr(), BUFFER_LEN.into()))?;
            CStr::from_ptr(buffer.as_mut_ptr()).to_str().map_err(|_| RsmiError::InvalidUtf8).map(str::to_string)
        }
    }

    /// Get the name string of a gpu device.
    #[doc(alias = "rsmi_dev_name_get")]
    pub fn get_device_name(&self, device_index: u32) -> Result<String, RsmiError> {
        self.get_device_string(device_index, rsmi_sym(&self.lib.rsmi_dev_name_get)?)
    }

    /// Get the brand string of a gpu device.
    #[doc(alias = "rsmi_dev_brand_get")]
    pub fn get_device_brand(&self, device_index: u32) -> Result<String, RsmiError> {
        self.get_device_string(device_index, rsmi_sym(&self.lib.rsmi_dev_brand_get)?)
    }

    /// Get the name string for a give vendor ID.
    #[doc(alias = "rsmi_dev_vendor_name_get")]
    pub fn get_device_vendor_name(&self, device_index: u32) -> Result<String, RsmiError> {
        self.get_device_string(device_index, rsmi_sym(&self.lib.rsmi_dev_vendor_name_get)?)
    }

    /// Get the vram vendor string of a gpu device.
    #[doc(alias = "rsmi_dev_vram_vendor_get")]
    pub fn get_device_vram_vendor_name(&self, device_index: u32) -> Result<String, RsmiError> {
        self.get_device_string(device_index, rsmi_sym(&self.lib.rsmi_dev_vendor_name_get)?)
    }

    /// Get the vram vendor string of a gpu device.
    #[doc(alias = "rsmi_dev_serial_number_get")]
    pub fn get_device_serial_number(&self, device_index: u32) -> Result<String, RsmiError> {
        self.get_device_string(device_index, rsmi_sym(&self.lib.rsmi_dev_serial_number_get)?)
    }

    pub fn get_device_subsystem

}

impl Drop for Rsmi {
    fn drop(&mut self) {

        if let Ok(sym) = self.lib.rsmi_shut_down {
            unsafe { sym(); }
        }

        unsafe { 
            ManuallyDrop::drop(&mut self.lib); 
        }

    }
}
