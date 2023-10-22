use thiserror::Error;

#[derive(Error, Debug)]
pub enum BackendError {
    #[error("failed to initialize nvml with nvml_wrapper")]
    NvmlInitError,
}
