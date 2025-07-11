/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot::init::{gdextension, ExtensionLibrary, InitLevel};

pub mod async_runtimes;
mod benchmarks;
mod builtin_tests;
mod common;
mod engine_tests;
mod framework;
mod object_tests;
mod register_tests;

// Import the async runtime integration
use async_runtimes::TokioIntegration;
use godot::task::register_runtime;

// ----------------------------------------------------------------------------------------------------------------------------------------------
// Entry point

#[gdextension(entry_symbol = itest_init)]
unsafe impl ExtensionLibrary for framework::IntegrationTests {
    fn on_level_init(level: InitLevel) {
        // Register the async runtime early in the initialization process
        // This is the proper way to integrate async runtimes with gdext
        if level == InitLevel::Scene {
            register_runtime::<TokioIntegration>().expect("Failed to register tokio runtime");
        }

        // Testing that we can initialize and use `Object`-derived classes during `Servers` init level. See `object_tests::init_level_test`.
        object_tests::initialize_init_level_test(level);
    }
}
