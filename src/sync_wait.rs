// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::Sender;

/// Perform a blocking wait on `sender` and return result of its computation.
///
/// Returns the result of task `res` such that:
///   - if res == None, then task was cancelled.
///   - if res completes if value, Result has Ok, otherwise has Err.
pub fn sync_wait<S>(_sender: S) -> Option<Result<S::Value, S::Error>>
where
    S: Sender,
{
    todo!();
}
