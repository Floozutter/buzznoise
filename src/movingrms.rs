pub trait MovingRMS {
    fn rms(&self) -> f32;
    fn push(&self, value: f32);
}
