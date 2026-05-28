#[cfg(all(target_os = "linux", not(target_os = "android")))]
use base64::{engine::general_purpose::STANDARD, Engine as _};
use log::{debug, trace};
use std::process::Command;

pub fn is_wsl() -> bool {
    std::path::Path::new("/proc/sys/fs/binfmt_misc/WSLInterop").exists()
}

fn get_roblox_credential_targets() -> Vec<String> {
    trace!("Getting all Roblox credential targets from Windows host");

    let output = match Command::new("cmd.exe")
        .args(["/c", "cmdkey /list | findstr roblox"])
        .output()
    {
        Ok(output) if output.status.success() => output,
        _ => {
            debug!("Failed to get credential targets");
            return Vec::new();
        }
    };
    let targets: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| {
            let trimmed_line = line.trim();
            if trimmed_line.starts_with("Target: LegacyGeneric:target=") {
                Some(trimmed_line.strip_prefix("Target: ").unwrap().to_string())
            } else {
                None
            }
        })
        .collect();

    debug!("Found {} Roblox credential targets", targets.len());
    targets
}

/// PowerShell script template for reading Windows credentials
/// This uses P/Invoke to call the Windows Credential Manager API directly
const CREDENTIAL_READER_SCRIPT: &str = r#"
$ErrorActionPreference = 'SilentlyContinue'
$VerbosePreference = 'SilentlyContinue'
$WarningPreference = 'SilentlyContinue'

# Define P/Invoke signatures for Windows Credential Manager
Add-Type @'
using System;
using System.Runtime.InteropServices;
using System.Text;

public class CredMan {
    [DllImport("advapi32.dll", CharSet = CharSet.Unicode)]
    public static extern bool CredRead(
        string target, 
        int type, 
        int reservedFlag, 
        out IntPtr credentialPtr
    );
    
    [DllImport("advapi32.dll")]
    public static extern bool CredFree([In] IntPtr cred);
    
    [StructLayout(LayoutKind.Sequential, CharSet = CharSet.Unicode)]
    public struct CREDENTIAL {
        public int Flags;
        public int Type;
        public IntPtr TargetName;
        public IntPtr Comment;
        public System.Runtime.InteropServices.ComTypes.FILETIME LastWritten;
        public int CredentialBlobSize;
        public IntPtr CredentialBlob;
        public int Persist;
        public int AttributeCount;
        public IntPtr Attributes;
        public IntPtr TargetAlias;
        public IntPtr UserName;
    }
}
'@ *> $null

# Try to read the credential
try {
    $credPtr = [IntPtr]::Zero
    $success = [CredMan]::CredRead('TARGET_PLACEHOLDER', 1, 0, [ref]$credPtr)
    
    if ($success) {
        $cred = [System.Runtime.InteropServices.Marshal]::PtrToStructure(
            $credPtr, 
            [type][CredMan+CREDENTIAL]
        )
        
        # Extract password bytes from the credential blob
        $passwordBytes = [byte[]]::new($cred.CredentialBlobSize)
        [System.Runtime.InteropServices.Marshal]::Copy(
            $cred.CredentialBlob, 
            $passwordBytes, 
            0, 
            $cred.CredentialBlobSize
        )
        
        # Convert bytes to string and output
        $password = [System.Text.Encoding]::UTF8.GetString($passwordBytes)
        [CredMan]::CredFree($credPtr)
        Write-Host $password -NoNewline
    }
} catch { }
"#;

fn get_credential(target: &str) -> Option<String> {
    trace!(
        "Attempting to read credential '{}' from Windows host",
        target
    );

    let script = CREDENTIAL_READER_SCRIPT.replace("TARGET_PLACEHOLDER", target);

    // PowerShell requires commands to be encoded as UTF-16 Little Endian, then Base64.
    // This is the most reliable way to pass a complex script from any shell (like WSL's bash).
    let utf16_script: Vec<u8> = script.encode_utf16().flat_map(u16::to_le_bytes).collect();
    let encoded_script = STANDARD.encode(utf16_script);

    let output = Command::new("powershell.exe")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-EncodedCommand",
            &encoded_script,
        ])
        .output()
        .ok()?;

    if !output.status.success() {
        trace!("PowerShell command failed for credential '{}'", target);
        return None;
    }

    let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if !result.is_empty() {
        debug!("Successfully retrieved credential '{}'", target);
        Some(result)
    } else {
        trace!("Failed to retrieve credential '{}' (empty result)", target);
        None
    }
}

fn find_credential_by_pattern(pattern: &str) -> Option<String> {
    let targets = get_roblox_credential_targets();
    targets
        .iter()
        .find(|target| target.contains(pattern))
        .and_then(|target| get_credential(target))
}

fn get_user_id() -> Option<String> {
    trace!("Looking for Roblox user ID in credential manager");

    if let Some(user_id) = find_credential_by_pattern("RobloxStudioAuthuserid") {
        debug!("Found user ID: {}", user_id);
        Some(user_id)
    } else {
        trace!("No user ID found in credential manager");
        None
    }
}

pub fn get_roblosecurity_cookie() -> Option<String> {
    trace!("Looking for ROBLOSECURITY cookie in credential manager");

    // First try to get user-specific cookie
    if let Some(user_id) = get_user_id() {
        debug!("Looking for user-specific cookie for user ID: {}", user_id);
        if let Some(cookie) =
            find_credential_by_pattern(&format!("RobloxStudioAuth.ROBLOSECURITY{}", user_id))
        {
            debug!("Found user-specific ROBLOSECURITY cookie");
            return Some(cookie);
        }
    }

    // Fallback to any ROBLOSECURITY cookie
    debug!("Looking for any ROBLOSECURITY cookie");
    if let Some(cookie) = find_credential_by_pattern("RobloxStudioAuth.ROBLOSECURITY") {
        debug!("Found fallback ROBLOSECURITY cookie");
        Some(cookie)
    } else {
        trace!("No ROBLOSECURITY cookie found in credential manager");
        None
    }
}

/// Read a value from Windows Registry via reg.exe
pub fn read_registry(key_path: &str, value_name: &str) -> Option<String> {
    trace!(
        "Attempting to read registry key '{}\\{}' from Windows host",
        key_path,
        value_name
    );

    let output = Command::new("reg.exe")
        .args(["query", &format!("HKCU\\{}", key_path), "/v", value_name])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .find_map(|line| {
            // Split the line by whitespace. The typical format is:
            // <ValueName>    <REG_TYPE>    <Value>
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 && parts[0] == value_name && parts[1] == "REG_SZ" {
                // Join the remaining parts to handle multi-word values
                let value = parts[2..].join(" ");
                debug!("Successfully read registry value");
                Some(value)
            } else {
                None
            }
        })
}
