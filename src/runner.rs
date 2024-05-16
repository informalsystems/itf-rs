//! Infrastructure for running a trace against a concrete implementation.

use crate::Trace;

pub trait Runner {
    type ActualState;
    type Result;
    type ExpectedState;
    type Error;

    fn init(&mut self, expected: &Self::ExpectedState) -> Result<Self::ActualState, Self::Error>;

    fn step(
        &mut self,
        actual: &mut Self::ActualState,
        expected: &Self::ExpectedState,
    ) -> Result<Self::Result, Self::Error>;

    fn result_invariant(
        &self,
        result: &Self::Result,
        expected: &Self::ExpectedState,
    ) -> Result<bool, Self::Error>;

    fn state_invariant(
        &self,
        actual: &Self::ActualState,
        expected: &Self::ExpectedState,
    ) -> Result<bool, Self::Error>;
}

impl<S> Trace<S> {
    pub fn run_on<R, E>(&self, mut runner: R) -> Result<(), E>
    where
        R: Runner<ExpectedState = S, Error = E>,
    {
        if let Some(expected_init) = self.states.first() {
            eprintln!("step: Initial");
            let mut actual = runner.init(&expected_init.value)?;
            assert!(
                runner.state_invariant(&actual, &expected_init.value)?,
                "State Invariant failed after Initialization"
            );
            for (i, expected) in self.states.iter().enumerate().skip(1) {
                println!("step: {i}");
                let result = runner.step(&mut actual, &expected.value)?;
                assert!(
                    runner.result_invariant(&result, &expected.value)?,
                    "Result Invariant failed after step {i}",
                );
                assert!(
                    runner.state_invariant(&actual, &expected.value)?,
                    "State Invariant failed after step {i}",
                );
            }
        }

        Ok(())
    }
}
