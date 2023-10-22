#[derive(Debug)]
pub struct Value<T: ToString> {
    pub val: T,
    pub unit: Option<String>,
}

impl<T: ToString> Value<T> {
    pub fn from(val: T) -> Self {
        Self { val, unit: None }
    }
    pub fn set_unit(mut self, unit: &str) -> Self {
        self.unit = Some(unit.to_string());
        self
    }
}

#[derive(Debug)]
pub struct MemoryInfo {
    pub total: Value<u64>,
    pub used: Value<u64>,
}

#[derive(Debug)]
pub struct PowerInfo {
    pub read: Value<u32>,
    pub limit: Value<u32>,
    pub limit_default: Value<u32>,
}
