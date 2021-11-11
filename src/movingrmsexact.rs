pub struct MovingRMSExact<T> {
    capacity: usize,
    squares: std::collections::VecDeque<T>,
}

impl<T> MovingRMSExact<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            squares: std::collections::VecDeque::with_capacity(capacity),
        }
    }
}
