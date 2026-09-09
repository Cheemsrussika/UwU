use crate::engine::{BlockEntityManager, ContainerKind, ContainerRef, ContainerSnapshot};

pub fn extract_container_snapshot(
    open_container: Option<ContainerRef>,
    block_entities: &BlockEntityManager,
) -> Option<ContainerSnapshot> {
    open_container.map(|oc| {
        let ws = crate::engine::container::read_backend(block_entities, &oc);
        let (prog_a, prog_b) = match oc.kind {
            ContainerKind::Furnace => {
                if let Some(f) = block_entities.furnaces.get(&oc.pos) {
                    let fl = if f.current_fuel_total > 0.0 { (f.burn_time / f.current_fuel_total).clamp(0.0, 1.0) } else { 0.0 };
                    let cook = (f.cook_time / crate::engine::blocks::furnace::COOK_TIME_STANDARD).clamp(0.0, 1.0);
                    (fl, cook)
                } else { (0.0, 0.0) }
            }
            _ => (0.0, 0.0),
        };
        ContainerSnapshot {
            kind: oc.kind,
            items: ws.iter().map(|s| s.map(|st| (st.item, st.count))).collect(),
            prog_a,
            prog_b,
        }
    })
}
