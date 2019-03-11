use log::debug;

use crate::builders::TuringMachineBuilder;
use crate::TuringMachine;

/// The result of a [`TuringMachine`](trait.TuringMachine.html) run
pub struct ExecutionResult<T: TuringMachine> {
    /// Did the machine accept the input?
    pub accepting: bool,

    /// The end tape
    pub tape: T::Tape,

    /// How many steps did it take to stop
    pub num_steps: usize,

    pub tm: T,
}

/// A wrapper struct, which takes a [`TuringMachine`](trait.TuringMachine.html) and collects information about the number of steps it takes
pub struct TuringMachineStatsExt<T> {
    tm: T,
    num_steps: usize,
}

impl<T: TuringMachine> TuringMachineStatsExt<T> {
    /// Decorate an existing [`TuringMachine`](trait.TuringMachine.html) with the step-counter
    pub fn new(tm: T) -> Self {
        Self { tm, num_steps: 0 }
    }

    /// Return how many steps the machine has done so far
    pub fn get_number_of_steps(&self) -> usize {
        self.num_steps
    }

    /// Runs to completion, and returns the execution result associated with it
    pub fn execute_and_get_result(mut self) -> ExecutionResult<T> {
        debug!("Starting Execution");

        let accepting = self.run();
        ExecutionResult {
            accepting,
            tape: self.tape().clone(),
            num_steps: self.get_number_of_steps(),
            tm: self.tm,
        }
    }
}

impl<T: TuringMachine> TuringMachine for TuringMachineStatsExt<T> {
    // Just forward everything to the corresponding operation

    type Tape = T::Tape;
    type StateTy = T::StateTy;
    type ReprTy = T::ReprTy;
    type ErrorTy = T::ErrorTy;

    fn from_builder(
        builder: TuringMachineBuilder<Self::StateTy, Self::ReprTy>,
    ) -> Result<Self, Self::ErrorTy> {
        Ok(Self::new(T::from_builder(builder)?))
    }

    fn step(&mut self) {
        self.num_steps = self.num_steps.saturating_add(1);
        debug!("Step #{}", self.num_steps);
        self.tm.step();
    }

    fn tape(&self) -> &Self::Tape {
        self.tm.tape()
    }

    fn is_accepting(&self) -> bool {
        self.tm.is_accepting()
    }

    fn is_rejecting(&self) -> bool {
        self.tm.is_rejecting()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::*;
    use crate::mocking::MockMachine;

    fn make_mock_machine<T>(t: Vec<char>) -> MockMachine<T>
    where
        T: StateTrait,
    {
        MockMachine {
            p: Default::default(),
            tape: t,
        }
    }

    #[test]
    fn check_initialization() {
        let stats = TuringMachineStatsExt::new(());
        assert_eq!(stats.get_number_of_steps(), 0);
    }

    #[test]
    fn check_stepping() {
        let mock = make_mock_machine::<usize>(vec!['1', '2', '3', '4']);
        let mut stats = TuringMachineStatsExt::new(mock);

        for i in 0..1000 {
            assert_eq!(stats.is_accepting(), false);
            assert_eq!(stats.is_rejecting(), false);
            assert_eq!(stats.get_number_of_steps(), i);
            assert_eq!(*stats.tape(), vec!['1', '2', '3', '4']);
            stats.step();
        }
    }

}
