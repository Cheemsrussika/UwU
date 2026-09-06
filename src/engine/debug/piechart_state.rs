use super::result_field::ResultField;

#[derive(Clone, Debug)]
pub struct ProfilerPieChartState {
    pub is_open: bool,
    pub current_path: String,
    pub bottom_offset: i32,
}

impl ProfilerPieChartState {
    pub fn new() -> Self {
        Self {
            is_open: false,
            current_path: "root".to_string(),
            bottom_offset: 0,
        }
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    /// Minecraft parity: 0 = parent, 1..=N = child
    pub fn handle_key_press(&mut self, key: usize, fields: &[ResultField]) {
        if key == 0 {
            if let Some(pos) = self.current_path.rfind('.') {
                self.current_path.truncate(pos);
            }
        } else if key < fields.len() {
            let child = &fields[key];
            if child.name != "unspecified" {
                if !self.current_path.is_empty() {
                    self.current_path.push('.');
                }
                self.current_path.push_str(&child.name);
            }
        }
    }
}