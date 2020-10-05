pub struct OnceIn {
    i: usize,
    n: usize,
}

impl OnceIn {
    /// panics if n == 0
    pub fn new(n: usize) -> Self {
        if n == 0 {
            panic!("n == 0")
        }
        Self { i: 0, n }
    }
}

impl Iterator for OnceIn {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        self.i = (self.i + 1) % self.n;
        Some(self.i == 0)
    }
}
