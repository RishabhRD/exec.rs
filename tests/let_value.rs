// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use exec::{sync_wait, InlineScheduler, Scheduler, SenderExt};

    #[test]
    fn let_value() {
        let work = InlineScheduler {}
            .schedule()
            .then(|_| 2)
            .let_value(|x| exec::just(x * 2))
            .then(|x| x + 1)
            .let_value(|x| exec::just(x + 1));
        let res = sync_wait(work);
        assert_eq!(res, Some(Ok(6)))
    }
}
