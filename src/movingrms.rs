pub trait MovingRMS<I, O> {
    fn rms(&self) -> O;
    fn push(&mut self, value: I);
}
