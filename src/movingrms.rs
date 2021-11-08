pub trait MovingRMS<I, O> {
    fn rms(&self) -> O;
    fn push(&self, value: I);
}
