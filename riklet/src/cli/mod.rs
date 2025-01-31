pub mod config;
pub mod function_config;

use clap::{value_parser, Parser};
use std::{net::Ipv4Addr, path::PathBuf};

/// The configuration of the riklet.
#[derive(Debug, Clone, Parser)]
#[command(name = "Riklet", version, about)]
pub struct CliConfiguration {
    /// The path to the Riklet configuration file. If the file not exists, it will be created.
    #[arg(short, long, default_value = "/etc/riklet/configuration.toml")]
    pub config_file: String,
    /// The IP of the Rik master node.
    #[arg(short, long)]
    pub master_ip: Option<String>,
    /// The level of verbosity.
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
    /// If set and there is a config file, values defined by the CLI will override values of the configuration file.
    #[arg(long)]
    pub override_config: bool,
    /// Path to the linux kernel.
    #[arg(
        long,
        value_name = "KERNEL_LOCATION",
        env = "KERNEL_LOCATION",
        default_value = "vmlinux.bin"
    )]
    pub kernel_path: PathBuf,
    /// Override the default network interface detected by the software
    #[arg(long, value_name = "IFACE", env = "IFACE", default_value = "eth0")]
    pub iface: Option<String>,
    /// Override the default IP gateway used by the software, it should match the IP given to ifnet
    #[arg(
        long,
        value_name = "IFACE_IP",
        env = "IFACE_IP",
        value_parser = value_parser!(Ipv4Addr)
    )]
    pub iface_ip: Option<Ipv4Addr>,
}
