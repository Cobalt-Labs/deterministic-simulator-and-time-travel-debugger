#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Tick,
    Input(u32),
    NetworkMessage(u32),
}

#[derive(Debug, Clone)]
pub struct TimedEvent {
    pub time: u64,
    pub event: Event,
}
