use crate::errors::Errors;
use phf::phf_map;

/// syntactic sugar
macro_rules! reference {
    ($value:expr) => {
        $value
    };
}

static PRESETS: phf::Map<&'static str, &'static str> = phf_map! {
    "ssh" => "T22",
    "ftp" => "T21",
    "dns" => "T53",
    "postgresql" => "T5432",
    "vite-dev" => "T5173",
    "vite-preview" => "T4173",
    "vite-build" => reference!("vite-preview"),
    "mc-java" => "T25565",
    "mc-bedrock" => "U19132",
    "rdp" => "T3389",
    "mysql" => "T3306",
    "ms-sql" => "T1433",
    "mongodb" => "T27017"
};

pub fn preset(value: &str) -> Result<&str, Errors> {
    PRESETS
        .get(value)
        .copied()
        .ok_or_else(|| Errors::InvalidPreset(value.into()))
}
