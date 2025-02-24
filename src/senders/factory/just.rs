// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{OperationState, Receiver, Sender};

struct JustOperationState<Value, Receiver> {
    pub value: Option<Value>,
    pub receiver: Receiver,
}

impl<V, R> OperationState for JustOperationState<V, R>
where
    R: Receiver<Value = V>,
{
    fn start(&mut self) {
        self.receiver.set_value(self.value.take().unwrap());
    }
}

pub struct JustSender<Value> {
    value: Value,
}

impl<Value> Sender for JustSender<Value> {
    type Value = Value;

    type Error = ();

    fn connect<R>(self, receiver: R) -> impl OperationState
    where
        R: Receiver<Value = Self::Value, Error = Self::Error>,
    {
        JustOperationState {
            value: Some(self.value),
            receiver,
        }
    }
}

/// Returns a sender, that completes on current context with given value.
pub fn just<Value>(value: Value) -> JustSender<Value> {
    JustSender { value }
}
