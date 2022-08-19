// Implementation heavily based on https://github.com/rust-lang/cargo/blob/master/crates/credential/cargo-credential-wincred/src/main.rs

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::shared::minwindef::TRUE;
use winapi::um::wincred;

type Error = Box<dyn std::error::Error>;

/// Converts a string to a nul-terminated wide UTF-16 byte sequence.
fn wstr(s: &str) -> Vec<u16> {
    let mut wide: Vec<u16> = OsStr::new(s).encode_wide().collect();
    if wide.iter().any(|b| *b == 0) {
        panic!("nul byte in wide string");
    }
    wide.push(0);
    wide
}

pub fn get(target_name: &str) -> Result<String, Error> {
    let target_name = wstr(target_name);
    let mut p_credential: wincred::PCREDENTIALW = std::ptr::null_mut();
    unsafe {
        if wincred::CredReadW(
            target_name.as_ptr(),
            wincred::CRED_TYPE_GENERIC,
            0,
            &mut p_credential,
        ) != TRUE
        {
            return Err(
                format!("failed to fetch token: {}", std::io::Error::last_os_error()).into(),
            );
        }
        let bytes = std::slice::from_raw_parts(
            (*p_credential).CredentialBlob,
            (*p_credential).CredentialBlobSize as usize,
        );
        String::from_utf8(bytes.to_vec()).map_err(|_| "failed to convert token to UTF8".into())
    }
}
