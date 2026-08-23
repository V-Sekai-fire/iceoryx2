// Copyright (c) 2023 Contributors to the Eclipse Foundation
//
// See the NOTICE file(s) distributed with this work for additional
// information regarding copyright ownership.
//
// This program and the accompanying materials are made available under the
// terms of the Apache Software License 2.0 which is available at
// https://www.apache.org/licenses/LICENSE-2.0, or the MIT license
// which is available at https://opensource.org/licenses/MIT.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![no_std]

#[cfg(configuration_override)]
pub(crate) mod internal {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]
    #![allow(improper_ctypes)]
    #![allow(unknown_lints)]
    #![allow(unnecessary_transmutes)]
    #![allow(clippy::all)]
    include!(concat!(env!("IOX2_CUSTOM_PLATFORM_CONFIGURATION_PATH")));
}

#[cfg(not(configuration_override))]
pub(crate) mod internal {
    #[cfg(all(not(target_os = "windows"), not(target_os = "nto")))]
    pub mod settings {
        pub const GLOBAL_CONFIG_PATH: &[u8] = b"/etc";
        pub const USER_CONFIG_PATH: &[u8] = b".config";
        pub const TEMP_DIRECTORY: &[u8] = b"/tmp/";
        pub const TEST_DIRECTORY: &[u8] = b"/tmp/iceoryx2/tests/";
        pub const SHARED_MEMORY_DIRECTORY: &[u8] = b"/dev/shm/";
        pub const PATH_SEPARATOR: u8 = b'/';
        pub const ROOT: &[u8] = b"/";
        pub const REQUIRED_SOCKET_DIRECTORY: Option<&[u8]> = None;
        pub const ICEORYX2_ROOT_PATH: &[u8] = b"/tmp/iceoryx2/";
        pub const FILENAME_LENGTH: usize = 255;
        // it is actually 4096 but to be more compatible with windows and also safe some stack the number
        // is reduced to 255
        pub const PATH_LENGTH: usize = 255;
        #[cfg(not(target_os = "macos"))]
        pub const AT_LEAST_TIMING_VARIANCE: f32 = 0.25;
        #[cfg(target_os = "macos")]
        pub const AT_LEAST_TIMING_VARIANCE: f32 = 1.0;
    }

    #[cfg(target_os = "nto")]
    pub mod settings {
        pub const GLOBAL_CONFIG_PATH: &[u8] = b"/etc";
        pub const USER_CONFIG_PATH: &[u8] = b".config";
        pub const TEMP_DIRECTORY: &[u8] = b"/data/iceoryx2/tmp/";
        pub const TEST_DIRECTORY: &[u8] = b"/data/iceoryx2/tests/";
        pub const SHARED_MEMORY_DIRECTORY: &[u8] = b"/dev/shmem/";
        pub const PATH_SEPARATOR: u8 = b'/';
        pub const ROOT: &[u8] = b"/";
        pub const REQUIRED_SOCKET_DIRECTORY: Option<&[u8]> = None;
        pub const ICEORYX2_ROOT_PATH: &[u8] = b"/data/iceoryx2/";
        pub const FILENAME_LENGTH: usize = 255;
        pub const PATH_LENGTH: usize = 255;
        pub const AT_LEAST_TIMING_VARIANCE: f32 = 0.25;
    }

    #[cfg(target_os = "windows")]
    pub mod settings {
        pub const GLOBAL_CONFIG_PATH: &[u8] = b"C:\\ProgramData";
        pub const USER_CONFIG_PATH: &[u8] = b".config";

        // C:\Temp is not a directory Windows defines. The per-user temporary directory is
        // %TEMP%, which every Windows install has and which C:\Temp is not a synonym for.
        // Defaulting here puts a world-writable directory at the root of the system drive,
        // and on a machine where it does not already exist iceoryx2 fails at node creation
        // with no message naming the path it wanted -- measured as `publisher: no node`.
        //
        // The path must be a compile-time constant, because it builds `Path` values in
        // const contexts. So this is a BUILD setting rather than a runtime one: option_env!
        // reads IOX2_TEMP_DIRECTORY when this crate is compiled, and upstream's default
        // stands when it is unset, so no existing build changes.
        //
        //     IOX2_TEMP_DIRECTORY='C:\Users\me\AppData\Local\Temp\' cargo build ...
        //
        // A trailing separator is required and is not appended here: a const fn cannot
        // concatenate byte strings, and a path silently joined without one would be worse
        // than a path the caller has to spell correctly.
        pub const TEMP_DIRECTORY: &[u8] = match option_env!("IOX2_TEMP_DIRECTORY") {
            Some(dir) => dir.as_bytes(),
            None => b"C:\\Temp\\",
        };
        pub const TEST_DIRECTORY: &[u8] = b"C:\\Temp\\iceoryx2\\tests\\";
        pub const SHARED_MEMORY_DIRECTORY: &[u8] = b"C:\\Temp\\iceoryx2\\shm\\";
        pub const PATH_SEPARATOR: u8 = b'\\';
        pub const ROOT: &[u8] = b"C:\\";
        pub const REQUIRED_SOCKET_DIRECTORY: Option<&[u8]> = None;
        pub const ICEORYX2_ROOT_PATH: &[u8] = b"C:\\Temp\\iceoryx2\\";
        pub const FILENAME_LENGTH: usize = 255;
        pub const PATH_LENGTH: usize = 255;
        pub const AT_LEAST_TIMING_VARIANCE: f32 = 1.0;
    }
}

pub mod settings {
    pub use super::internal::settings::*;
}

pub use settings::*;
