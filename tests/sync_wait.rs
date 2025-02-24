#[cfg(test)]
pub mod tests {
    use exec::*;

    #[test]
    fn sync_wait_with_schedule() {
        let mut sch = schedulers::InlineScheduler {};
        let work = sch.schedule();
        let res = sync_wait(work);
        assert!(res == Some(Ok(())));
    }
}
