// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

//! exec.rs
//! The module defines basics for asynchronous task and its execution.
//!
//! An asynchronous task is a function whose operation would be completed in
//! future.
//!
//!  ------------------------
//! | Sender -----> Receiver |
//!  ------------------------
//!     Operation State
//!
//! The above diagram shows the model for async task.

/// Represents executable task.
pub trait OperationState {
    /// Start the task represented by operation state.
    fn start(&mut self);
}

/// This models the part of task which contains callback when task completes.
///
/// A task can be completed with Value, Error or cancelled.
pub trait Receiver {
    /// Type of value with which task can be completed.
    type Value;

    /// Type of error with which task can be completed.
    type Error;

    /// Callback for task, when it completes with value.
    fn set_value(self, value: Self::Value);

    /// Callback for task, when it completes with error.
    fn set_error(self, error: Self::Error);

    /// Callback for task, when it gets cancelled.
    fn set_cancelled(self);
}

/// This models the part of task which represents the work upon completion of
/// which, receiver's callback should be called.
pub trait Sender {
    /// Type of value with which task can be completed.
    type Value;

    /// Type of error with which task can be completed.
    type Error;

    /// Connects sender with receiver to produce executable task represented
    /// by operation state.
    fn connect<R>(self, receiver: R) -> impl OperationState
    where
        R: Receiver<Value = Self::Value, Error = Self::Error>;
}

/// Represents handle to execution context.
pub trait Scheduler {
    /// Type of sender obtained from scheduling a task on scheduler.
    type Sender;

    /// Returns a sender that represent task scheduled on scheduler.
    fn schedule(&mut self) -> Self::Sender;
}
