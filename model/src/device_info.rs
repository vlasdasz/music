use std::env;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub os:   String,
    pub arch: String,
}

impl Default for DeviceInfo {
    fn default() -> DeviceInfo {
        let os = env::consts::OS.to_owned();
        let arch = env::consts::ARCH.to_owned();
        Self { os, arch }
    }
}
