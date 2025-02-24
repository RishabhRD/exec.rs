// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

//! ## exec.rs
//!
//! The module defines basics for asynchronous task and its execution.
//!
//! An asynchronous task is a function whose operation would be completed in
//! future.
//!
//! ```text
//!  ------------------------
//! | Sender -----> Receiver |
//!  ------------------------
//!     Operation State
//! ```
//!
//! The above diagram shows the model for async task.

#[doc(hidden)]
pub mod core;
#[doc(inline)]
pub use core::*;

#[doc(hidden)]
pub mod sync_wait;
#[doc(inline)]
pub use sync_wait::*;

pub mod schedulers;
