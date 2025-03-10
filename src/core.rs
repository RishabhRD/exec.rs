// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

/// Represents executable task.
pub trait OperationState {
    /// Start the task represented by operation state.
    ///
    /// # Precondition
    ///   - Called no more than 1 time.
    ///   - self would not be moved after call to start is done.
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

    /// Type of operation state obtained on connect.
    type OpState<R>: OperationState
    where
        R: Receiver<Value = Self::Value, Error = Self::Error>;

    /// Connects sender with receiver to produce executable task represented
    /// by operation state.
    fn connect<R>(self, receiver: R) -> Self::OpState<R>
    where
        R: Receiver<Value = Self::Value, Error = Self::Error>;
}

/// Represents handle to execution context.
pub trait Scheduler: Clone {
    /// Type of sender obtained from schedule opertaion.
    type ScheduleSender: Sender;

    /// Returns a sender that represent task scheduled on scheduler.
    fn schedule(&mut self) -> Self::ScheduleSender;
}
