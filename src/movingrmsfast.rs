pub struct MovingRMSFast<T> {
    capacity: usize,
    squares: std::collections::VecDeque<T>,
    sum: T,
}
