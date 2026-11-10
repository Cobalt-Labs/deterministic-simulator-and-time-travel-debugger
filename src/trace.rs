use crate::event::Event;

pub struct Trace {
    pub events: Vec<Event>,
}

impl Trace {
    pub fn new() -> Self {
        Trace { events: Vec::new() }
    }

    pub fn record(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn truncate(&mut self, len: usize) {
        self.events.truncate(len);
    }
}
