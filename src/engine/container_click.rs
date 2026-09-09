use crate::engine::blocks::BlockEntityManager;
use crate::engine::Inventory;
use crate::engine::items::ItemStack;
use super::container::{player_slot_mut, ContainerKind, ContainerRef, SlotTarget};
use super::container_backend::{read_backend, write_backend};
use super::container_insert::quick_insert;
use super::container_click_slots::{handle_left_click, handle_right_click};

pub fn apply_container_click(
    inv: &mut Inventory,
    be: &mut BlockEntityManager,
    c: &ContainerRef,
    target: SlotTarget,
    right: bool,
    shift: bool,
) {
    match target {
        SlotTarget::Player(p) => {
            if p >= 36 { return; }
            if shift {
                let stack = player_slot_mut(inv, p).take();
                if let Some(s) = stack {
                    let rem = quick_insert(be, c, s);
                    if let Some(r) = rem { *player_slot_mut(inv, p) = Some(r); }
                }
                return;
            }
            inv.click_slot(p, right, false);
        }
        SlotTarget::Backend(b) => {
            let size = c.kind.backend_size();
            if b >= size { return; }

            if c.kind == ContainerKind::CraftingTable && b == 9 {
                if inv.carried_item.is_some() { return; }
                if let Some(ct) = be.crafting_tables.get_mut(&c.pos) {
                    if let Some(res) = ct.take_result() { inv.carried_item = Some(res); }
                }
                return;
            }

            if shift {
                let mut ws = read_backend(be, c);
                if let Some(stack) = ws[b].take() {
                    let rem = inv.add_stack(&stack);
                    ws[b] = if rem > 0 { Some(ItemStack::new(stack.item, rem)) } else { None };
                }
                write_backend(be, c, &ws);
                return;
            }

            let mut ws = read_backend(be, c);
            let slot = ws.get_mut(b).unwrap();
            let carried = &mut inv.carried_item;

            if right {
                handle_right_click(slot, carried);
            } else {
                handle_left_click(slot, carried);
            }
            write_backend(be, c, &ws);
        }
    }
}
