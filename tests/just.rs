// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    #[test]
    fn just() {
        let work = exec::just(2);
        let res = exec::sync_wait(work);
        assert_eq!(res, Some(Ok(2)));
    }

    #[test]
    fn just_error() {
        let work = exec::just_error(2);
        let res = exec::sync_wait(work);
        assert_eq!(res, Some(Err(2)));
    }
}
