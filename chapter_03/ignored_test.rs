// compile in test mode: `rustc --test ignored_test.rs`
// run tests using: `./ignored_test`

pub fn silly_loop() {
    for _ in 1..1_000_000_000 {};
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    pub fn test_silly_loop() {
        ::silly_loop();
    }
}