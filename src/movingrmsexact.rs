use super::movingrms::MovingRMS;

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

impl MovingRMS<f64> for MovingRMSExact<f64> {
    fn push(&mut self, value: f64) {
        let pushable = if self.squares.len() < self.capacity {
            true
        } else {
            self.squares.pop_front().is_some()
        };
        if pushable {
            self.squares.push_back(value * value);
        }
        debug_assert!(self.squares.len() <= self.capacity, "push exceeded capacity");
    }

    fn rms(&self) -> Option<f64> {
        if self.squares.len() > 0 {
            Some(self.squares.iter().sum::<f64>() / self.squares.len() as f64)
        } else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_rms_is_none() {
        let mut zero_capped = MovingRMSExact::<f64>::new(0);
        assert_eq!(zero_capped.rms(), None);
        for i in 0..100 {
            zero_capped.push(f64::from(i));
            assert_eq!(zero_capped.rms(), None);
        }
    }

    #[test]
    fn zeros() {
        let mut exact = MovingRMSExact::<f64>::new(64);
        for _ in 0..100 {
            exact.push(0.0);
            assert_eq!(exact.rms(), Some(0.0));
        }
    }
}
