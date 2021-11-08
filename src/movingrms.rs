pub trait MovingRMS<I, O=I> {
    fn push(&mut self, value: I);
    fn rms(&self) -> O;
}
