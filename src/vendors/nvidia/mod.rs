use nvml_wrapper::{self, enum_wrappers::device::TemperatureSensor, Nvml};

use super::error::BackendError;
use super::traits::{GpuInfo, GpuProviderInfo};
use super::types::{MemoryInfo, PowerInfo, Value};

pub mod nvml_utils;

pub struct NvGpuIter<'a> {
    index: u32,
    nvml: &'a Nvml,
}

impl<'a> Iterator for NvGpuIter<'a> {
    type Item = NvGpu<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.nvml.device_count().unwrap() {
            let gpu = NvGpu::get_by_id(&self.nvml, self.index);
            self.index += 1;
            Some(gpu)
        } else {
            None
        }
    }
}

pub fn gpu_iter<'a>(nvml: &'a Nvml) -> NvGpuIter<'a> {
    NvGpuIter { index: 0, nvml }
}

pub struct CudaVersion {
    pub major: Value<i32>,
    pub minor: Value<i32>,
}

pub struct NvGpuProvider {
    nvml: Nvml,
}

impl NvGpuProvider {
    pub fn new() -> Result<Self, BackendError> {
        Ok(Self {
            nvml: nvml_utils::nvml_initiate(None)?,
        })
    }

    // pub fn new_with_lib(lib_candidates: &[String]) -> Result<Self, BackendError> {
    //     Ok(Self {
    //         nvml: nvml_utils::nvml_initiate(Some(lib_candidates))?,
    //     })
    // }

    pub fn gpu_iter(&self) -> NvGpuIter {
        gpu_iter(&self.nvml)
    }

    pub fn cuda_version(&self) -> Result<CudaVersion, BackendError> {
        let version = self
            .nvml
            .sys_cuda_driver_version()
            .map_err(BackendError::NvmlWrapperError)?;
        Ok(CudaVersion {
            major: Value::from(nvml_wrapper::cuda_driver_version_major(version)),
            minor: Value::from(nvml_wrapper::cuda_driver_version_minor(version)),
        })
    }
}

impl<'a> GpuProviderInfo<'a> for NvGpuProvider {
    fn driver_version(&self) -> Result<Value<String>, BackendError> {
        let version = self
            .nvml
            .sys_driver_version()
            .map_err(BackendError::NvmlWrapperError)?;
        Ok(Value::from(version))
    }

    fn device_count(&self) -> Result<Value<u32>, BackendError> {
        let count = self
            .nvml
            .device_count()
            .map_err(BackendError::NvmlWrapperError)?;
        Ok(Value::from(count))
    }
}

pub struct NvGpu<'a> {
    pub gpu: nvml_wrapper::Device<'a>,
}

impl<'a> NvGpu<'a> {
    pub fn get_by_id(nvml: &'a Nvml, id: u32) -> Self {
        let gpu = nvml.device_by_index(id).unwrap();
        assert!(id == gpu.index().unwrap()); // TODO:
        Self { gpu }
    }
}

impl<'a> GpuInfo<'a> for NvGpu<'a> {
    fn index(&self) -> Result<Value<u32>, BackendError> {
        let idx = self.gpu.index().map_err(BackendError::NvmlWrapperError)?;
        Ok(Value::from(idx))
    }

    fn name(&self) -> Result<Value<String>, BackendError> {
        let name = self.gpu.name().map_err(BackendError::NvmlWrapperError)?;
        Ok(Value::from(name))
    }

    fn power_info(&self) -> Result<PowerInfo, BackendError> {
        let read = self
            .gpu
            .power_usage()
            .map_err(BackendError::NvmlWrapperError)?;
        let limit = self
            .gpu
            .power_management_limit()
            .map_err(BackendError::NvmlWrapperError)?;
        let limit_constraints = self
            .gpu
            .power_management_limit_constraints()
            .map_err(BackendError::NvmlWrapperError)?;
        Ok(PowerInfo {
            read: Value::from(read).set_unit("mW"),
            limit: Value::from(limit).set_unit("mW"),
            limit_default: Value::from(limit_constraints.max_limit).set_unit("mW"),
        })
    }

    fn utilization(&self) -> Result<Value<u32>, BackendError> {
        let util = self
            .gpu
            .utilization_rates()
            .map_err(BackendError::NvmlWrapperError)?;
        Ok(Value::from(util.gpu))
    }

    fn temperature(&self) -> Result<Value<u32>, BackendError> {
        let temp = self
            .gpu
            .temperature(TemperatureSensor::Gpu)
            .map_err(BackendError::NvmlWrapperError)?;
        Ok(Value::from(temp).set_unit("°C"))
    }

    fn memory_info(&self) -> Result<MemoryInfo, BackendError> {
        let mem_info = self
            .gpu
            .memory_info()
            .map_err(BackendError::NvmlWrapperError)?;
        Ok(MemoryInfo {
            total: Value::from(mem_info.total).set_unit("MB"),
            used: Value::from(mem_info.used).set_unit("MB"),
        })
    }
}
