// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use std::sync::{Arc, Condvar, Mutex};

use crate::{OperationState, Receiver, Sender};

type TaskResult<Value, Error> = Option<Option<Result<Value, Error>>>;

struct NotifyReceiver<Value, Error> {
    pub shared_state: Arc<(Mutex<TaskResult<Value, Error>>, Condvar)>,
}

impl<V, E> Receiver for NotifyReceiver<V, E> {
    type Value = V;

    type Error = E;

    fn set_value(&mut self, value: Self::Value) {
        let (lock, cvar) = &*self.shared_state;
        *lock.lock().unwrap() = Some(Some(Ok(value)));
        cvar.notify_one();
    }

    fn set_error(&mut self, error: Self::Error) {
        let (lock, cvar) = &*self.shared_state;
        *lock.lock().unwrap() = Some(Some(Err(error)));
        cvar.notify_one();
    }

    fn set_cancelled(&mut self) {
        let (lock, cvar) = &*self.shared_state;
        *lock.lock().unwrap() = Some(None);
        cvar.notify_one();
    }
}

/// Perform a blocking wait on `sender` and return result of its computation.
///
/// Returns the result of task `res` such that:
///   - if res == None, then task was cancelled.
///   - if res completes if value, Result has Ok, otherwise has Err.
pub fn sync_wait<S>(sender: S) -> Option<Result<S::Value, S::Error>>
where
    S: Sender,
{
    let shared_state = Arc::new((Mutex::new(None), Condvar::new()));
    let receiver = NotifyReceiver {
        shared_state: shared_state.clone(),
    };
    let mut op = sender.connect(receiver);
    op.start();

    let (lock, cvar) = &*shared_state;
    let mut state = lock.lock().unwrap();
    while state.is_none() {
        state = cvar.wait(state).unwrap();
    }
    let result: Option<Result<S::Value, S::Error>> = state.take().unwrap();
    result
}
