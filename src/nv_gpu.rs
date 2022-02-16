use crate::utils::exec;
use quick_xml::de::from_str;
use serde::{Deserialize, Deserializer};
use std::fmt;
use std::ops;

// TODO: Result<GPUInfo, Err>
pub fn get_nvidia_gpu_info() -> GPUInfo {
    let out = exec("nvidia-smi", Some(vec!["-q", "-x"])).unwrap();
    from_str(&out).unwrap()
}

#[derive(Debug, PartialEq)]
pub enum ValidValue {
    INT(isize),
    FLOAT(f64),
}

impl ops::Div for &ValidValue {
    type Output = f64;
    fn div(self, rhs: Self) -> f64 {
        let lhs: f64 = match self {
            ValidValue::INT(v) => *v as f64,
            ValidValue::FLOAT(v) => *v,
        };
        let rhs: f64 = match rhs {
            ValidValue::INT(v) => *v as f64,
            ValidValue::FLOAT(v) => *v,
        };
        lhs / rhs
    }
}

impl fmt::Display for ValidValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidValue::INT(v) => write!(f, "{}", v),
            ValidValue::FLOAT(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Value {
    pub val: ValidValue,
    pub unit: Option<String>,
}

impl Value {
    pub fn val_as_isize(&self) -> isize {
        match self.val {
            ValidValue::FLOAT(v) => v.round() as isize,
            ValidValue::INT(v) => v,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn _get_unit(s: &Option<String>) -> String {
            match s {
                Some(u) => u.to_string(),
                None => String::from(""),
            }
        }
        fn _display<T: fmt::Display>(
            _f: &mut fmt::Formatter<'_>,
            val: T,
            unit: String,
        ) -> fmt::Result {
            write!(_f, "{}{}", val, unit)
        }
        match &self.val {
            ValidValue::INT(v) if v == &-1 => {
                write!(f, "N/A")
            }
            ValidValue::INT(v) => _display(f, v, _get_unit(&self.unit)),
            ValidValue::FLOAT(v) => _display(f, v, _get_unit(&self.unit)),
        }
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let str_v: Vec<&str> = s.split(" ").collect();
        Ok(match str_v.len() {
            2 => {
                let val_str = str_v.get(0).unwrap();
                let val = {
                    if val_str.contains(".") {
                        ValidValue::FLOAT(val_str.parse().unwrap())
                    } else {
                        ValidValue::INT(val_str.parse().unwrap())
                    }
                };
                Value {
                    val,
                    unit: match str_v.get(1) {
                        Some(&u) => Some(u.to_string()),
                        _ => None,
                    },
                }
            }
            _ => Value {
                val: ValidValue::INT(-1),
                unit: None,
            },
        })
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ProcessInfo {
    pub gpu_instance_id: String,     // N/A
    pub compute_instance_id: String, // N/A
    pub pid: u32,
    #[serde(rename = "type")]
    pub p_type: String,
    #[serde(rename = "process_name")]
    pub name: String,
    pub used_memory: Value, // MiB
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Processes {
    #[serde(rename = "process_info")]
    pub items: Option<Vec<ProcessInfo>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Clock {
    #[serde(rename = "graphics_clock")]
    // pub graphics: String, // MHz
    pub graphics: Value,
    #[serde(rename = "sm_clock")]
    pub sm: Option<Value>, // MHz
    #[serde(rename = "mem_clock")]
    pub mem: Option<Value>, // MHz
    #[serde(rename = "video_clock")]
    pub video: Option<Value>, // MHz
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct PowerInfo {
    #[serde(rename = "power_state")]
    pub state: String, // P2
    #[serde(rename = "power_management")]
    pub management: String, // Supported
    #[serde(rename = "power_draw")]
    pub draw: Value, // W
    #[serde(rename = "power_limit")]
    pub limit: Value, // W
    #[serde(rename = "default_power_limit")]
    pub default_limit: Value, // W
    #[serde(rename = "enforced_power_limit")]
    pub enforced_limit: Value, // W
    #[serde(rename = "min_power_limit")]
    pub min_limit: Value, // W
    #[serde(rename = "max_power_limit")]
    pub max_limit: Value, // W
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Temperature {
    #[serde(rename = "gpu_temp")]
    pub value: Value, // C
    #[serde(rename = "gpu_temp_max_threshold")]
    pub max_threshold: Value, // C
    #[serde(rename = "gpu_temp_slow_threshold")]
    pub slow_threshold: Value, // C
    #[serde(rename = "gpu_temp_max_gpu_threshold")]
    pub max_gpu_threshold: Value, // C
    #[serde(rename = "gpu_target_temperature")]
    pub target_temperature: Value, // C
    pub memory_temp: Value, // C | N/A
    #[serde(rename = "gpu_temp_max_mem_threshold")]
    pub max_mem_threshold: Value, // C | N/A
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Utilization {
    #[serde(rename = "gpu_util")]
    pub gpu: Value, // %
    #[serde(rename = "memory_util")]
    pub memory: Value, // %
    #[serde(rename = "encoder_util")]
    pub encoder: Value, // %
    #[serde(rename = "decoder_util")]
    pub decoder: Value, // %
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct MemUsage {
    pub total: Value, // MiB
    pub used: Value,  // MiB
    pub free: Value,  // MiB
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct PCI {
    pub pci_bus: String,
    pub pci_device: String,
    pub pci_domain: String,
    pub pci_device_id: String,
    pub pci_bus_id: String,
    pub pci_sub_system_id: String,
    // pci_gpu_link_info
    // pci_bridge_chip
    pub replay_counter: u8,
    pub replay_rollover_counter: u8,
    pub tx_util: String,
    pub rx_util: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct GPU {
    pub id: String,
    pub product_name: String,
    // product_brand: String,
    // product_architecture: String,
    // display_mode: Mode,
    // display_active: Mode,
    // persistence_mode: Mode,
    // mid_mode
    // mig_devices
    // accounting_mode: Mode,
    // accounting_mode_buffer_size: u32,
    // driver_model
    // serial: Option<String>,
    pub uuid: String,
    // minor_number: u8,
    // vbios_version: String,
    // multigpu_board: String, // Yes | No
    // board_id: String,
    // gpu_part_number: Option<String>,
    // gpu_module_id: u8,
    // inforom_version
    // gpu_operation_mode
    // gsp_firmware_version
    // gpu_virtualization_mode
    // ibmnpu
    // pci: PCI,
    pub fan_speed: String,
    // performance_state: String, // P2
    // clocks_throttle_reasons
    pub fb_memory_usage: MemUsage,
    pub bar1_memory_usage: MemUsage,
    pub compute_mode: String, // Default
    pub utilization: Utilization,
    // encoder_stats
    // fbc_stats
    // ecc_mode
    // ecc_errors
    // retired_pages
    // remapped_rows
    pub temperature: Temperature,
    // supported_gpu_target_temp
    pub power_readings: PowerInfo,
    pub clocks: Clock,
    // applications_clocks: Clock,
    // default_applications_clocks: Clock,
    // max_clocks: Clock,
    // max_customer_boost_clocks: Clock
    // clock_policy
    // voltage
    // supported_clocks
    pub processes: Processes,
    // accounted_processes
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct GPUInfo {
    pub timestamp: String,
    pub driver_version: String,
    pub cuda_version: String,
    pub attached_gpus: u8,
    #[serde(rename = "gpu")]
    pub gpus: Vec<GPU>,
}
