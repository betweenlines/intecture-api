// Copyright 2015-2017 Intecture Developers.
//
// Licensed under the Mozilla Public License 2.0 <LICENSE or
// https://www.tldrlegal.com/l/mpl-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

use command::providers::factory;
use error_chain::ChainedError;
use errors::*;
use futures::{future, Future};
use provider::Provider;
use remote::{ExecutableResult, ProviderName, Response, ResponseResult};
use std::process;
use super::PackageProvider;
use telemetry::Os;
use tokio_core::reactor::Handle;
use tokio_process::CommandExt;
use tokio_proto::streaming::Message;

/// The Pkg `Package` provider.
pub struct Pkg;

impl Provider for Pkg {
    fn available() -> bool {
        process::Command::new("/usr/bin/type")
            .arg("pkg")
            .status()
            .unwrap()
            .success()
    }

    fn name(&self) -> ProviderName {
        ProviderName::PackagePkg
    }
}

impl PackageProvider for Pkg {
    #[doc(hidden)]
    fn installed(&self, handle: &Handle, name: &str, _: &Os) -> ExecutableResult {
        let handle = handle.clone();
        let name = name.to_owned();

        Box::new(process::Command::new("pkg")
            .args(&["query", "\"%n\"", &name])
            .output_async(&handle)
            .chain_err(|| "Could not get installed packages")
            .and_then(move |output| {
                future::ok(
                    Message::WithoutBody(
                        ResponseResult::Ok(
                            Response::Bool(
                                output.status.success()))))
            }))
    }

    #[doc(hidden)]
    fn install(&self, handle: &Handle, name: &str) -> ExecutableResult {
        let cmd = match factory() {
            Ok(c) => c,
            Err(e) => return Box::new(future::ok(
                Message::WithoutBody(
                    ResponseResult::Err(
                        format!("{}", e.display_chain()))))),
        };
        cmd.exec(handle, name, &["pkg".into(), "install".into(), "-y".into()])
    }

    #[doc(hidden)]
    fn uninstall(&self, handle: &Handle, name: &str) -> ExecutableResult {
        let cmd = match factory() {
            Ok(c) => c,
            Err(e) => return Box::new(future::ok(
                Message::WithoutBody(
                    ResponseResult::Err(
                        format!("{}", e.display_chain()))))),
        };
        cmd.exec(handle, name, &["pkg".into(), "delete".into(), "-y".into()])
    }
}