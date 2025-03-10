// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{OperationState, Receiver, Sender};

pub struct JustCancelledOperationState<Receiver> {
    pub receiver: Option<Receiver>,
}

impl<V, R> OperationState for JustCancelledOperationState<R>
where
    R: Receiver<Error = V>,
{
    fn start(&mut self) {
        self.receiver.take().unwrap().set_cancelled();
    }
}

#[derive(Clone)]
pub struct JustCancelledSender {}

impl Sender for JustCancelledSender {
    type Value = ();

    type Error = ();

    type OpState<R>
        = JustCancelledOperationState<R>
    where
        R: Receiver<Value = Self::Value, Error = Self::Error>;

    fn connect<R>(self, receiver: R) -> Self::OpState<R>
    where
        R: Receiver<Value = Self::Value, Error = Self::Error>,
    {
        JustCancelledOperationState {
            receiver: Some(receiver),
        }
    }
}

/// Returns a sender, that completes on current context with indicating cancellation.
pub fn just_cancelled() -> JustCancelledSender {
    JustCancelledSender {}
}
