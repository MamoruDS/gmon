use super::error::BackendError;
use super::types::{MemoryInfo, PowerInfo, Value};

pub trait GpuInfo<'a> {
    fn index(&self) -> Result<Value<u32>, BackendError>;
    fn name(&self) -> Result<Value<String>, BackendError>;
    fn utilization(&self) -> Result<Value<u32>, BackendError>;
    fn temperature(&self) -> Result<Value<u32>, BackendError>;
    fn power_info(&self) -> Result<PowerInfo, BackendError>;
    fn memory_info(&self) -> Result<MemoryInfo, BackendError>;
}
