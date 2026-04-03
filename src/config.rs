// config.rs - HWID Configuration Module
// Allows specifying exact hardware IDs instead of random generation
// Security audit: demonstrates that an attacker can dictate specific HWIDs
// to impersonate a target machine's hardware fingerprint

use serde::{Deserialize, Serialize};
use std::io::{Result, Error, ErrorKind};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HwidConfig {
    /// Disk serial number configuration
    #[serde(default)]
    pub disk: Option<DiskConfig>,

    /// MAC address configuration
    #[serde(default)]
    pub mac: Option<MacConfig>,

    /// System UUID configuration
    #[serde(default)]
    pub system_uuid: Option<SystemUuidConfig>,

    /// Motherboard serial/UUID configuration
    #[serde(default)]
    pub motherboard: Option<MotherboardConfig>,

    /// CPU ID configuration
    #[serde(default)]
    pub cpu: Option<CpuConfig>,

    /// GPU ID configuration
    #[serde(default)]
    pub gpu: Option<GpuConfig>,

    /// Volume serial number configuration
    #[serde(default)]
    pub volume: Option<VolumeConfig>,

    /// BIOS information configuration
    #[serde(default)]
    pub bios: Option<BiosConfig>,

    /// Network stack configuration
    #[serde(default)]
    pub network: Option<NetworkConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskConfig {
    /// Drive index (e.g., 0 for PhysicalDrive0)
    pub drive_index: u32,
    /// Exact serial number to write (e.g., "WD-WCC1234567890AB")
    pub serial: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacConfig {
    /// Network adapter name
    pub adapter_name: String,
    /// Exact MAC address (e.g., "00:1B:21:AB:CD:EF")
    pub address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemUuidConfig {
    /// Exact UUID (e.g., "550E8400-E29B-41D4-A716-446655440000")
    pub uuid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotherboardConfig {
    /// Exact motherboard serial (e.g., "MB-0012345678")
    pub serial: Option<String>,
    /// Exact system UUID
    pub uuid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuConfig {
    /// CPU brand string (e.g., "Intel(R) Core(TM) i7-9700K CPU @ 3.60GHz")
    pub brand_string: Option<String>,
    /// CPU family (e.g., 0x6)
    pub family: Option<u32>,
    /// CPU model (e.g., 0x9E)
    pub model: Option<u32>,
    /// CPU stepping (e.g., 0xA)
    pub stepping: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuConfig {
    /// PCI vendor ID (e.g., 0x10DE for NVIDIA)
    pub vendor_id: Option<u16>,
    /// PCI device ID (e.g., 0x2204 for RTX 3090)
    pub device_id: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeConfig {
    /// Drive letter (e.g., 'C')
    pub drive_letter: char,
    /// Exact volume serial (32-bit, e.g., 0xABCD1234)
    pub serial: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiosConfig {
    /// BIOS vendor name (e.g., "American Megatrends Inc.")
    pub vendor: String,
    /// BIOS version string (e.g., "1.50")
    pub version: String,
    /// BIOS release date (e.g., "01/01/2023")
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Custom hostname (e.g., "DESKTOP-ABC1234")
    pub hostname: Option<String>,
}

impl HwidConfig {
    /// Load configuration from a JSON file
    pub fn load_from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| Error::new(ErrorKind::NotFound, format!("Config file not found: {}: {}", path, e)))?;
        Self::load_from_str(&content)
    }

    /// Parse configuration from a JSON string
    pub fn load_from_str(json: &str) -> Result<Self> {
        serde_json::from_str(json)
            .map_err(|e| Error::new(ErrorKind::InvalidData, format!("Invalid config JSON: {}", e)))
    }

    /// Generate a sample configuration file showing all available fields
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
            }),
            motherboard: Some(MotherboardConfig {
                serial: Some("MB-0012345678".to_string()),
                uuid: Some("A1B2C3D4-E5F6-4789-ABCD-EF0123456789".to_string()),
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
    }

    #[test]
    fn test_partial_config() {
        let json = r#"{ "disk": { "drive_index": 0, "serial": "TEST-1234" } }"#;
        let config = HwidConfig::load_from_str(json).unwrap();
        assert!(config.disk.is_some());
        assert!(config.mac.is_none());
        assert_eq!(config.disk.unwrap().serial.unwrap(), "TEST-1234");
    }

    #[test]
    fn test_empty_config() {
        let json = "{}";
        let config = HwidConfig::load_from_str(json).unwrap();
        assert!(config.disk.is_none());
        assert!(config.mac.is_none());
    }
}
