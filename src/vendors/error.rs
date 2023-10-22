use thiserror::Error;

#[derive(Error, Debug)]
pub enum GMONError {
    #[error("failed to initialize nvml with nvml_wrapper")]
    NvmlInitError,
}
