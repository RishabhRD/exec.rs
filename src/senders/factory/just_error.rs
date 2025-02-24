// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{OperationState, Receiver, Sender};

struct JustErrorOperationState<Error, Receiver> {
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

pub struct JustErrorSender<Error> {
    error: Error,
}

impl<Error> Sender for JustErrorSender<Error> {
    type Value = ();

    type Error = Error;

    fn connect<R>(self, receiver: R) -> impl OperationState
    where
        R: Receiver<Value = Self::Value, Error = Self::Error>,
    {
        JustErrorOperationState {
            error: Some(self.error),
            receiver,
        }
    }
}

pub fn just_error<Error>(error: Error) -> JustErrorSender<Error> {
    JustErrorSender { error }
}
