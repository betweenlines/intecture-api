// Copyright 2015-2017 Intecture Developers.
//
// Licensed under the Mozilla Public License 2.0 <LICENSE or
// https://www.tldrlegal.com/l/mpl-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

use command::factory;
use error_chain::ChainedError;
use errors::*;
use futures::{future, Future};
use remote::{ExecutableResult, Response, ResponseResult};
use std::process;
use super::ServiceProvider;
use telemetry::{LinuxDistro, OsFamily, Telemetry};
use tokio_core::reactor::Handle;
use tokio_process::CommandExt;
use tokio_proto::streaming::Message;

pub struct Redhat;

impl ServiceProvider for Redhat {
    fn available(telemetry: &Telemetry) -> Result<bool> {
        Ok(telemetry.os.family == OsFamily::Linux(LinuxDistro::RHEL))
    }

    fn running(&self, handle: &Handle, name: &str) -> ExecutableResult {
        let status = match process::Command::new("service")
            .args(&[name, "status"])
            .status_async2(handle)
            .chain_err(|| "Error checking if service is running")
        {
            Ok(s) => s,
            Err(e) => return Box::new(future::err(e)),
        };
        Box::new(status.map(|s| Message::WithoutBody(
                ResponseResult::Ok(
                    Response::Bool(s.success()))))
            .map_err(|e| Error::with_chain(e, ErrorKind::SystemCommand("service <service> status"))))
    }

    fn action(&self, handle: &Handle, name: &str, action: &str) -> ExecutableResult {
        let cmd = match factory() {
            Ok(c) => c,
            Err(e) => return Box::new(future::ok(
                Message::WithoutBody(
                    ResponseResult::Err(
                        format!("{}", e.display_chain()))))),
        };
        cmd.exec(handle, &["service", action, name])
    }

    fn enabled(&self, handle: &Handle, name: &str) -> ExecutableResult {
        let status = match process::Command::new("/usr/sbin/chkconfig")
            .arg(name)
            .status_async2(handle)
            .chain_err(|| "Error checking if service is enabled")
        {
            Ok(s) => s,
            Err(e) => return Box::new(future::err(e)),
        };
        Box::new(status.map(|s| Message::WithoutBody(
                ResponseResult::Ok(
                    Response::Bool(s.success()))))
            .map_err(|e| Error::with_chain(e, ErrorKind::SystemCommand("chkconfig <service>"))))
    }

    fn enable(&self, handle: &Handle, name: &str) -> ExecutableResult {
        Box::new(process::Command::new("/usr/sbin/chkconfig")
            .args(&[name, "on"])
            .output_async(handle)
            .map(|out| {
                if out.status.success() {
                    Message::WithoutBody(ResponseResult::Ok(Response::Null))
                } else {
                    Message::WithoutBody(ResponseResult::Err(
                        format!("Could not enable service: {}", String::from_utf8_lossy(&out.stderr))))
                }
            })
            .map_err(|e| Error::with_chain(e, ErrorKind::SystemCommand("chkconfig <service> on"))))
    }

    fn disable(&self, handle: &Handle, name: &str) -> ExecutableResult {
        Box::new(process::Command::new("/usr/sbin/chkconfig")
            .args(&[name, "off"])
            .output_async(handle)
            .map(|out| {
                if out.status.success() {
                    Message::WithoutBody(ResponseResult::Ok(Response::Null))
                } else {
                    Message::WithoutBody(ResponseResult::Err(
                        format!("Could not disable service: {}", String::from_utf8_lossy(&out.stderr))))
                }
            })
            .map_err(|e| Error::with_chain(e, ErrorKind::SystemCommand("chkconfig <service> off"))))
    }
}
