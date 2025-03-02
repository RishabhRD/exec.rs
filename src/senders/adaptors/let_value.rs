// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{OperationState, Receiver, Sender};

pub struct LetValueOpState<FirstSender, Continuation, SecondSender, ExternalReceiver>
where
    FirstSender: Sender<Error = SecondSender::Error>,
    Continuation: Fn(FirstSender::Value) -> SecondSender,
    SecondSender: Sender<Value = ExternalReceiver::Value, Error = ExternalReceiver::Error>,
    ExternalReceiver: Receiver,
{
    first_sender: Option<FirstSender>,
    continuation: Option<Continuation>,
    external_receiver: Option<ExternalReceiver>,
    #[allow(clippy::type_complexity)]
    first_op: Option<
        FirstSender::OpState<
            LetValueReceiver<FirstSender::Value, SecondSender, Continuation, ExternalReceiver>,
        >,
    >,
    second_op: Option<SecondSender::OpState<ExternalReceiver>>,
}

impl<FirstSender, Continuation, SecondSender, ExternalReceiver> OperationState
    for LetValueOpState<FirstSender, Continuation, SecondSender, ExternalReceiver>
where
    FirstSender: Sender<Error = SecondSender::Error>,
    Continuation: Fn(FirstSender::Value) -> SecondSender,
    ExternalReceiver: Receiver,
    SecondSender: Sender<Value = ExternalReceiver::Value, Error = ExternalReceiver::Error>,
{
    fn start(&mut self) {
        let let_rec = LetValueReceiver {
            external_receiver: self.external_receiver.take().unwrap(),
            continuation: self.continuation.take().unwrap(),
            dummy: std::marker::PhantomData {},
            next_op_state: &mut self.second_op,
        };
        self.first_op = Some(self.first_sender.take().unwrap().connect(let_rec));
        self.first_op.as_mut().unwrap().start();
    }
}

pub struct LetValueReceiver<FirstValue, SecondSender, Continuation, ExternalReceiver>
where
    Continuation: Fn(FirstValue) -> SecondSender,
    ExternalReceiver: Receiver,
    SecondSender: Sender<Value = ExternalReceiver::Value, Error = ExternalReceiver::Error>,
{
    external_receiver: ExternalReceiver,
    continuation: Continuation,
    next_op_state: *mut Option<SecondSender::OpState<ExternalReceiver>>,
    dummy: std::marker::PhantomData<FirstValue>,
}

impl<FirstValue, SecondSender, Continuation, ExternalReceiver> Receiver
    for LetValueReceiver<FirstValue, SecondSender, Continuation, ExternalReceiver>
where
    Continuation: Fn(FirstValue) -> SecondSender,
    ExternalReceiver: Receiver,
    SecondSender: Sender<Value = ExternalReceiver::Value, Error = ExternalReceiver::Error>,
{
    type Value = FirstValue;

    type Error = ExternalReceiver::Error;

    fn set_value(self, value: Self::Value) {
        let op = (self.continuation)(value).connect(self.external_receiver);
        unsafe {
            *self.next_op_state = Some(op);
            self.next_op_state
                .as_mut()
                .unwrap()
                .as_mut()
                .unwrap()
                .start();
        }
    }

    fn set_error(self, error: Self::Error) {
        self.external_receiver.set_error(error);
    }

    fn set_cancelled(self) {
        self.external_receiver.set_cancelled();
    }
}

pub struct LetValueSender<FirstSender, Continuation, SecondSender>
where
    FirstSender: Sender,
    Continuation: Fn(FirstSender::Value) -> SecondSender,
    SecondSender: Sender<Error = FirstSender::Error>,
{
    first_sender: FirstSender,
    continuation: Continuation,
}

impl<FirstSender, Continuation, SecondSender> Sender
    for LetValueSender<FirstSender, Continuation, SecondSender>
where
    FirstSender: Sender,
    Continuation: Fn(FirstSender::Value) -> SecondSender,
    SecondSender: Sender<Error = FirstSender::Error>,
{
    type Value = SecondSender::Value;

    type Error = SecondSender::Error;

    type OpState<R>
        = LetValueOpState<FirstSender, Continuation, SecondSender, R>
    where
        R: Receiver<Value = Self::Value, Error = Self::Error>;

    fn connect<R>(self, receiver: R) -> Self::OpState<R>
    where
        R: Receiver<Value = Self::Value, Error = Self::Error>,
    {
        LetValueOpState {
            first_sender: Some(self.first_sender),
            continuation: Some(self.continuation),
            external_receiver: Some(receiver),
            first_op: None,
            second_op: None,
        }
    }
}

pub fn let_value<S, F, OS>(sender: S, continuation: F) -> LetValueSender<S, F, OS>
where
    S: Sender,
    F: Fn(S::Value) -> OS,
    OS: Sender<Error = S::Error>,
{
    LetValueSender {
        first_sender: sender,
        continuation,
    }
}
