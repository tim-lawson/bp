use core::fmt;
use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GpuType {
    RTX2080,
    RTX3090,
    A100,
}

pub const GPU_TYPES: [GpuType; 3] = [GpuType::RTX2080, GpuType::RTX3090, GpuType::A100];

impl fmt::Display for GpuType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GpuType::RTX2080 => write!(f, "RTX2080"),
            GpuType::RTX3090 => write!(f, "RTX3090"),
            GpuType::A100 => write!(f, "A100"),
        }
    }
}

impl GpuType {
    fn ord(&self) -> u8 {
        match *self {
            GpuType::RTX2080 => 3,
            GpuType::RTX3090 => 2,
            GpuType::A100 => 1,
        }
    }
    pub fn name(&self) -> &str {
        match *self {
            GpuType::RTX2080 => "rtx_2080",
            GpuType::RTX3090 => "rtx_3090",
            GpuType::A100 => "A100",
        }
    }
    pub fn system_memory_per_gpu(&self) -> u8 {
        match *self {
            GpuType::RTX2080 => 22,
            GpuType::RTX3090 => 62,
            GpuType::A100 => 124,
        }
    }
    pub fn cpus_per_gpu(&self) -> u8 {
        match *self {
            GpuType::RTX2080 => 2,
            GpuType::RTX3090 => 2,
            GpuType::A100 => 16,
        }
    }
}

impl Ord for GpuType {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.ord()).cmp(&(other.ord()))
    }
}

impl PartialOrd for GpuType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for GpuType {
    fn eq(&self, other: &Self) -> bool {
        self.ord() == other.ord()
    }
}

impl Eq for GpuType {}
