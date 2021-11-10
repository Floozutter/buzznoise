pub trait MovingRMS<T> {
    fn push(&mut self, value: T);
    fn rms(&self) -> Option<T>;
}
