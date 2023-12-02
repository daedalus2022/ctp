mod ffi_utils;
mod mdapi;
mod pb;
mod service;

pub use ffi_utils::*;
pub use mdapi::*;

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::*;

    #[test]
    fn get_api_version_works() {
        assert_eq!(ctp_sys::get_api_version().is_some(), true);
        // assert_eq!("v6.6.5_20210924 14:18:43.576",get_api_version().unwrap());
    }
}
