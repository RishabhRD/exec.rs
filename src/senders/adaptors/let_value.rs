// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{OperationState, Receiver, Sender};

struct LetValueOpState<Op1, Op2>
where
    Op1: OperationState,
    Op2: OperationState,
{
    first_op: Op1,
    second_op: Option<Op2>,
}

impl<Op1, Op2> OperationState for LetValueOpState<Op1, Op2>
where
    Op1: OperationState,
    Op2: OperationState,
{
    fn start(&mut self) {
        self.first_op.start();
    }
}

struct LetValueReceiver<
    'a,
    InternalValue,
    InternalError,
    ExternalSender,
    Continuation,
    ExternalReceiver,
    NextOpState,
> where
    Continuation: Fn(InternalValue) -> ExternalSender,
    ExternalReceiver: Receiver<Error = InternalError>,
    ExternalSender: Sender<Value = ExternalReceiver::Value, Error = ExternalReceiver::Error>,
{
    external_receiver: Option<ExternalReceiver>,
    continuation: Continuation,
    next_op_state: &'a mut Option<NextOpState>,
    dummy: std::marker::PhantomData<InternalValue>,
}

impl<
        'a,
        InternalValue,
        InternalError,
        ExternalSender,
        Continuation,
        ExternalReceiver,
        NextOpState,
    > Receiver
    for LetValueReceiver<
        'a,
        InternalValue,
        InternalError,
        ExternalSender,
        Continuation,
        ExternalReceiver,
        NextOpState,
    >
where
    Continuation: Fn(InternalValue) -> ExternalSender,
    ExternalReceiver: Receiver<Error = InternalError>,
    ExternalSender: Sender<Value = ExternalReceiver::Value, Error = ExternalReceiver::Error>,
{
    type Value = InternalValue;

    type Error = InternalError;

    fn set_value(&mut self, value: Self::Value) {
        let op = (self.continuation)(value).connect(self.external_receiver.take().unwrap());
        self.next_op_state = op;
        op.start();
    }

    fn set_error(&mut self, error: Self::Error) {
        self.external_receiver.as_mut().unwrap().set_error(error);
    }

    fn set_cancelled(&mut self) {
        self.external_receiver.as_mut().unwrap().set_cancelled();
    }
}
