use crate::event::Event;
use crate::system::System;
use crate::trace::Trace;

pub struct Simulator<S: System> {
    system: S, 
    trace: Trace, 
    current_step: usize,
}

impl<S: System> Simulator<S> {
    pub fn new(system: S) -> Self {
        Self { system, trace: Trace::new(), current_step: 0 }
    }

    pub fn step(&mut self, event: Event) {
        if self.current_step < self.trace.events.len() {
            self.trace.truncate(self.current_step);
        }

        self.system.process(&event);
        self.trace.record(event);
        self.current_step += 1;
    }

    pub fn rewind_to(&mut self, target_step: usize) {
        if target_step > self.trace.events.len() {
            panic!("Can not rewind time behind recorded histroy!");
        }
        if target_step == self.current_step {
            return
        }
        todo!("Implement system reset/snapshot");
    }

    pub fn state(&self) -> &S {
        &self.system
    }

    pub fn current_step(&self) -> usize {
        self.current_step
    }
}