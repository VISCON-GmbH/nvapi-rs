// Tests in this file call apis and simply expect them to return without causing
// segfaults, etc.

// Not actually testing results. Just calling the api.
#![allow(unused_must_use)]

use core::panic;

extern crate nvapi;

#[test]
fn physicalgpu_enumerate() {
    if let Ok(_) = nvapi::initialize() {
        match nvapi::PhysicalGpu::enumerate() {
            Ok(gpus) => {
                println!("Found {} GPUs", gpus.len());
                for (i, gpu) in gpus.iter().enumerate() {
                    println!("GPU {}: {:?}", i, gpu.handle());
                }
            }
            Err(e) => println!("Failed to enumerate GPUs: {:?}", e),
        }
    }
}

#[test]
fn physicalgpu_basic_info() {
    if let Ok(_) = nvapi::initialize() {
        if let Ok(gpus) = nvapi::PhysicalGpu::enumerate() {
            for (i, gpu) in gpus.iter().enumerate() {
                println!("=== GPU {} Basic Info ===", i);
                
                if let Ok(name) = gpu.full_name() {
                    println!("Full Name: {}", name);
                }
                
                if let Ok(short_name) = gpu.short_name() {
                    println!("Short Name: {}", short_name);
                }
                
                if let Ok(bios) = gpu.vbios_version_string() {
                    println!("BIOS Version: {}", bios);
                }
                
                if let Ok(driver) = gpu.driver_model() {
                    println!("Driver Model: {:?}", driver);
                }
                
                if let Ok(gpu_id) = gpu.gpu_id() {
                    println!("GPU ID: {}", gpu_id);
                }
                
                if let Ok(pci) = gpu.pci_identifiers() {
                    println!("PCI Identifiers: {:?}", pci);
                }
                
                if let Ok(board) = gpu.board_number() {
                    println!("Board Number: {:?}", board);
                }
                
                if let Ok(system_type) = gpu.system_type() {
                    println!("System Type: {:?}", system_type);
                }
            }
        }
    }
}

#[test]
fn physicalgpu_hardware_info() {
    if let Ok(_) = nvapi::initialize() {
        if let Ok(gpus) = nvapi::PhysicalGpu::enumerate() {
            for (i, gpu) in gpus.iter().enumerate() {
                println!("=== GPU {} Hardware Info ===", i);
                
                if let Ok(cores) = gpu.core_count() {
                    println!("Core Count: {}", cores);
                }
                
                if let Ok(pipes) = gpu.shader_pipe_count() {
                    println!("Shader Pipe Count: {}", pipes);
                }
                
                if let Ok(sub_pipes) = gpu.shader_sub_pipe_count() {
                    println!("Shader Sub-Pipe Count: {}", sub_pipes);
                }
                
                if let Ok(ram_type) = gpu.ram_type() {
                    println!("RAM Type: {:?}", ram_type);
                }
                
                if let Ok(ram_maker) = gpu.ram_maker() {
                    println!("RAM Maker: {:?}", ram_maker);
                }
                
                if let Ok(bus_width) = gpu.ram_bus_width() {
                    println!("RAM Bus Width: {}", bus_width);
                }
                
                if let Ok(bank_count) = gpu.ram_bank_count() {
                    println!("RAM Bank Count: {}", bank_count);
                }
                
                if let Ok(partition_count) = gpu.ram_partition_count() {
                    println!("RAM Partition Count: {}", partition_count);
                }
                
                if let Ok(foundry) = gpu.foundry() {
                    println!("Foundry: {:?}", foundry);
                }
            }
        }
    }
}

#[test]
fn physicalgpu_memory_info() {
    if let Ok(_) = nvapi::initialize() {
        if let Ok(gpus) = nvapi::PhysicalGpu::enumerate() {
            for (i, gpu) in gpus.iter().enumerate() {
                println!("=== GPU {} Memory Info ===", i);
                
                if let Ok(memory) = gpu.memory_info() {
                    println!("Memory Info: {:?}", memory);
                }
            }
        }
    }
}

#[test]
fn physicalgpu_clock_frequencies() {
    if let Ok(_) = nvapi::initialize() {
        if let Ok(gpus) = nvapi::PhysicalGpu::enumerate() {
            for (i, gpu) in gpus.iter().enumerate() {
                println!("=== GPU {} Clock Frequencies ===", i);
                
                // Test different clock frequency types
                if let Ok(base_clocks) = gpu.clock_frequencies(nvapi::ClockFrequencyType::Base) {
                    println!("Base Clocks: {:?}", base_clocks);
                }
                
                if let Ok(boost_clocks) = gpu.clock_frequencies(nvapi::ClockFrequencyType::Boost) {
                    println!("Boost Clocks: {:?}", boost_clocks);
                }
                
                if let Ok(current_clocks) = gpu.clock_frequencies(nvapi::ClockFrequencyType::Current) {
                    println!("Current Clocks: {:?}", current_clocks);
                }
            }
        }
    }
}

#[test]
fn physicalgpu_performance_state() {
    if let Ok(_) = nvapi::initialize() {
        if let Ok(gpus) = nvapi::PhysicalGpu::enumerate() {
            for (i, gpu) in gpus.iter().enumerate() {
                println!("=== GPU {} Performance State ===", i);
                
                if let Ok(pstate) = gpu.current_pstate() {
                    println!("Current PState: {:?}", pstate);
                }
                
                if let Ok(pstates) = gpu.pstates() {
                    println!("PStates: {:?}", pstates);
                }
                
                if let Ok(utilizations) = gpu.dynamic_pstates_info() {
                    println!("Utilizations: {:?}", utilizations);
                }
                
                if let Ok(perf_decrease) = gpu.performance_decrease() {
                    println!("Performance Decrease: {:?}", perf_decrease);
                }
            }
        }
    }
}

#[test]
fn physicalgpu_thermal_info() {
    if let Ok(_) = nvapi::initialize() {
        if let Ok(gpus) = nvapi::PhysicalGpu::enumerate() {
            for (i, gpu) in gpus.iter().enumerate() {
                println!("=== GPU {} Thermal Info ===", i);
                
                if let Ok(tach) = gpu.tachometer() {
                    println!("Tachometer: {} RPM", tach);
                }
                
                if let Ok(thermal) = gpu.thermal_settings(None) {
                    println!("Thermal Settings: {:?}", thermal);
                }
                
                if let Ok(thermal_info) = gpu.thermal_limit_info() {
                    println!("Thermal Limit Info: {:?}", thermal_info);
                }
                
                if let Ok(thermal_limit) = gpu.thermal_limit() {
                    println!("Thermal Limit: {:?}", thermal_limit);
                }
            }
        }
    }
}

#[test]
fn physicalgpu_power_info() {
    if let Ok(_) = nvapi::initialize() {
        if let Ok(gpus) = nvapi::PhysicalGpu::enumerate() {
            for (i, gpu) in gpus.iter().enumerate() {
                println!("=== GPU {} Power Info ===", i);
                
                if let Ok(power_usage) = gpu.power_usage() {
                    println!("Power Usage: {:?}", power_usage);
                }
                
                if let Ok(power_info) = gpu.power_limit_info() {
                    println!("Power Limit Info: {:?}", power_info);
                }
                
                if let Ok(power_limit) = gpu.power_limit() {
                    println!("Power Limit: {:?}", power_limit);
                }
                
                if let Ok(perf_info) = gpu.perf_info() {
                    println!("Perf Info: {:?}", perf_info);
                }
                
                if let Ok(perf_status) = gpu.perf_status() {
                    println!("Perf Status: {:?}", perf_status);
                }
            }
        }
    }
}

#[test]
fn physicalgpu_voltage_info() {
    if let Ok(_) = nvapi::initialize() {
        if let Ok(gpus) = nvapi::PhysicalGpu::enumerate() {
            for (i, gpu) in gpus.iter().enumerate() {
                println!("=== GPU {} Voltage Info ===", i);
                
                if let Ok(core_voltage) = gpu.core_voltage() {
                    println!("Core Voltage: {:?}", core_voltage);
                }
                
                if let Ok(voltage_boost) = gpu.core_voltage_boost() {
                    println!("Voltage Boost: {:?}", voltage_boost);
                }
                
                if let Ok(voltage_domains) = gpu.voltage_domains_status() {
                    println!("Voltage Domains: {:?}", voltage_domains);
                }
                
                if let Ok(voltage_step) = gpu.voltage_step() {
                    println!("Voltage Step: {:?}", voltage_step);
                }
                
                if let Ok(voltage_table) = gpu.voltage_table() {
                    println!("Voltage Table: {:?}", voltage_table);
                }
            }
        }
    }
}

#[test]
fn physicalgpu_vfp_info() {
    if let Ok(_) = nvapi::initialize() {
        if let Ok(gpus) = nvapi::PhysicalGpu::enumerate() {
            for (i, gpu) in gpus.iter().enumerate() {
                println!("=== GPU {} VFP Info ===", i);
                
                if let Ok(vfp_mask) = gpu.vfp_mask() {
                    println!("VFP Mask: {:?}", vfp_mask);
                    
                    // Use the mask for vfp_table and vfp_curve tests
                    if let Ok(vfp_table) = gpu.vfp_table(vfp_mask.mask) {
                        println!("VFP Table: {:?}", vfp_table);
                    }
                    
                    if let Ok(vfp_curve) = gpu.vfp_curve(vfp_mask.mask) {
                        println!("VFP Curve: {:?}", vfp_curve);
                    }
                }
                
                if let Ok(vfp_ranges) = gpu.vfp_ranges() {
                    println!("VFP Ranges: {:?}", vfp_ranges);
                }
                
                if let Ok(vfp_locks) = gpu.vfp_locks() {
                    println!("VFP Locks: {:?}", vfp_locks);
                }
            }
        }
    }
}

#[test]
fn physicalgpu_cooler_info() {
    if let Ok(_) = nvapi::initialize() {
        if let Ok(gpus) = nvapi::PhysicalGpu::enumerate() {
            for (i, gpu) in gpus.iter().enumerate() {
                println!("=== GPU {} Cooler Info ===", i);
                
                if let Ok(cooler_settings) = gpu.cooler_settings(None) {
                    println!("Cooler Settings: {:?}", cooler_settings);
                    
                    // Test cooler policy table for each cooler
                    for (cooler_idx, cooler) in cooler_settings.iter().enumerate() {
                        if let Ok(policy_table) = gpu.cooler_policy_table(cooler_idx as u32, cooler.default_policy) {
                            println!("Cooler {} Policy Table: {:?}", cooler_idx, policy_table);
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn physicalgpu_display_ids_connected() {
    if let Ok(_) = nvapi::initialize() {
        if let Ok(gpus) = nvapi::PhysicalGpu::enumerate() {
            for (i, gpu) in gpus.iter().enumerate() {
                println!("=== GPU {} Connected Displays ===", i);
                // Bug: if there are zero connected displays this may crash.
                if let Ok(displays) = gpu.display_ids_connected(nvapi::ConnectedIdsFlags::empty()) {
                    println!("Connected displays: {:?}", displays);
                }
            }
        }
    }
}

#[test]
fn physicalgpu_display_ids_all() {
    if let Ok(_) = nvapi::initialize() {
        if let Ok(gpus) = nvapi::PhysicalGpu::enumerate() {
            for (i, gpu) in gpus.iter().enumerate() {
                println!("=== GPU {} All Displays ===", i);
                // Bug: if there are zero connected displays this may crash.
                if let Ok(displays) = gpu.display_ids_all() {
                    println!("All displays: {:?}", displays);
                }
            }
        }
    }
}

#[test]
fn gsync_topology() {
    let Ok(_) = nvapi::initialize() else {
        panic!("nvapi::initialize failed");
    };
    
    let Ok(gsync_device) = nvapi::GSyncDevice::enum_sync_devices() else {
        panic!("GSyncDevice::enum_sync_devices failed");
    }; 

    for gsync in gsync_device {
        let Ok(topology) = gsync.get_topology() else {
            println!("get_topology error");
            continue;
        };
        println!(
            "Connected GPUs: {} ### Active Displays: {}",
            topology.0.len(),
            topology.1.len()
        );
        println!("topology: {:#?}", topology);
    }
}
