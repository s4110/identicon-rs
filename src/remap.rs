
pub struct Remap {
    target: u32,
}

impl Remap {
    pub fn new(target: u32) -> Self {
        Self { target }
    }

    // https://qiita.com/mao_/items/0bcae49d4ea1a5ac0cf0
    fn remap(
        target_value: u32,
        original_range_min: u32,
        original_range_max: u32,
        target_range_min: u32,
        target_range_max: u32,
    ) -> f32 {
        let v1 = (target_range_min + (target_range_max - target_range_min)) as f32;
        let v2 = (target_value - original_range_min) as f32;
        let v3 = (original_range_max - original_range_min) as f32;

        v1 * v2 / v3
    }

    pub fn to_hue(&self) -> f32 {
        Remap::remap(self.target, 0, 4095, 0, 360)
    }

    pub fn to_sat(&self) -> f32 {
        let result = Remap::remap(self.target, 0, 255, 0, 20);
        65.0 - result
    }

    pub fn to_lum(&self) -> f32 {
        let result = Remap::remap(self.target, 0, 255, 0, 20);
        75.0 - result
    }
}