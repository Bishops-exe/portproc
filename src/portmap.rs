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
    "mongodb" => "T27017",
    "http" => "T80",
    "https" => "T443",
    "redis" => "T6379",
    "smtp" => "T25",
    "smtps" => "T465",
    "imap" => "T143",
    "imaps" => "T993",
    "pop3" => "T110",
    "pop3s" => "T995",
    "telnet" => "T23",
    "memcached" => "T11211",
    "elasticsearch" => "T9200",
    "rabbitmq" => "T5672",
    "k8s-api" => "T6443",
    "next-dev" => "T3000",
    "cra-dev" => reference!("next-dev"),
    "create-react-app-dev" => reference!("cra-dev")
};

pub fn preset(value: &str) -> Result<&str, Errors> {
    PRESETS
        .get(value)
        .copied()
        .ok_or_else(|| Errors::InvalidPreset(value.into()))
}
