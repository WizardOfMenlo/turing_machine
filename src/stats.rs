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
    pub fn execute_and_get_result(&mut self) -> ExecutionResult<T> {
        let accepting = self.run();
        ExecutionResult {
            accepting,
            tape: self.tape().clone(),
            num_steps: self.get_number_of_steps(),
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
        self.num_steps.saturating_add(1);
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
