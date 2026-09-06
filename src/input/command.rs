use winit::event::MouseButton;

pub enum LogicCommand {
    MoveInput(glam::Vec3),
    Jump(bool),
    Sneak(bool),
    Sprint(bool),
    RotateCamera(f32),
    ZoomCamera(f32),
    ToggleInventory,
    UpdateCursor {
        aspect: f32,
        mouse_pos: (f32, f32),
        screen_size: (f32, f32),
    },
    MouseAction {
        button: MouseButton,
        is_pressed: bool,
        aspect: f32,
        mouse_pos: (f32, f32),
        screen_size: (f32, f32),
        is_shift: bool,
    },
    SelectSlot(usize),
    NextSlot,
    PrevSlot,
    TogglePieChart,
    ToggleChunkBorders,
    ProfilerNavigate(usize),
    RemoteBlockChange { x: i32, y: i32, z: i32, block_type: u8 },
}