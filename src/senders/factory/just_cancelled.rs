// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{OperationState, Receiver, Sender};

struct JustCancelledOperationState<Receiver> {
    pub receiver: Receiver,
}

impl<V, R> OperationState for JustCancelledOperationState<R>
where
    R: Receiver<Error = V>,
{
    fn start(&mut self) {
        self.receiver.set_cancelled();
    }
}

#[derive(Clone)]
pub struct JustCancelledSender {}

impl Sender for JustCancelledSender {
    type Value = ();

    type Error = ();

    fn connect<R>(self, receiver: R) -> impl OperationState
    where
        R: Receiver<Value = Self::Value, Error = Self::Error>,
    {
        JustCancelledOperationState { receiver }
    }
}

/// Returns a sender, that completes on current context with indicating cancellation.
pub fn just_cancelled() -> JustCancelledSender {
    JustCancelledSender {}
}
