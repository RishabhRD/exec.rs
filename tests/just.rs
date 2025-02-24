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
}
