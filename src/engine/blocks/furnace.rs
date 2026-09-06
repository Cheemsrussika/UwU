use bevy::prelude::Component;
use crate::engine::items::ItemStack;
use super::furnace_fuel::get_fuel_burn_time;
use super::furnace_recipe::get_smelting_result;

pub const COOK_TIME_STANDARD: f32 = 10.0;

#[derive(Component, Clone, Debug, Default)]
pub struct Furnace {
    pub input: Option<ItemStack>,
    pub fuel: Option<ItemStack>,
    pub output: Option<ItemStack>,
    pub cook_time: f32,
    pub burn_time: f32,
    pub current_fuel_total: f32,
}

impl Furnace {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_burning(&self) -> bool {
        self.burn_time > 0.0
    }

    pub fn update(&mut self, dt: f32) {
        if self.burn_time > 0.0 {
            self.burn_time = (self.burn_time - dt).max(0.0);
        }

        let can_smelt = self.can_smelt();
        if !self.is_burning() && can_smelt {
            self.consume_fuel();
        }

        if self.is_burning() && can_smelt {
            self.cook_time += dt;
            if self.cook_time >= COOK_TIME_STANDARD {
                self.cook_time = 0.0;
                self.produce_output();
            }
        } else if !can_smelt {
            self.cook_time = (self.cook_time - dt * 2.0).max(0.0);
        }
    }

    fn can_smelt(&self) -> bool {
        let in_item = match self.input.as_ref() { Some(s) if s.count > 0 => s.item, _ => return false };
        let out_type = match get_smelting_result(in_item) { Some(t) => t, None => return false };
        match self.output.as_ref() {
            None => true,
            Some(out) => out.item == out_type && out.count < out_type.max_stack_size(),
        }
    }

    fn consume_fuel(&mut self) {
        if let Some(fuel) = self.fuel.as_mut() {
            if let Some(time) = get_fuel_burn_time(fuel.item) {
                self.burn_time = time;
                self.current_fuel_total = time;
                if fuel.count <= 1 { self.fuel = None; } else { fuel.count -= 1; }
            }
        }
    }

    fn produce_output(&mut self) {
        let in_item = match self.input.as_mut() {
            Some(s) => {
                let it = s.item;
                if s.count <= 1 { self.input = None; } else { s.count -= 1; }
                it
            }
            None => return,
        };
        let out_type = match get_smelting_result(in_item) { Some(t) => t, None => return };
        match self.output.as_mut() {
            Some(out) => out.count += 1,
            None => self.output = Some(ItemStack::new(out_type, 1)),
        }
    }
}
