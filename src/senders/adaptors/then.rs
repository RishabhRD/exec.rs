// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{Receiver, Sender};

pub struct ThenReceiver<ExternalReceiver, InternalValue, ExternalValue, Operation>
where
    ExternalReceiver: Receiver<Value = ExternalValue>,
    Operation: Fn(InternalValue) -> ExternalValue,
{
    pub external_receiver: ExternalReceiver,
    pub operation: Operation,
    pub v: std::marker::PhantomData<InternalValue>,
}

impl<ExternalReceiver, InternalValue, ExternalValue, Operation> Receiver
    for ThenReceiver<ExternalReceiver, InternalValue, ExternalValue, Operation>
where
    ExternalReceiver: Receiver<Value = ExternalValue>,
    Operation: Fn(InternalValue) -> ExternalValue,
{
    type Value = InternalValue;

    type Error = ExternalReceiver::Error;

    fn set_value(&mut self, value: Self::Value) {
        let v: ExternalValue = (self.operation)(value);
        self.external_receiver.set_value(v);
    }

    fn set_error(&mut self, error: Self::Error) {
        self.external_receiver.set_error(error);
    }

    fn set_cancelled(&mut self) {
        self.external_receiver.set_cancelled();
    }
}

pub struct ThenSender<InternalSender, Operation, OutputValue>
where
    InternalSender: Sender,
    Operation: Fn(InternalSender::Value) -> OutputValue,
{
    pub intenral_sender: InternalSender,
    pub operation: Operation,
}

impl<InternalSender, Operation, OutputValue> Sender
    for ThenSender<InternalSender, Operation, OutputValue>
where
    InternalSender: Sender,
    Operation: Fn(InternalSender::Value) -> OutputValue,
{
    type Value = OutputValue;

    type Error = InternalSender::Error;

    fn connect<R>(self, receiver: R) -> impl crate::OperationState
    where
        R: crate::Receiver<Value = Self::Value, Error = Self::Error>,
    {
        let internal_receiver = ThenReceiver {
            external_receiver: receiver,
            operation: self.operation,
            v: std::marker::PhantomData {},
        };
        self.intenral_sender.connect(internal_receiver)
    }
}

/// Returns a sender which completes with value continuation(InternalSender::Value).
///
/// The resultant sender completes on scheduler where sender completes.
pub fn then<InternalSender, Continuation, OutputValue>(
    sender: InternalSender,
    continuation: Continuation,
) -> ThenSender<InternalSender, Continuation, OutputValue>
where
    InternalSender: Sender,
    Continuation: Fn(InternalSender::Value) -> OutputValue,
{
    ThenSender {
        intenral_sender: sender,
        operation: continuation,
    }
}
