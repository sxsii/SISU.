use sysinfo::{Disks, /*Process, */System};
use winreg::{
    enums::HKEY_LOCAL_MACHINE,
    RegKey,
};
use windows::{
    Win32::Graphics::Dxgi::{
        CreateDXGIFactory1, IDXGIFactory1, DXGI_ADAPTER_DESC1, DXGI_ADAPTER_FLAG_SOFTWARE,
    }
};
// use std::{thread, time::Duration};
use serde::Serialize; 

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_computer_specs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");


}

// for comp specs

#[allow(dead_code)]
#[derive(Debug, Serialize)]
struct ComputerSpecs {
    os: String,
    cpu: String,
    ram: String,
    storage_used: String,
    storage_total: String,
    gpu: Vec<String>,
    directx: String,
}

#[tauri::command]
fn get_computer_specs() -> ComputerSpecs {
    let mut sys = System::new_all();
    sys.refresh_all();

    ComputerSpecs {
        os: get_os_info(),
        cpu: get_processor_info(&sys),
        ram: get_memory_info(&sys),
        storage_used: get_storage_used(),
        storage_total: get_storage_total(),
        gpu: get_graphics_info(),
        directx: get_directx_version(),
    }
}


// helper functions
fn get_os_info() -> String {
    System::long_os_version().unwrap_or_else(|| "Unknown OS".to_string())
}

fn get_processor_info(sys: &System) -> String {
    sys.cpus()
        .first()
        .map(|cpu| cpu.brand().to_string())
        .unwrap_or_else(|| "Unknown CPU".to_string())
}

fn get_memory_info(sys: &System) -> String {
    let total_ram = sys.total_memory();
    let (val, unit) = bytes_to_readable(total_ram);
    format!("{} {}", val, unit)
}

fn get_storage_used() -> String {
    let disks = Disks::new_with_refreshed_list();
    let total: u64 = disks.list().iter().map(|d| d.total_space()).sum();
    let available: u64 = disks.list().iter().map(|d| d.available_space()).sum();
    let used = total - available;
    let (val, unit) = bytes_to_readable(used);
    format!("{} {}", val, unit)
}

fn get_storage_total() -> String {
    let disks = Disks::new_with_refreshed_list();
    let total: u64 = disks.list().iter().map(|d| d.total_space()).sum();
    let (val, unit) = bytes_to_readable(total);
    format!("{} {}", val, unit)
}

fn get_directx_version() -> String {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok(directx) = hklm.open_subkey("SOFTWARE\\Microsoft\\DirectX") {
        if let Ok(version) = directx.get_value::<String, _>("Version") {
            return match version.as_str() {
                "4.09.00.0904" => "DirectX 9.0c".to_string(),
                "4.10.0000.0904" => "DirectX 10".to_string(),
                "4.11.0000.0904" => "DirectX 11".to_string(),
                "4.12.0000.0904" => "DirectX 12".to_string(),
                _ => format!("Unknown ({})", version),
            };
        }
    }
    "Unknown".to_string()
}

fn get_graphics_info() -> Vec<String> {
    let mut gpus = Vec::new();
    unsafe {
        if let Ok(factory) = CreateDXGIFactory1::<IDXGIFactory1>() {
            let mut i = 0;
            loop {
                match factory.EnumAdapters1(i) {
                    Ok(adapter) => {
                        if let Ok(desc) = adapter.GetDesc1() {
                            let name = String::from_utf16_lossy(
                                &desc.Description
                                    .iter()
                                    .take_while(|&&c| c != 0)
                                    .cloned()
                                    .collect::<Vec<u16>>(),
                            );

                            if (desc.Flags & DXGI_ADAPTER_FLAG_SOFTWARE.0 as u32) == 0
                                && desc.DedicatedVideoMemory > 0
                            {
                                let (vram_val, vram_unit) =
                                    bytes_to_readable(desc.DedicatedVideoMemory as u64);
                                gpus.push(format!("{} ({} {})", name, vram_val, vram_unit));
                            }
                        }
                        i += 1;
                    }
                    Err(_) => break,
                }
            }
        }
    }
    gpus
}

// smaller helper functions
#[allow(dead_code)]
fn collect_gpus(out: &mut Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let factory: IDXGIFactory1 = CreateDXGIFactory1()?;
        let mut i = 0;

        loop {
            match factory.EnumAdapters1(i) {
                Ok(adapter) => {
                    let desc: DXGI_ADAPTER_DESC1 = adapter.GetDesc1()?;
                    let name = String::from_utf16_lossy(
                        &desc.Description
                            .iter()
                            .take_while(|&&c| c != 0)
                            .cloned()
                            .collect::<Vec<u16>>(),
                    );

                    if (desc.Flags & DXGI_ADAPTER_FLAG_SOFTWARE.0 as u32) != 0
                        || desc.DedicatedVideoMemory == 0
                    {
                        i += 1;
                        continue;
                    }

                    out.push(name);
                    i += 1;
                }
                Err(_) => break,
            }
        }
    }
    Ok(())
}

fn bytes_to_readable(bytes: u64) -> (u64, &'static str) {
    let bytes_f = bytes as f64;

    let tb = bytes_f / 1024.0 / 1024.0 / 1024.0 / 1024.0;
    if tb >= 1.0 {
        return (tb.round() as u64, "TB");
    }

    let gb = bytes_f / 1024.0 / 1024.0 / 1024.0;
    if gb >= 1.0 {
        return (gb.round() as u64, "GB");
    }

    let mb = bytes_f / 1024.0 / 1024.0;
    if mb >= 1.0 {
        return (mb.round() as u64, "MB");
    }

    let kb = bytes_f / 1024.0;
    if kb >= 1.0 {
        return (kb.round() as u64, "KB");
    }

    (bytes, "B")
}