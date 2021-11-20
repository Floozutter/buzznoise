pub trait MovingRMS<T> {
    fn push(&mut self, value: T);
    fn rms(&self) -> Option<T>;

    fn extend<U: IntoIterator<Item=T>>(&mut self, iter: U) {
        for x in iter {
            self.push(x);
        }
    }
}
