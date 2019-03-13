use crate::builders::TuringMachineBuilder;
use crate::stats::{ExecutionResult, TuringMachineStatsExt};
use crate::TuringMachine;

pub struct LimitedTuringMachineExt<T> {
    tm: TuringMachineStatsExt<T>,
    max_steps: usize,
    limited: bool,
}

impl<T: TuringMachine> LimitedTuringMachineExt<T> {
    pub fn new(tm: T) -> Self {
        Self {
            tm: TuringMachineStatsExt::new(tm),
            max_steps: 0,
            limited: false,
        }
    }

    pub fn new_with_limit(tm: T, max_steps: usize) -> Self {
        Self {
            tm: TuringMachineStatsExt::new(tm),
            max_steps,
            limited: true,
        }
    }

    pub fn get_number_of_steps(&self) -> usize {
        self.tm.get_number_of_steps()
    }

    pub fn execute_and_get_result(mut self) -> ExecutionResult<T> {
        while !(self.is_accepting() || self.is_rejecting()) {
            self.step();
        }

        ExecutionResult {
            accepting: self.is_accepting(),
            tape: self.tape().clone(),
            num_steps: self.tm.get_number_of_steps(),
            tm: self.tm.inner(),
        }
    }
}

impl<T: TuringMachine> TuringMachine for LimitedTuringMachineExt<T> {
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
        if self.is_accepting() || self.is_rejecting() {
            return;
        }
        self.tm.step();
    }

    fn tape(&self) -> &Self::Tape {
        self.tm.tape()
    }

    fn is_accepting(&self) -> bool {
        if self.limited && self.max_steps < self.get_number_of_steps() {
            false
        } else {
            self.tm.is_accepting()
        }
    }

    fn is_rejecting(&self) -> bool {
        if self.limited && self.max_steps < self.get_number_of_steps() {
            true
        } else {
            self.tm.is_rejecting()
        }
    }
}
