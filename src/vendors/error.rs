use nvml_wrapper::error::NvmlError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BackendError {
    #[error("failed to initialize nvml with nvml_wrapper")]
    NvmlInitError,
    #[error("failed due to nvml_wrapper error {0}")]
    NvmlWrapperError(#[from] NvmlError),
}
