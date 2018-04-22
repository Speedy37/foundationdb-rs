// Copyright 2018 foundationdb-rs developers, https://github.com/bluejekyll/foundationdb-rs/graphs/contributors
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use error::{self, Result};
use foundationdb_sys as fdb_sys;
use network::NetworkBuilder;

pub fn get_max_api_version() -> i32 {
    unsafe { fdb_sys::fdb_get_max_api_version() as i32 }
}

pub struct FdbApi(private::PrivateFdbApi);

// forces the FdnApi construction to be private to this module
mod private {
    pub(super) struct PrivateFdbApi;
}

impl FdbApi {
    pub fn network(self) -> NetworkBuilder {
        NetworkBuilder::new(self)
    }
}

pub struct FdbApiBuilder {
    runtime_version: i32,
    header_version: i32,
}

impl FdbApiBuilder {
    pub fn set_runtime_version(mut self, version: i32) -> Self {
        self.runtime_version = version;
        self
    }

    pub fn set_header_version(mut self, version: i32) -> Self {
        self.header_version = version;
        self
    }

    /// The API version can only be initialized once in the lifetime of a process
    pub fn build(self) -> Result<FdbApi> {
        unsafe {
            error::eval(fdb_sys::fdb_select_api_version_impl(
                self.runtime_version,
                self.header_version,
            ))?;
        }

        Ok(FdbApi(private::PrivateFdbApi))
    }
}

impl Default for FdbApiBuilder {
    fn default() -> Self {
        FdbApiBuilder {
            runtime_version: fdb_sys::FDB_API_VERSION as i32,
            header_version: fdb_sys::FDB_API_VERSION as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_api() {
        assert!(get_max_api_version() > 0);
    }
}