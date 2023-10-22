use super::error::BackendError;
use super::types::{MemoryInfo, PowerInfo, Value};

pub trait GpuProviderInfo<'a, T>
where
    T: GpuInfo<'a>,
{
    type IterType: Iterator<Item = T>;

    fn gpu_iter(&'a self) -> Self::IterType;

    fn driver_version(&self) -> Result<Value<String>, BackendError>;
    fn device_count(&self) -> Result<Value<u32>, BackendError>;
}

pub trait GpuInfo<'a> {
    fn index(&self) -> Result<Value<u32>, BackendError>;
    fn name(&self) -> Result<Value<String>, BackendError>;
    fn utilization(&self) -> Result<Value<u32>, BackendError>;
    fn temperature(&self) -> Result<Value<u32>, BackendError>;
    fn power_info(&self) -> Result<PowerInfo, BackendError>;
    fn memory_info(&self) -> Result<MemoryInfo, BackendError>;
}
