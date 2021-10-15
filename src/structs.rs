use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub oxirc_api_version: String,
    pub boot_stages: Vec<String>,
    pub die_on_failure: bool,
}

#[derive(Deserialize)]
pub struct Unit {
    pub name: String,
    pub unit_type: String,
    pub runas: String,
    pub requires: Vec<String>,
    pub command: String,
    pub args: Vec<String>,
}

pub enum UnitStatuses {
    Unknown,
    Ready,
    Running,
    Finished,
    Crashed,
    ConfigError
}

pub struct UnitContainer {
    pub unit: Unit,
    pub last_modifed: std::time::SystemTime,
    pub status: UnitStatuses,
}