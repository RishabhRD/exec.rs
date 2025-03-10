// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{OperationState, Receiver, Scheduler, Sender};

pub struct InlineOperationState<R>
where
    R: Receiver<Value = (), Error = ()>,
{
    pub receiver: Option<R>,
}

impl<R> OperationState for InlineOperationState<R>
where
    R: Receiver<Value = (), Error = ()>,
{
    fn start(&mut self) {
        self.receiver.take().unwrap().set_value(());
    }
}

#[derive(Clone)]
pub struct InlineSender {}

impl Sender for InlineSender {
    type Value = ();

    type Error = ();

    type OpState<R>
        = InlineOperationState<R>
    where
        R: Receiver<Value = Self::Value, Error = Self::Error>;

    fn connect<R>(self, receiver: R) -> Self::OpState<R>
    where
        R: Receiver<Value = Self::Value, Error = Self::Error>,
    {
        InlineOperationState {
            receiver: Some(receiver),
        }
    }
}

/// Scheduler that schedules task on current thread.
#[derive(Clone)]
pub struct InlineScheduler {}

impl Scheduler for InlineScheduler {
    type ScheduleSender = InlineSender;

    fn schedule(&mut self) -> Self::ScheduleSender {
        InlineSender {}
    }
}
