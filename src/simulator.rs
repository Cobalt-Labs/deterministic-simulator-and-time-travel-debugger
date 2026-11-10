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

    
}