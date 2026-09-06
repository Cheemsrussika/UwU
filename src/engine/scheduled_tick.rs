use std::cmp::Ordering;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TickPriority {
    ExtremelyHigh,
    VeryHigh,
    High,
    Normal,
    Low,
    VeryLow,
    ExtremelyLow,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ScheduledTick<T: Copy + Eq> {
    pub target: T,
    pub pos: (i32, i32, i32),
    pub trigger_tick: u64,
    pub priority: TickPriority,
    pub sub_tick_order: u64,
}

impl<T: Copy + Eq> Ord for ScheduledTick<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse for min-heap
        other.trigger_tick.cmp(&self.trigger_tick)
            .then_with(|| (other.priority as u8).cmp(&(self.priority as u8)))
            .then_with(|| other.sub_tick_order.cmp(&self.sub_tick_order))
    }
}

impl<T: Copy + Eq> PartialOrd for ScheduledTick<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}