// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use exec::Scheduler;

    #[test]
    fn sync_wait_with_schedule() {
        let mut sch = exec::InlineScheduler {};
        let work = sch.schedule();
        let res = exec::sync_wait(work);
        assert!(res == Some(Ok(())));
    }
}
