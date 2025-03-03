// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

pub trait StopCallback {
    /// unregisters the callback.
    ///
    /// # Precondition
    ///   - To be called only once in its lifetime.
    ///
    /// # Postcondition
    ///   - If not called explicitly, should be called by destructor.
    fn unregister(&mut self);
}

pub trait StopToken {
    type CallbackHandle: StopCallback;

    /// Returns true if stop token has stop requested.
    fn stop_requested(&self) -> bool;

    /// Creates a new callback that would be called when stop would be requested.
    ///
    /// Callback would be called on same thread which did request stop.
    /// If token would already have stop_requested() == true at time of creation
    /// of callback, then callback would be called on thread creating stop callback.
    fn register_callback<F>(&mut self, callback: F) -> Self::CallbackHandle
    where
        F: FnMut();
}

pub trait StopSource {
    /// Type of stop token.
    type StopToken;

    /// Requests stop for source.
    fn stop(&mut self);

    /// Returns true if stop has been requested already.
    fn stop_requested();

    /// Returns a stop token for checking stop requested.
    fn stop_token();
}

pub struct NeverStopCallback {}

impl StopCallback for NeverStopCallback {
    fn unregister(&mut self) {}
}

/// A stop token, that never stops.
pub struct NeverStopToken {}

impl StopToken for NeverStopToken {
    type CallbackHandle = NeverStopCallback;

    fn stop_requested(&self) -> bool {
        false
    }

    fn register_callback<F>(&mut self, _callback: F) -> Self::CallbackHandle
    where
        F: FnMut(),
    {
        NeverStopCallback {}
    }
}
