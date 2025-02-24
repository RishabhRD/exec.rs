// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{OperationState, Receiver, Scheduler, Sender};

struct InlineOperationState<R>
where
    R: Receiver<Value = (), Error = ()>,
{
    pub receiver: R,
}

impl<R> OperationState for InlineOperationState<R>
where
    R: Receiver<Value = (), Error = ()>,
{
    fn start(&mut self) {
        self.receiver.set_value(());
    }
}

#[doc(hidden)]
#[derive(Clone)]
pub struct InlineSender {}

impl Sender for InlineSender {
    type Value = ();

    type Error = ();

    fn connect<R>(self, receiver: R) -> impl OperationState
    where
        R: Receiver<Value = Self::Value, Error = Self::Error>,
    {
        InlineOperationState { receiver }
    }
}

/// Scheduler that schedules task on current thread.
pub struct InlineScheduler {}

impl Scheduler for InlineScheduler {
    type ScheduleSender = InlineSender;

    fn schedule(&mut self) -> Self::ScheduleSender {
        InlineSender {}
    }
}
