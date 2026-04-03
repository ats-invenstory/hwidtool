// config.rs - HWID Configuration Module
// Allows specifying exact hardware IDs instead of random generation
// Used for white-hat security testing of HWID-based authentication systems

use serde::{Deserialize, Serialize};
use std::io::{Result, Error, ErrorKind};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HwidConfig {
    #[serde(default)]
    pub disk: Option<DiskConfig>,

    #[serde(default)]
    pub mac: Option<MacConfig>,

    #[serde(default)]
    pub system_uuid: Option<SystemUuidConfig>,

    #[serde(default)]
    pub motherboard: Option<MotherboardConfig>,

    #[serde(default)]
    pub cpu: Option<CpuConfig>,

    #[serde(default)]
    pub gpu: Option<GpuConfig>,

    #[serde(default)]
    pub volume: Option<VolumeConfig>,

    #[serde(default)]
    pub bios: Option<BiosConfig>,

    #[serde(default)]
    pub network: Option<NetworkConfig>,

    #[serde(default)]
    pub acpi: Option<AcpiConfig>,

    #[serde(default)]
    pub pci_hide: Option<PciConfig>,

    #[serde(default)]
    pub wmi: Option<WmiConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskConfig {
    pub drive_index: u32,
    pub serial: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacConfig {
    pub adapter_name: String,
    pub address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemUuidConfig {
    pub uuid: Option<String>,
    /// Windows Product ID (e.g., "00330-80000-00000-AA123")
    #[serde(default)]
    pub product_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotherboardConfig {
    pub serial: Option<String>,
    pub uuid: Option<String>,
    /// BaseBoardManufacturer (e.g., "ASUSTeK COMPUTER INC.")
    #[serde(default)]
    pub manufacturer: Option<String>,
    /// BaseBoardProduct / SystemProductName (e.g., "PRIME Z390-A")
    #[serde(default)]
    pub product_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuConfig {
    pub brand_string: Option<String>,
    pub family: Option<u32>,
    pub model: Option<u32>,
    pub stepping: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuConfig {
    pub vendor_id: Option<u16>,
    pub device_id: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeConfig {
    pub drive_letter: char,
    pub serial: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiosConfig {
    pub vendor: String,
    pub version: String,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    #[serde(default)]
    pub hostname: Option<String>,
    #[serde(default)]
    pub domain_name: Option<String>,
    #[serde(default)]
    pub dhcp_hostname: Option<String>,
    #[serde(default)]
    pub netbios_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcpiConfig {
    /// OEM ID for ACPI tables (max 6 chars, e.g., "ALASKA")
    #[serde(default)]
    pub oem_id: Option<String>,
    /// OEM Table ID for ACPI tables (max 8 chars, e.g., "A M I  ")
    #[serde(default)]
    pub oem_table_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PciConfig {
    /// PCI device IDs to hide (e.g., ["PCI\\VEN_15AD", "PCI\\VEN_80EE"])
    pub device_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WmiConfig {
    /// WMI classes to intercept (e.g., ["Win32_DiskDrive", "Win32_BaseBoard"])
    #[serde(default)]
    pub intercept_classes: Option<Vec<String>>,
}

impl HwidConfig {
    pub fn load_from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| Error::new(ErrorKind::NotFound, format!("Config file not found: {}: {}", path, e)))?;
        Self::load_from_str(&content)
    }

    pub fn load_from_str(json: &str) -> Result<Self> {
        serde_json::from_str(json)
            .map_err(|e| Error::new(ErrorKind::InvalidData, format!("Invalid config JSON: {}", e)))
    }

    pub fn generate_sample() -> String {
        let sample = HwidConfig {
            disk: Some(DiskConfig {
                drive_index: 0,
                serial: Some("WD-WCC7K0EXAMPLE01".to_string()),
            }),
            mac: Some(MacConfig {
                adapter_name: "Ethernet".to_string(),
                address: Some("00:1B:21:AB:CD:EF".to_string()),
            }),
            system_uuid: Some(SystemUuidConfig {
                uuid: Some("550E8400-E29B-41D4-A716-446655440000".to_string()),
                product_id: Some("00330-80000-00000-AA123".to_string()),
            }),
            motherboard: Some(MotherboardConfig {
                serial: Some("MB-0012345678".to_string()),
                uuid: Some("A1B2C3D4-E5F6-4789-ABCD-EF0123456789".to_string()),
                manufacturer: Some("ASUSTeK COMPUTER INC.".to_string()),
                product_name: Some("PRIME Z390-A".to_string()),
            }),
            cpu: Some(CpuConfig {
                brand_string: Some("Intel(R) Core(TM) i7-9700K CPU @ 3.60GHz".to_string()),
                family: Some(0x6),
                model: Some(0x9E),
                stepping: Some(0xA),
            }),
            gpu: Some(GpuConfig {
                vendor_id: Some(0x10DE),
                device_id: Some(0x2204),
            }),
            volume: Some(VolumeConfig {
                drive_letter: 'C',
                serial: Some(0xABCD1234),
            }),
            bios: Some(BiosConfig {
                vendor: "American Megatrends Inc.".to_string(),
                version: "1.50".to_string(),
                date: "01/01/2023".to_string(),
            }),
            network: Some(NetworkConfig {
                hostname: Some("DESKTOP-ABC1234".to_string()),
                domain_name: Some("WORKGROUP".to_string()),
                dhcp_hostname: Some("DESKTOP-ABC1234".to_string()),
                netbios_name: Some("DESKTOP-ABC123".to_string()),
            }),
            acpi: Some(AcpiConfig {
                oem_id: Some("ALASKA".to_string()),
                oem_table_id: Some("A M I  ".to_string()),
            }),
            pci_hide: Some(PciConfig {
                device_ids: vec![
                    "PCI\\VEN_15AD".to_string(),
                    "PCI\\VEN_80EE".to_string(),
                ],
            }),
            wmi: Some(WmiConfig {
                intercept_classes: Some(vec![
                    "Win32_DiskDrive".to_string(),
                    "Win32_BaseBoard".to_string(),
                    "Win32_BIOS".to_string(),
                    "Win32_ComputerSystemProduct".to_string(),
                    "Win32_NetworkAdapter".to_string(),
                    "Win32_VideoController".to_string(),
                    "Win32_Processor".to_string(),
                ]),
            }),
        };

        serde_json::to_string_pretty(&sample).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_config_roundtrip() {
        let sample_json = HwidConfig::generate_sample();
        let parsed = HwidConfig::load_from_str(&sample_json).unwrap();
        assert!(parsed.disk.is_some());
        assert!(parsed.mac.is_some());
        assert!(parsed.cpu.is_some());
        assert!(parsed.gpu.is_some());
        assert!(parsed.acpi.is_some());
        assert!(parsed.pci_hide.is_some());
        assert!(parsed.wmi.is_some());
    }

    #[test]
    fn test_partial_config() {
        let json = r#"{ "disk": { "drive_index": 0, "serial": "TEST-1234" } }"#;
        let config = HwidConfig::load_from_str(json).unwrap();
        assert!(config.disk.is_some());
        assert!(config.mac.is_none());
        assert!(config.acpi.is_none());
        assert_eq!(config.disk.unwrap().serial.unwrap(), "TEST-1234");
    }

    #[test]
    fn test_empty_config() {
        let json = "{}";
        let config = HwidConfig::load_from_str(json).unwrap();
        assert!(config.disk.is_none());
        assert!(config.mac.is_none());
        assert!(config.acpi.is_none());
    }

    #[test]
    fn test_new_fields_backward_compat() {
        // Old-style config without new fields should still parse
        let json = r#"{
            "motherboard": { "serial": "MB-123" },
            "system_uuid": { "uuid": "550E8400-E29B-41D4-A716-446655440000" },
            "network": { "hostname": "PC-TEST" }
        }"#;
        let config = HwidConfig::load_from_str(json).unwrap();
        let mb = config.motherboard.unwrap();
        assert_eq!(mb.serial.unwrap(), "MB-123");
        assert!(mb.manufacturer.is_none());
        assert!(mb.product_name.is_none());
        let uuid = config.system_uuid.unwrap();
        assert!(uuid.product_id.is_none());
        let net = config.network.unwrap();
        assert!(net.domain_name.is_none());
    }
}
