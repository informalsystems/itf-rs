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

    fn test(
        &mut self,
        mut trace: impl Iterator<Item = Self::ExpectedState>,
    ) -> Result<(), Self::Error> {
        if let Some(expected_init) = trace.next() {
            eprintln!("step: Initial");
            let mut actual = self.init(&expected_init)?;
            assert!(
                self.state_invariant(&actual, &expected_init)?,
                "State Invariant failed after Initialization"
            );
            for (i, expected) in trace.enumerate() {
                println!("step: {}", i + 1);
                let result = self.step(&mut actual, &expected)?;
                assert!(
                    self.result_invariant(&result, &expected)?,
                    "Result Invariant failed after step {}",
                    i + 1
                );
                assert!(
                    self.state_invariant(&actual, &expected)?,
                    "State Invariant failed after step {}",
                    i + 1
                );
            }
        }

        Ok(())
    }
}
