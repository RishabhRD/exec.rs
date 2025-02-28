// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{OperationState, Receiver, Sender};

pub struct JustErrorOperationState<Error, Receiver> {
    pub error: Option<Error>,
    pub receiver: Receiver,
}

impl<V, R> OperationState for JustErrorOperationState<V, R>
where
    R: Receiver<Error = V>,
{
    fn start(&mut self) {
        self.receiver.set_error(self.error.take().unwrap());
    }
}

#[derive(Clone)]
pub struct JustErrorSender<Error> {
    error: Error,
}

impl<Error> Sender for JustErrorSender<Error> {
    type Value = ();

    type Error = Error;

    type OpState<R>
        = JustErrorOperationState<Error, R>
    where
        R: Receiver<Value = Self::Value, Error = Self::Error>;

    fn connect<R>(self, receiver: R) -> Self::OpState<R>
    where
        R: Receiver<Value = Self::Value, Error = Self::Error>,
    {
        JustErrorOperationState {
            error: Some(self.error),
            receiver,
        }
    }
}

/// Returns a sender, that completes on current context with given error.
pub fn just_error<Error>(error: Error) -> JustErrorSender<Error> {
    JustErrorSender { error }
}
