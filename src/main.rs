// main.rs - HWID Spoofer Entry Point
// Supports loading exact hardware IDs from a JSON configuration file
// Usage: hwspoof.exe [config.json]
//   - With no arguments: generates random HWIDs (original behavior)
//   - With a config file: applies the exact HWIDs specified in the file
//   - With --sample: prints a sample config JSON to stdout

mod config;
mod disk_serial;
mod mac_address;
mod motherboard;
mod system_uuid;
mod cpu_id;
mod gpu_id;
mod pci_devices;
mod registry_clean;
mod wmi_spoof;
mod volume_serial;
mod network_stack;
mod bios_info;
mod acpi_tables;
mod driver_hooks;
mod evasion;

use config::HwidConfig;
use std::io::Result;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "--sample" {
        println!("{}", HwidConfig::generate_sample());
        return;
    }

    let config = if args.len() > 1 {
        match HwidConfig::load_from_file(&args[1]) {
            Ok(cfg) => {
                println!("[main] Loaded config from: {}", args[1]);
                cfg
            }
            Err(e) => {
                eprintln!("[main] Failed to load config: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        println!("[main] No config file specified, using random HWIDs");
        println!("[main] Use --sample to generate a sample config file");
        HwidConfig::default()
    };

    // Run anti-analysis checks
    if let Err(e) = evasion::check_debugger() {
        eprintln!("[main] Debugger check failed: {}", e);
    }
    if let Err(e) = evasion::check_vm() {
        eprintln!("[main] VM check failed: {}", e);
    }
    if let Err(e) = evasion::check_sandbox() {
        eprintln!("[main] Sandbox check failed: {}", e);
    }

    if let Err(e) = apply_hwid_config(&config) {
        eprintln!("[main] Error applying HWID config: {}", e);
        std::process::exit(1);
    }

    println!("[main] ═══════════════════════════════════════════");
    println!("[main] All HWID modifications complete");
    println!("[main] ═══════════════════════════════════════════");
}

fn apply_hwid_config(config: &HwidConfig) -> Result<()> {
    // 1. Disk Serial
    if let Some(ref disk) = config.disk {
        println!("[main] Applying disk serial spoof...");
        disk_serial::spoof_disk_serial(disk.drive_index, disk.serial.clone())?;
    } else {
        println!("[main] Applying disk serial spoof (random)...");
        disk_serial::spoof_disk_serial(0, None)?;
    }

    // 2. MAC Address
    if let Some(ref mac) = config.mac {
        println!("[main] Applying MAC address spoof...");
        mac_address::spoof_mac_address(&mac.adapter_name, mac.address.clone())?;
    } else {
        println!("[main] Applying MAC address spoof (random)...");
        mac_address::spoof_mac_address("Ethernet", None)?;
    }

    // 3. System UUID
    if let Some(ref sys_uuid) = config.system_uuid {
        println!("[main] Applying system UUID spoof...");
        system_uuid::spoof_system_uuid(sys_uuid.uuid.clone(), sys_uuid.product_id.clone())?;
    } else {
        println!("[main] Applying system UUID spoof (random)...");
        system_uuid::spoof_system_uuid(None, None)?;
    }

    // 4. Motherboard
    if let Some(ref mb) = config.motherboard {
        println!("[main] Applying motherboard spoof...");
        motherboard::spoof_motherboard(
            mb.serial.clone(),
            mb.uuid.clone(),
            mb.manufacturer.clone(),
            mb.product_name.clone(),
        )?;
    } else {
        println!("[main] Applying motherboard spoof (random)...");
        motherboard::spoof_motherboard(None, None, None, None)?;
    }

    // 5. CPU ID
    if let Some(ref cpu) = config.cpu {
        println!("[main] Applying CPU ID spoof...");
        cpu_id::spoof_cpu_id_custom(
            cpu.brand_string.clone(),
            cpu.family,
            cpu.model,
            cpu.stepping,
        )?;
    } else {
        println!("[main] Applying CPU ID spoof (random)...");
        cpu_id::spoof_cpu_id()?;
    }

    // 6. GPU ID
    if let Some(ref gpu) = config.gpu {
        println!("[main] Applying GPU ID spoof...");
        gpu_id::spoof_gpu_id(gpu.vendor_id, gpu.device_id)?;
    } else {
        println!("[main] Applying GPU ID spoof (random)...");
        gpu_id::spoof_gpu_id(None, None)?;
    }

    // 7. Volume Serial
    if let Some(ref vol) = config.volume {
        println!("[main] Applying volume serial spoof...");
        volume_serial::spoof_volume_serial(vol.drive_letter, vol.serial)?;
    } else {
        println!("[main] Applying volume serial spoof (random)...");
        volume_serial::spoof_volume_serial('C', None)?;
    }

    // 8. BIOS Info
    if let Some(ref bios) = config.bios {
        println!("[main] Applying BIOS info spoof...");
        bios_info::spoof_bios_info(&bios.vendor, &bios.version, &bios.date)?;
    } else {
        println!("[main] Applying BIOS info spoof (default)...");
        bios_info::spoof_bios_info("American Megatrends Inc.", "1.50", "01/01/2023")?;
    }

    // 9. Network Stack
    if let Some(ref net) = config.network {
        println!("[main] Applying network stack spoof...");
        network_stack::spoof_network_stack_custom(
            net.hostname.clone(),
            net.domain_name.clone(),
            net.dhcp_hostname.clone(),
            net.netbios_name.clone(),
        )?;
    } else {
        println!("[main] Applying network stack spoof (random)...");
        network_stack::spoof_network_stack()?;
    }

    // 10. ACPI Tables
    if let Some(ref acpi) = config.acpi {
        println!("[main] Applying ACPI table overrides...");
        acpi_tables::inject_acpi_override(acpi.oem_id.clone(), acpi.oem_table_id.clone())?;
    } else {
        println!("[main] Applying ACPI table overrides (default)...");
        acpi_tables::inject_acpi_override(None, None)?;
    }

    // 11. Registry cleanup
    println!("[main] Cleaning registry artifacts...");
    registry_clean::clean_registry_artifacts()?;

    // 12. PCI device hiding
    println!("[main] Hiding PCI devices...");
    let pci_devices_list = config.pci_hide.as_ref().map(|p| p.device_ids.clone());
    pci_devices::hide_pci_devices(pci_devices_list)?;

    // 13. Driver hooks
    println!("[main] Installing driver hooks...");
    driver_hooks::install_driver_hooks()?;

    // 14. WMI hooks
    println!("[main] Hooking WMI queries...");
    let wmi_classes = config.wmi.as_ref().and_then(|w| w.intercept_classes.clone());
    wmi_spoof::hook_wmi_queries(wmi_classes)?;

    Ok(())
}
