// network_stack.rs - Network Stack Modification
use std::io::Result;
use std::ptr::null_mut;

#[cfg(windows)]
use winapi::um::winreg::{RegOpenKeyExW, RegSetValueExW, RegCloseKey, HKEY_LOCAL_MACHINE};
#[cfg(windows)]
use winapi::um::winnt::{KEY_WRITE, REG_SZ};

pub fn spoof_network_stack() -> Result<()> {
    spoof_network_stack_custom(None, None, None, None)
}

pub fn spoof_network_stack_custom(
    custom_hostname: Option<String>,
    domain_name: Option<String>,
    dhcp_hostname: Option<String>,
    netbios_name: Option<String>,
) -> Result<()> {
    println!("[network_stack] Modifying network identifiers");

    modify_hostname_custom(custom_hostname)?;
    modify_domain_name(domain_name)?;
    modify_dhcp_hostname(dhcp_hostname)?;
    modify_netbios_name(netbios_name)?;

    Ok(())
}

fn modify_hostname_custom(custom_hostname: Option<String>) -> Result<()> {
    let new_hostname = custom_hostname
        .unwrap_or_else(|| format!("PC-{:08X}", rand::random::<u32>()));

    #[cfg(windows)]
    unsafe {
        use std::os::windows::ffi::OsStrExt;
        use std::ffi::OsStr;

        let path: Vec<u16> = OsStr::new("SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters")
            .encode_wide()
            .chain(Some(0))
            .collect();

        let mut hkey = null_mut();
        if RegOpenKeyExW(HKEY_LOCAL_MACHINE, path.as_ptr(), 0, KEY_WRITE, &mut hkey) == 0 {
            let name: Vec<u16> = OsStr::new("Hostname").encode_wide().chain(Some(0)).collect();
            let data: Vec<u16> = new_hostname.encode_utf16().collect();

            RegSetValueExW(
                hkey, name.as_ptr(), 0, REG_SZ,
                data.as_ptr() as *const u8,
                (data.len() * 2) as u32
            );

            RegCloseKey(hkey);
        }
    }

    println!("[network_stack] Hostname set to: {}", new_hostname);
    Ok(())
}

fn modify_domain_name(domain: Option<String>) -> Result<()> {
    let new_domain = domain.unwrap_or_else(|| "WORKGROUP".to_string());

    #[cfg(windows)]
    unsafe {
        use std::os::windows::ffi::OsStrExt;
        use std::ffi::OsStr;

        let path: Vec<u16> = OsStr::new("SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters")
            .encode_wide()
            .chain(Some(0))
            .collect();

        let mut hkey = null_mut();
        if RegOpenKeyExW(HKEY_LOCAL_MACHINE, path.as_ptr(), 0, KEY_WRITE, &mut hkey) == 0 {
            let name: Vec<u16> = OsStr::new("Domain").encode_wide().chain(Some(0)).collect();
            let data: Vec<u16> = new_domain.encode_utf16().collect();

            RegSetValueExW(
                hkey, name.as_ptr(), 0, REG_SZ,
                data.as_ptr() as *const u8,
                (data.len() * 2) as u32
            );

            RegCloseKey(hkey);
        }
    }

    println!("[network_stack] Domain name set to: {}", new_domain);
    Ok(())
}

fn modify_dhcp_hostname(dhcp_hostname: Option<String>) -> Result<()> {
    let new_dhcp_hostname = dhcp_hostname
        .unwrap_or_else(|| format!("PC-{:08X}", rand::random::<u32>()));

    #[cfg(windows)]
    unsafe {
        use std::os::windows::ffi::OsStrExt;
        use std::ffi::OsStr;

        let path: Vec<u16> = OsStr::new("SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters")
            .encode_wide()
            .chain(Some(0))
            .collect();

        let mut hkey = null_mut();
        if RegOpenKeyExW(HKEY_LOCAL_MACHINE, path.as_ptr(), 0, KEY_WRITE, &mut hkey) == 0 {
            let name: Vec<u16> = OsStr::new("DhcpHostname").encode_wide().chain(Some(0)).collect();
            let data: Vec<u16> = new_dhcp_hostname.encode_utf16().collect();

            RegSetValueExW(
                hkey, name.as_ptr(), 0, REG_SZ,
                data.as_ptr() as *const u8,
                (data.len() * 2) as u32
            );

            RegCloseKey(hkey);
        }
    }

    println!("[network_stack] DHCP hostname set to: {}", new_dhcp_hostname);
    Ok(())
}

fn modify_netbios_name(netbios: Option<String>) -> Result<()> {
    let name = netbios.unwrap_or_else(|| format!("PC-{:08X}", rand::random::<u32>()));
    // NetBIOS names are limited to 15 characters
    let truncated = &name[..name.len().min(15)];

    #[cfg(windows)]
    unsafe {
        use std::os::windows::ffi::OsStrExt;
        use std::ffi::OsStr;

        // Set ComputerName
        let path: Vec<u16> = OsStr::new("SYSTEM\\CurrentControlSet\\Control\\ComputerName\\ComputerName")
            .encode_wide()
            .chain(Some(0))
            .collect();

        let mut hkey = null_mut();
        if RegOpenKeyExW(HKEY_LOCAL_MACHINE, path.as_ptr(), 0, KEY_WRITE, &mut hkey) == 0 {
            let value_name: Vec<u16> = OsStr::new("ComputerName").encode_wide().chain(Some(0)).collect();
            let data: Vec<u16> = truncated.encode_utf16().collect();

            RegSetValueExW(
                hkey, value_name.as_ptr(), 0, REG_SZ,
                data.as_ptr() as *const u8,
                (data.len() * 2) as u32
            );

            RegCloseKey(hkey);
        }

        // Also set ActiveComputerName
        let active_path: Vec<u16> = OsStr::new("SYSTEM\\CurrentControlSet\\Control\\ComputerName\\ActiveComputerName")
            .encode_wide()
            .chain(Some(0))
            .collect();

        let mut active_hkey = null_mut();
        if RegOpenKeyExW(HKEY_LOCAL_MACHINE, active_path.as_ptr(), 0, KEY_WRITE, &mut active_hkey) == 0 {
            let value_name: Vec<u16> = OsStr::new("ComputerName").encode_wide().chain(Some(0)).collect();
            let data: Vec<u16> = truncated.encode_utf16().collect();

            RegSetValueExW(
                active_hkey, value_name.as_ptr(), 0, REG_SZ,
                data.as_ptr() as *const u8,
                (data.len() * 2) as u32
            );

            RegCloseKey(active_hkey);
        }
    }

    println!("[network_stack] NetBIOS name set to: {}", truncated);
    Ok(())
}
