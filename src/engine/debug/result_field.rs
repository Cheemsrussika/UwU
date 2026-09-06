#[derive(Clone, Debug)]
pub struct ResultField {
    pub name: String,
    pub percentage: f64,
    pub global_percentage: f64,
    pub count: u64,
}

impl ResultField {
    pub fn new(name: String, percentage: f64, global_percentage: f64, count: u64) -> Self {
        Self { name, percentage, global_percentage, count }
    }

    /// Minecraft parity color: (name.hashCode() & 11184810) + -12303292
    pub fn get_color(&self) -> [f32; 4] {
        let mut h: u32 = 0;
        for b in self.name.bytes() {
            h = h.wrapping_mul(31).wrapping_add(b as u32);
        }
        let rgb = (h & 0xAAAAAA) | 0x444444;
        let r = ((rgb >> 16) & 0xFF) as f32 / 255.0;
        let g = ((rgb >> 8) & 0xFF) as f32 / 255.0;
        let b = (rgb & 0xFF) as f32 / 255.0;
        [r, g, b, 1.0]
    }
}