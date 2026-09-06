use super::*;
use glam::Vec3;

#[test]
fn test_result_field_color() {
    let f1 = ResultField::new("tick".into(), 45.0, 45.0, 1);
    let col = f1.get_color();
    assert!(col[0] >= 0.0 && col[0] <= 1.0);
    assert!(col[1] >= 0.0 && col[1] <= 1.0);
    assert!(col[2] >= 0.0 && col[2] <= 1.0);
    assert_eq!(col[3], 1.0);
}

#[test]
fn test_profiler_hierarchy() {
    let mut p = Profiler::new();
    p.start_tick();
    p.push("tick");
    p.push("physics");
    std::thread::sleep(std::time::Duration::from_millis(1));
    p.pop();
    p.push("fluid");
    std::thread::sleep(std::time::Duration::from_millis(1));
    p.pop();
    p.pop();
    p.end_tick();

    let root_times = p.get_times("root");
    assert!(!root_times.is_empty());
    assert_eq!(root_times[0].name, "root");

    let tick_times = p.get_times("root.tick");
    assert!(!tick_times.is_empty());
    assert_eq!(tick_times[0].name, "tick");
}

#[test]
fn test_piechart_navigation() {
    let mut state = ProfilerPieChartState::new();
    assert!(!state.is_open);
    state.toggle();
    assert!(state.is_open);

    let fields = vec![
        ResultField::new("root".into(), 100.0, 100.0, 1),
        ResultField::new("tick".into(), 60.0, 60.0, 1),
        ResultField::new("render".into(), 40.0, 40.0, 1),
    ];

    state.handle_key_press(1, &fields);
    assert_eq!(state.current_path, "root.tick");

    state.handle_key_press(0, &fields);
    assert_eq!(state.current_path, "root");
}

#[test]
fn test_chunk_border_mesh() {
    let (v, i) = crate::render::build_chunk_border_mesh(Vec3::new(5.0, 4.0, 12.0));
    assert!(!v.is_empty());
    assert!(!i.is_empty());
    assert_eq!(i.len() % 3, 0); // Valid triangle topology
}