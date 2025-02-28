// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

mod then;
use let_value::LetValueSender;
pub use then::then;
use then::ThenSender;

mod let_value;
pub use let_value::let_value;

use crate::Sender;

pub trait SenderExt: Sender + Sized {
    /// Returns a sender which completes with value continuation(InternalSender::Value).
    ///
    /// The resultant sender completes on scheduler where self completes.
    fn then<Continuation, OutputValue>(
        self,
        continuation: Continuation,
    ) -> ThenSender<Self, Continuation, OutputValue>
    where
        Continuation: Fn(Self::Value) -> OutputValue,
    {
        then(self, continuation)
    }

    fn let_value<F, OutputSender>(self, continuation: F) -> LetValueSender<Self, F, OutputSender>
    where
        F: Fn(Self::Value) -> OutputSender,
        OutputSender: Sender<Error = Self::Error>,
    {
        let_value(self, continuation)
    }
}

impl<S> SenderExt for S where S: Sender + Sized {}
