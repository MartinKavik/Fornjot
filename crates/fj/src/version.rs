//! API for checking compatibility between the Fornjot app and a model

use std::{ffi::CString, os::raw::c_char};

/// The Fornjot package version
///
/// Can be used to check for compatibility between a model and the Fornjot app
/// that runs it.
///
/// This is just the version specified in the Cargo package, which will stay
/// constant between releases, even though changes are made throughout. A match
/// of this version does not conclusively determine that the app and a model are
/// compatible.
pub static VERSION_PKG: &str = env!("FJ_VERSION_PKG");

/// The full Fornjot version
///
/// Can be used to check for compatibility between a model and the Fornjot app
/// that runs it.
pub static VERSION_FULL: &str = env!("FJ_VERSION_FULL");

/// C-ABI-compatible representation of a version
///
/// Used by the Fornjot application to check for compatibility between a model
/// and the app.
#[repr(C)]
pub struct RawVersion(*mut c_char);

impl RawVersion {
    /// Convert the `RawVersion` into a string
    ///
    /// # Safety
    ///
    /// Must be a `RawVersion` returned from one of the hidden version functions
    /// in this module.
    pub fn into_string(self) -> String {
        unsafe {
            CString::from_raw(self.0)
                .into_string()
                .expect("Failed to convert RawVersion into String")
        }
    }
}

#[no_mangle]
extern "C" fn version_pkg() -> RawVersion {
    let version = CString::new(VERSION_PKG)
        .expect("Failed to convert VERSION_PKG into CString");
    RawVersion(version.into_raw())
}

#[no_mangle]
extern "C" fn version_full() -> RawVersion {
    let version = CString::new(VERSION_FULL)
        .expect("Failed to convert VERSION_FULL into CString");
    RawVersion(version.into_raw())
}
