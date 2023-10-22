use std::ffi::OsStr;

use nvml_wrapper::error::NvmlError;
use nvml_wrapper::Nvml;

use super::error::GMONError;

enum LibCandidate<'a> {
    Default,
    Custom(&'a str),
}

static NVML_LIB_CAN: [LibCandidate; 2] = [
    LibCandidate::Default,
    LibCandidate::Custom("libnvidia-ml.so.1"),
];

fn init_nvml(lib: &LibCandidate) -> Result<Nvml, NvmlError> {
    match lib {
        LibCandidate::Default => Nvml::init(),
        LibCandidate::Custom(path) => Nvml::builder().lib_path(OsStr::new(&path)).init(),
    }
}

pub fn nvml_initiate(custom_candidates: Option<&Vec<String>>) -> Result<Nvml, GMONError> {
    match {
        for can in custom_candidates
            .unwrap_or(&vec![])
            .iter()
            .map(|s| LibCandidate::Custom(s))
            .collect::<Vec<LibCandidate>>()
            .iter()
            .chain(NVML_LIB_CAN.iter())
        {
            match init_nvml(can) {
                Ok(nvml) => {
                    Some(nvml);
                    break;
                }
                Err(_) => continue,
            }
        }
        None
    } {
        Some(nvml) => Ok(nvml),
        None => Err(GMONError::NvmlInitError),
    }
}
