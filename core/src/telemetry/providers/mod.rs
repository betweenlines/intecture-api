// Copyright 2015-2017 Intecture Developers.
//
// Licensed under the Mozilla Public License 2.0 <LICENSE or
// https://www.tldrlegal.com/l/mpl-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

mod centos;
mod debian;
mod freebsd;
mod macos;

pub use self::centos::{Centos, RemoteProvider as CentosRemoteProvider};
pub use self::debian::{Debian, RemoteProvider as DebianRemoteProvider};
pub use self::freebsd::{Freebsd, RemoteProvider as FreebsdRemoteProvider};
pub use self::macos::{Macos, RemoteProvider as MacosRemoteProvider};
