extern crate proptest;

use proptest::test_runner::TestRunner;
use proptest::strategy::Strategy;
use proptest::strategy::ValueTree;

#[derive(Default)]
pub struct SeedGen {
    runner: TestRunner,
}

impl SeedGen {
    /// Creates a new value generator with the default RNG.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new value generator with a deterministic RNG.
    pub fn deterministic() -> Self {
        Self {
            runner: TestRunner::deterministic(),
        }
    }

    /// Generates a single value for this strategy.
    pub fn generate<S: Strategy>(&mut self, strategy: S) -> S::Value {
        strategy
            .new_tree(&mut self.runner)
            .expect("Some went wrong with generating seeds!")
            .current()
    }
}
