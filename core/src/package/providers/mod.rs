// Copyright 2015-2017 Intecture Developers.
//
// Licensed under the Mozilla Public License 2.0 <LICENSE or
// https://www.tldrlegal.com/l/mpl-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

//! OS abstractions for `Package`.

mod apt;
mod dnf;
mod homebrew;
mod nix;
mod pkg;
mod yum;

use errors::*;
use remote::ExecutableResult;
pub use self::apt::Apt;
pub use self::dnf::Dnf;
pub use self::homebrew::Homebrew;
pub use self::nix::Nix;
pub use self::pkg::Pkg;
pub use self::yum::Yum;
use telemetry::Os;
use tokio_core::reactor::Handle;

/// Specific implementation of `Package`
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Provider {
    Apt,
    Dnf,
    Homebrew,
    Nix,
    Pkg,
    Yum,
}

pub trait PackageProvider {
    fn available() -> Result<bool> where Self: Sized;
    fn installed(&self, &Handle, &str, &Os) -> ExecutableResult;
    fn install(&self, &Handle, &str) -> ExecutableResult;
    fn uninstall(&self, &Handle, &str) -> ExecutableResult;
}

#[doc(hidden)]
pub fn factory() -> Result<Box<PackageProvider>> {
    if Apt::available()? {
        Ok(Box::new(Apt))
    }
    else if Dnf::available()? {
        Ok(Box::new(Dnf))
    }
    else if Homebrew::available()? {
        Ok(Box::new(Homebrew))
    }
    else if Nix::available()? {
        Ok(Box::new(Nix))
    }
    else if Pkg::available()? {
        Ok(Box::new(Pkg))
    }
    else if Yum::available()? {
        Ok(Box::new(Yum))
    } else {
        Err(ErrorKind::ProviderUnavailable("Package").into())
    }
}
