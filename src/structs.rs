use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub oxirc_api_version: String,
    pub boot_stages: Vec<String>,
    pub die_on_failure: bool,
}

#[derive(Deserialize)]
pub struct UnitStage {
    pub name: String,
    pub sub: String,
}

#[derive(Deserialize)]
pub struct Unit {
    pub name: String,
    pub unit_type: String,
    pub runas: String,
    pub stage: UnitStage,
}

pub struct UnitContainer {
    pub unit: Unit,
    pub last_modifed: std::time::SystemTime,
}