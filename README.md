# HWID Tool - Hardware Identifier Testing Utility

A Rust-based tool for white-hat security testing of HWID-based authentication systems. Allows you to specify exact hardware identifiers via a JSON config file to test how client software validates hardware fingerprints.

## Requirements

- **OS**: Windows 10 (1909+) or Windows 11, x64 only
- **Permissions**: Must run as Administrator (right-click > Run as administrator)
- **Build tools**: Rust toolchain + Visual Studio C++ build tools

## Building

```
cargo build --release
```

Binary output: `target\release\hwspoof.exe`

## Usage

**Always run as Administrator.** The tool modifies `HKEY_LOCAL_MACHINE` registry keys and uses IOCTLs that require elevation.

### Step 1: Dump current HWIDs (do this FIRST)

Before making any changes, capture the machine's current hardware IDs so you have a backup and a baseline:

```
hwspoof.exe --dump > original_hwids.json
```

This reads all hardware identifiers from the running system and saves them as a JSON config file. Keep this file safe — you'll need it to restore original values.

### Step 2: Create your test config

Either edit the dumped file, or generate a blank template:

```
hwspoof.exe --sample > template.json
```

Edit the JSON to set the exact HWIDs you want to test with. You can include only the sections you need — omit any section to skip that component entirely.

### Step 3: Apply the config

```
hwspoof.exe your_config.json
```

This writes the specified hardware IDs to the system. A reboot is recommended for full propagation.

### Step 4: Restore original HWIDs

When you're done testing, apply your backup to restore the original values:

```
hwspoof.exe original_hwids.json
```

Reboot after restoring.

## Commands

| Command | Description |
|---|---|
| `hwspoof.exe config.json` | Apply HWIDs from the specified config file |
| `hwspoof.exe --dump` | Read current system HWIDs and print as JSON |
| `hwspoof.exe --sample` | Print a sample config template with example values |
| `hwspoof.exe` | Show usage help (does nothing to the system) |

## Config File Reference

All fields are optional. Omit a section entirely to skip that component.

```json
{
  "disk": {
    "drive_index": 0,
    "serial": "WD-WCC7K0EXAMPLE01"
  },
  "mac": {
    "adapter_name": "Ethernet",
    "address": "00:1B:21:AB:CD:EF"
  },
  "system_uuid": {
    "uuid": "550E8400-E29B-41D4-A716-446655440000",
    "product_id": "00330-80000-00000-AA123"
  },
  "motherboard": {
    "serial": "MB-0012345678",
    "uuid": "A1B2C3D4-E5F6-4789-ABCD-EF0123456789",
    "manufacturer": "ASUSTeK COMPUTER INC.",
    "product_name": "PRIME Z390-A"
  },
  "cpu": {
    "brand_string": "Intel(R) Core(TM) i7-9700K CPU @ 3.60GHz",
    "family": 6,
    "model": 158,
    "stepping": 10
  },
  "gpu": {
    "vendor_id": 4318,
    "device_id": 8708
  },
  "volume": {
    "drive_letter": "C",
    "serial": 2882343476
  },
  "bios": {
    "vendor": "American Megatrends Inc.",
    "version": "1.50",
    "date": "01/01/2023"
  },
  "network": {
    "hostname": "DESKTOP-ABC1234",
    "domain_name": "WORKGROUP",
    "dhcp_hostname": "DESKTOP-ABC1234",
    "netbios_name": "DESKTOP-ABC123"
  },
  "acpi": {
    "oem_id": "ALASKA",
    "oem_table_id": "A M I  "
  },
  "pci_hide": {
    "device_ids": ["PCI\\VEN_15AD", "PCI\\VEN_80EE"]
  },
  "wmi": {
    "intercept_classes": [
      "Win32_DiskDrive",
      "Win32_BaseBoard",
      "Win32_BIOS",
      "Win32_ComputerSystemProduct",
      "Win32_NetworkAdapter",
      "Win32_VideoController",
      "Win32_Processor"
    ]
  }
}
```

## Config Fields Explained

### Where to find values on a target machine

| Field | Command to query |
|---|---|
| `disk.serial` | `wmic diskdrive get serialnumber` |
| `mac.adapter_name` | `netsh interface show interface` |
| `mac.address` | `getmac /v` |
| `system_uuid.uuid` | `wmic csproduct get uuid` |
| `system_uuid.product_id` | Registry: `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\ProductId` |
| `motherboard.serial` | `wmic baseboard get serialnumber` |
| `motherboard.manufacturer` | `wmic baseboard get manufacturer` |
| `motherboard.product_name` | `wmic baseboard get product` |
| `cpu.brand_string` | `wmic cpu get name` |
| `cpu.family` | `wmic cpu get family` |
| `cpu.model` | `wmic cpu get model` |
| `cpu.stepping` | `wmic cpu get stepping` |
| `gpu.vendor_id` | Device Manager > GPU > Hardware IDs (`VEN_XXXX`, convert hex to decimal) |
| `gpu.device_id` | Device Manager > GPU > Hardware IDs (`DEV_XXXX`, convert hex to decimal) |
| `volume.serial` | `vol C:` (convert hex serial to decimal) |
| `bios.vendor` | `wmic bios get manufacturer` |
| `bios.version` | `wmic bios get smbiosbiosversion` |
| `bios.date` | `wmic bios get releasedate` |
| `network.hostname` | `hostname` |
| `network.netbios_name` | Max 15 characters |

### Notes

- `gpu.vendor_id` and `gpu.device_id` are **decimal** integers in the JSON (e.g., NVIDIA `0x10DE` = `4318`, RTX 3090 `0x2204` = `8708`)
- `volume.serial` is a **decimal** u32 (e.g., `ABCD-1234` hex = `2882343476` decimal)
- `network.netbios_name` is truncated to 15 characters per the NetBIOS spec
- `acpi.oem_id` is max 6 characters, `acpi.oem_table_id` is max 8 characters

## Typical Workflow

```
REM 1. On the target machine, dump its HWIDs
hwspoof.exe --dump > target_machine.json

REM 2. On your test machine, dump originals as backup
hwspoof.exe --dump > my_original.json

REM 3. Apply the target machine's identity
hwspoof.exe target_machine.json

REM 4. Reboot, then test the client's software

REM 5. When done, restore your originals
hwspoof.exe my_original.json

REM 6. Reboot to finalize restoration
```
