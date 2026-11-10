use crate::system::System;

pub struct Snapshot<S: System> {
    pub step: usize,
    pub state: S,
}

pub struct SnapshotCache<S: System> {
    initial_state: S,
    snapshots: Vec<Snapshot<S>>,
    interval: usize,
}

impl<S: System> SnapshotCache<S> {
    pub fn new(interval: usize) -> Self {
        Self {
            initial_state: S::default(),
            snapshots: Vec::new(),
            interval: interval.max(1),
        }
    }

    pub fn maybe_snapshot(&mut self, step: usize, state: &S) {
        if step % self.interval == 0 {
            self.snapshots.push(Snapshot {
                step,
                state: state.clone(),
            });
        }
    }

    pub fn get_closest(&self, target_step: usize) -> Snapshot<S> {
        let idx = self.snapshots.partition_point(|s| s.step <= target_step);

        if idx == 0 {
            Snapshot {
                step: 0,
                state: self.initial_state.clone(),
            }
        } else {
            self.snapshots[idx - 1].clone()
        }
    }

    pub fn truncate(&mut self, current_step: usize) {
        self.snapshots.retain(|s| s.step <= current_step);
    }
}

impl<S: System> Clone for Snapshot<S> {
    fn clone(&self) -> Self {
        Self {
            step: self.step,
            state: self.state.clone(),
        }
    }
}
