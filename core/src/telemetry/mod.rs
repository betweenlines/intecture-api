// Copyright 2015-2017 Intecture Developers.
//
// Licensed under the Mozilla Public License 2.0 <LICENSE or
// https://www.tldrlegal.com/l/mpl-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

//! System generated data about your host.
//!
//! Telemetry is retrieved automatically when you create a new `Host`, which is
//! nice of it. Call [`Host.telemetry()`](../host/trait.Host.html#tymethod.telemetry)
//! to access it.

mod providers;
#[doc(hidden)] pub mod serializable;

use errors::*;
use futures::Future;
use host::Host;
use pnet::datalink::NetworkInterface;
use remote::{Request, Response};
#[doc(hidden)] pub use self::providers::factory;
use std::path::PathBuf;

/// Top level structure that contains static information about a `Host`.
#[derive(Debug)]
pub struct Telemetry {
    /// Information on the CPU
    pub cpu: Cpu,
    /// Information on the filesystem
    pub fs: Vec<FsMount>,
    /// Host's FQDN
    pub hostname: String,
    /// Amount of RAM, in bytes
    pub memory: u64,
    /// Information on network interfaces
    pub net: Vec<NetworkInterface>,
    /// Information about the operating system
    pub os: Os,
    /// Information on the current user
    pub user: User,
}

/// Information about the `Host`s CPU.
#[derive(Debug, Serialize, Deserialize)]
pub struct Cpu {
    /// Processor vendor, e.g. "GenuineIntel"
    pub vendor: String,
    /// Full description of the processor
    pub brand_string: String,
    /// Number of cores in the processor
    pub cores: u32,
}

/// Information about a specific filesystem mount.
#[derive(Debug, Serialize, Deserialize)]
pub struct FsMount {
    /// The device path, e.g. /dev/sd0s1
    pub filesystem: String,
    /// Path to where the device is mounted, e.g. /boot
    pub mountpoint: String,
    /// Capacity of device in Kb
    pub size: u64,
    /// Amount used in Kb
    pub used: u64,
    /// Remaining capacity available in Kb
    pub available: u64,
    /// Percentage used as a decimal
    pub capacity: f32,
}

/// Information about the `Host`s OS.
#[derive(Debug, Serialize, Deserialize)]
pub struct Os {
    /// OS architecture, e.g. "x86_64"
    pub arch: String,
    /// OS family
    pub family: OsFamily,
    /// OS name
    pub platform: OsPlatform,
    /// Full version string, e.g. "10.13"
    pub version_str: String,
    /// Major version number, e.g. "10"
    pub version_maj: u32,
    /// Minor version number, e.g. "13"
    pub version_min: u32,
    /// Patch version number, e.g. "0"
    pub version_patch: u32,
}

/// Operating system family
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum OsFamily {
    Bsd,
    Darwin,
    Linux(LinuxDistro),
}

/// Operating system name
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum OsPlatform {
    Centos,
    Debian,
    Fedora,
    Freebsd,
    Macos,
    Nixos,
    Ubuntu,
}

/// Linux distribution name
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum LinuxDistro {
    Debian,
    RHEL,
    Standalone,
}

/// Information on the current user
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user: String,
    pub uid: u32,
    pub group: String,
    pub gid: u32,
    pub home_dir: PathBuf,
}

impl Telemetry {
    pub fn load<H: Host>(host: &H) -> Box<Future<Item = Telemetry, Error = Error>> {
        Box::new(host.request(Request::TelemetryLoad)
            .chain_err(|| ErrorKind::Request { endpoint: "Telemetry", func: "load" })
            .map(|msg| match msg.into_inner() {
                Response::TelemetryLoad(t) => Telemetry::from(t),
                _ => unreachable!(),
            }))
    }
}

impl User {
    // Whether this user is root, which is calculated as `uid == 0`.
    pub fn is_root(&self) -> bool {
        self.uid == 0
    }
}
