use crate::event::Event;

pub trait System: Clone + Default {
    fn process(&mut self, event: &Event);
}
