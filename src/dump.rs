// dump.rs - Dump Current System Hardware IDs
// Reads all hardware identifiers from the running system and outputs
// a ready-to-use HwidConfig JSON that can be loaded on another machine.

use crate::config::*;
use std::io::Result;

pub fn dump_current_hwids() -> Result<HwidConfig> {
    let config = HwidConfig {
        disk: dump_disk_config(),
        mac: dump_mac_config(),
        system_uuid: dump_system_uuid_config(),
        motherboard: dump_motherboard_config(),
        cpu: dump_cpu_config(),
        gpu: dump_gpu_config(),
        volume: dump_volume_config(),
        bios: dump_bios_config(),
        network: dump_network_config(),
        acpi: dump_acpi_config(),
        pci_hide: None,
        wmi: None,
    };

    Ok(config)
}

// --- Disk Serial ---

fn dump_disk_config() -> Option<DiskConfig> {
    let serial = wmic_query("diskdrive", "SerialNumber");
    Some(DiskConfig {
        drive_index: 0,
        serial: serial.map(|s| s.trim().to_string()),
    })
}

// --- MAC Address ---

fn dump_mac_config() -> Option<MacConfig> {
    let adapter_name = first_network_adapter().unwrap_or_else(|| "Ethernet".to_string());
    let mac = get_mac_for_adapter(&adapter_name);
    Some(MacConfig {
        adapter_name,
        address: mac,
    })
}

fn first_network_adapter() -> Option<String> {
    #[cfg(windows)]
    {
        use std::process::Command;
        if let Ok(output) = Command::new("netsh")
            .args(&["interface", "show", "interface"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines().skip(3) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() > 3 && parts[1] == "Connected" {
                    return Some(parts[3..].join(" "));
                }
            }
        }
    }
    None
}

fn get_mac_for_adapter(adapter_name: &str) -> Option<String> {
    #[cfg(windows)]
    {
        use std::process::Command;
        if let Ok(output) = Command::new("getmac").args(&["/v", "/fo", "csv", "/nh"]).output() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if line.to_lowercase().contains(&adapter_name.to_lowercase()) {
                    let parts: Vec<&str> = line.split(',').collect();
                    if parts.len() > 2 {
                        let mac = parts[2].trim_matches('"').trim().to_string();
                        if !mac.is_empty() && mac != "N/A" {
                            return Some(mac);
                        }
                    }
                }
            }
        }
    }
    None
}

// --- System UUID ---

fn dump_system_uuid_config() -> Option<SystemUuidConfig> {
    let uuid = wmic_query("csproduct", "UUID");
    let product_id = reg_query(
        "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion",
        "ProductId",
    );
    Some(SystemUuidConfig {
        uuid: uuid.map(|s| s.trim().to_string()),
        product_id: product_id,
    })
}

// --- Motherboard ---

fn dump_motherboard_config() -> Option<MotherboardConfig> {
    let serial = wmic_query("baseboard", "SerialNumber");
    let uuid = wmic_query("csproduct", "UUID");
    let manufacturer = wmic_query("baseboard", "Manufacturer");
    let product = wmic_query("baseboard", "Product");
    Some(MotherboardConfig {
        serial: serial.map(|s| s.trim().to_string()),
        uuid: uuid.map(|s| s.trim().to_string()),
        manufacturer: manufacturer.map(|s| s.trim().to_string()),
        product_name: product.map(|s| s.trim().to_string()),
    })
}

// --- CPU ---

fn dump_cpu_config() -> Option<CpuConfig> {
    let brand = wmic_query("cpu", "Name");
    let family = wmic_query("cpu", "Family").and_then(|s| s.trim().parse::<u32>().ok());
    // WMIC reports "Model" but CPUID model needs to come from the actual CPUID leaf.
    // We read what WMIC reports as a reasonable approximation.
    let model = wmic_query("cpu", "Model").and_then(|s| s.trim().parse::<u32>().ok());
    let stepping = wmic_query("cpu", "Stepping").and_then(|s| s.trim().parse::<u32>().ok());
    Some(CpuConfig {
        brand_string: brand.map(|s| s.trim().to_string()),
        family,
        model,
        stepping,
    })
}

// --- GPU ---

fn dump_gpu_config() -> Option<GpuConfig> {
    // Read PCI hardware ID from WMIC to extract VEN_ and DEV_ values
    #[cfg(windows)]
    {
        use std::process::Command;
        if let Ok(output) = Command::new("wmic")
            .args(&["path", "Win32_VideoController", "get", "PNPDeviceID", "/value"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                let line = line.trim();
                if line.starts_with("PNPDeviceID=") {
                    let pnp_id = &line["PNPDeviceID=".len()..];
                    let vid = extract_pci_id(pnp_id, "VEN_");
                    let did = extract_pci_id(pnp_id, "DEV_");
                    if vid.is_some() || did.is_some() {
                        return Some(GpuConfig {
                            vendor_id: vid,
                            device_id: did,
                        });
                    }
                }
            }
        }
    }
    None
}

fn extract_pci_id(pnp_id: &str, prefix: &str) -> Option<u16> {
    if let Some(start) = pnp_id.find(prefix) {
        let hex_start = start + prefix.len();
        let hex_str: String = pnp_id[hex_start..].chars().take(4).collect();
        u16::from_str_radix(&hex_str, 16).ok()
    } else {
        None
    }
}

// --- Volume Serial ---

fn dump_volume_config() -> Option<VolumeConfig> {
    #[cfg(windows)]
    {
        use std::process::Command;
        if let Ok(output) = Command::new("cmd").args(&["/c", "vol", "C:"]).output() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Output like: "Volume Serial Number is ABCD-1234"
            for line in stdout.lines() {
                if let Some(pos) = line.find(" is ") {
                    let serial_str = line[pos + 4..].trim().replace("-", "");
                    if let Ok(serial) = u32::from_str_radix(&serial_str, 16) {
                        return Some(VolumeConfig {
                            drive_letter: 'C',
                            serial: Some(serial),
                        });
                    }
                }
            }
        }
    }
    Some(VolumeConfig {
        drive_letter: 'C',
        serial: None,
    })
}

// --- BIOS ---

fn dump_bios_config() -> Option<BiosConfig> {
    let vendor = wmic_query("bios", "Manufacturer");
    let version = wmic_query("bios", "SMBIOSBIOSVersion");
    let date = wmic_query("bios", "ReleaseDate");
    // Only return if we got at least the vendor
    let vendor = vendor.map(|s| s.trim().to_string())?;
    Some(BiosConfig {
        vendor,
        version: version.map(|s| s.trim().to_string()).unwrap_or_default(),
        date: date.map(|s| {
            // WMIC date format: 20230101000000.000000+000 -> 01/01/2023
            let s = s.trim();
            if s.len() >= 8 {
                format!("{}/{}/{}", &s[4..6], &s[6..8], &s[0..4])
            } else {
                s.to_string()
            }
        }).unwrap_or_default(),
    })
}

// --- Network ---

fn dump_network_config() -> Option<NetworkConfig> {
    let hostname = hostname_query();
    let domain = reg_query(
        "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters",
        "Domain",
    );
    let computer_name = reg_query(
        "SYSTEM\\CurrentControlSet\\Control\\ComputerName\\ComputerName",
        "ComputerName",
    );
    Some(NetworkConfig {
        hostname: hostname,
        domain_name: domain,
        dhcp_hostname: hostname_query(), // typically matches hostname
        netbios_name: computer_name,
    })
}

fn hostname_query() -> Option<String> {
    #[cfg(windows)]
    {
        use std::process::Command;
        if let Ok(output) = Command::new("hostname").output() {
            let name = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !name.is_empty() {
                return Some(name);
            }
        }
    }
    None
}

// --- ACPI ---

fn dump_acpi_config() -> Option<AcpiConfig> {
    // ACPI OEM info can be read from registry BIOS description
    let oem = reg_query("HARDWARE\\DESCRIPTION\\System\\BIOS", "SystemManufacturer");
    // Return a basic config; exact ACPI table OEM IDs require kernel access
    Some(AcpiConfig {
        oem_id: oem.map(|s| {
            // Truncate to 6 chars for ACPI OEM ID field
            let trimmed = s.trim();
            trimmed[..trimmed.len().min(6)].to_string()
        }),
        oem_table_id: None,
    })
}

// --- Helpers ---

/// Run a WMIC query and return the first non-empty value
fn wmic_query(alias: &str, property: &str) -> Option<String> {
    #[cfg(windows)]
    {
        use std::process::Command;
        if let Ok(output) = Command::new("wmic")
            .args(&[alias, "get", property, "/value"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let prefix = format!("{}=", property);
            for line in stdout.lines() {
                let line = line.trim();
                if line.starts_with(&prefix) {
                    let value = line[prefix.len()..].trim().to_string();
                    if !value.is_empty() {
                        return Some(value);
                    }
                }
            }
        }
    }
    #[cfg(not(windows))]
    {
        let _ = (alias, property);
    }
    None
}

/// Read a string value from the Windows registry
fn reg_query(path: &str, value_name: &str) -> Option<String> {
    #[cfg(windows)]
    {
        use std::process::Command;
        // Use reg.exe query which works without unsafe code
        let full_path = format!("HKLM\\{}", path);
        if let Ok(output) = Command::new("reg")
            .args(&["query", &full_path, "/v", value_name])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                let line = line.trim();
                if line.contains(value_name) {
                    // Format: "    ValueName    REG_SZ    Data"
                    let parts: Vec<&str> = line.splitn(3, "REG_").collect();
                    if parts.len() >= 2 {
                        // After "REG_SZ    " or "REG_DWORD    " comes the actual data
                        if let Some(data_start) = parts[1].find("    ") {
                            let data = parts[1][data_start..].trim().to_string();
                            if !data.is_empty() {
                                return Some(data);
                            }
                        }
                    }
                }
            }
        }
    }
    #[cfg(not(windows))]
    {
        let _ = (path, value_name);
    }
    None
}
