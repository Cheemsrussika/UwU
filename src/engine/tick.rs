use std::collections::{BinaryHeap, HashSet};
use super::scheduled_tick::{ScheduledTick, TickPriority};

pub struct TickSystem<T: Copy + Eq + std::hash::Hash> {
    pub tick_rate: f32,
    accumulator: f32,
    pub tick_count: u64,
    sub_tick_counter: u64,
    queue: BinaryHeap<ScheduledTick<T>>,
    scheduled_set: HashSet<(T, (i32, i32, i32))>,
}

impl<T: Copy + Eq + std::hash::Hash> TickSystem<T> {
    pub fn new(tick_rate: f32) -> Self {
        Self {
            tick_rate, accumulator: 0.0, tick_count: 0, sub_tick_counter: 0,
            queue: BinaryHeap::new(), scheduled_set: HashSet::new(),
        }
    }

    pub fn schedule_tick(&mut self, target: T, pos: (i32, i32, i32), delay: u64, priority: TickPriority) {
        if self.scheduled_set.insert((target, pos)) {
            self.sub_tick_counter += 1;
            self.queue.push(ScheduledTick {
                target, pos, trigger_tick: self.tick_count + delay, priority,
                sub_tick_order: self.sub_tick_counter,
            });
        }
    }

    pub fn advance(&mut self, dt: f32) -> u32 {
        self.accumulator += dt;
        let tick_time = 1.0 / self.tick_rate;
        let mut ticks = 0;
        while self.accumulator >= tick_time && ticks < 5 {
            self.accumulator -= tick_time;
            self.tick_count += 1;
            ticks += 1;
        }
        ticks
    }

    pub fn drain_current_ticks(&mut self) -> Vec<ScheduledTick<T>> {
        let mut ready = Vec::new();
        while let Some(top) = self.queue.peek() {
            if top.trigger_tick <= self.tick_count {
                let tick = self.queue.pop().unwrap();
                self.scheduled_set.remove(&(tick.target, tick.pos));
                ready.push(tick);
            } else {
                break;
            }
        }
        ready
    }
}