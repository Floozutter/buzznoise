pub trait MovingRMS<I, O> {
    fn push(&mut self, value: I);
    fn rms(&self) -> O;
}
