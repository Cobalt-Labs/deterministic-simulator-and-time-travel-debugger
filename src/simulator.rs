use crate::event::Event;
use crate::snapshot::SnapshotCache;
use crate::system::System;
use crate::trace::Trace;

pub struct Simulator<S: System> {
    system: S,
    trace: Trace,
    cache: SnapshotCache<S>,
    current_step: usize,
}

impl<S: System> Simulator<S> {
    pub fn new(snapshot_interval: usize) -> Self {
        let initial = S::default();
        let mut cache = SnapshotCache::new(snapshot_interval);
        cache.maybe_snapshot(0, &initial);

        Self {
            system: initial,
            trace: Trace::new(),
            current_step: 0,
            cache,
        }
    }

    pub fn step(&mut self, event: Event) {
        if self.current_step < self.trace.events.len() {
            self.trace.truncate(self.current_step);
            self.cache.truncate(self.current_step);
        }

        self.system.process(&event);
        self.trace.record(event);
        self.current_step += 1;
        self.cache.maybe_snapshot(self.current_step, &self.system);
    }

    pub fn rewind_to(&mut self, target_step: usize) {
        if target_step > self.current_step {
            panic!(
                "Can not rewind time to the future! (Current: {}, Target: {})",
                self.current_step, target_step
            );
        }
        if target_step == self.current_step {
            return;
        }

        let snapshot = self.cache.get_closest(target_step);

        self.system = snapshot.state;
        self.current_step = snapshot.step;

        for i in self.current_step..target_step {
            let event = self.trace.events[i].clone();
            self.system.process(&event);
        }

        self.current_step = target_step;
    }

    pub fn system(&self) -> &S {
        &self.system
    }

    pub fn current_step(&self) -> usize {
        self.current_step
    }

    pub fn state(&self) -> &S {
        &self.system
    }
}
