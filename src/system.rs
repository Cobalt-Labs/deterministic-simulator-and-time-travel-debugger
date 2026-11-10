use crate::event::Event;

pub trait System {
    fn process(&mut self, event: &Event);
}