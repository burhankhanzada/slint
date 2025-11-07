// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

mod gpu_rendering_context;

mod servo_rendering_adapter;
mod surfman_context;

pub use servo_rendering_adapter::ServoRenderingAdapter;

#[cfg(target_vendor = "apple")]
mod metal;

#[cfg(any(target_os = "android", target_os = "windows"))]
pub use servo_rendering_adapter::create_software_context;

#[cfg(not(any(target_os = "android", target_os = "windows")))]
pub use {
    gpu_rendering_context::GPURenderingContext, servo_rendering_adapter::try_create_gpu_context,
};
