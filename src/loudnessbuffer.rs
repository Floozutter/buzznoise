pub struct LoudnessBuffer {
    capacity: usize,
    squares: std::collections::VecDeque<f32>,
    sum: f32,
}

impl LoudnessBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            squares: std::collections::VecDeque::with_capacity(capacity),
            sum: 0.0,
        }
    }

    pub fn push(&mut self, sample: f32) {
        let square = sample.powi(2);
        self.squares.push_back(square);
        self.sum += square;
        while self.squares.len() > self.capacity {
            self.sum -= self.squares.pop_front().unwrap_or_default();
        }
        self.sum = self.sum.max(0.0);
    }

    pub fn rms(&self) -> f32 {
        (self.sum / self.squares.len() as f32).sqrt()
    }
}

impl Extend<f32> for LoudnessBuffer {
    fn extend<T: IntoIterator<Item=f32>>(&mut self, samples: T) {
        for s in samples {
            self.push(s);
        }
    }
}
