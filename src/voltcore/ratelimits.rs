use std::time::Instant;

#[derive(Debug, Clone, Copy)]
pub struct TokenBucket {
    capacity: usize,
    tokens: usize,
    last_refresh: Instant,
}

impl TokenBucket {
    pub fn new(tokens: usize) -> Self {
        TokenBucket {
            capacity: tokens,
            tokens: tokens,
            last_refresh: Instant::now(),
        }
    }

    pub fn try_acquire(&mut self) -> bool {
        let refresh_time = Instant::now() - self.last_refresh;
        let tokens_to_add = (refresh_time.as_secs_f64() * self.capacity as f64).round() as usize;
        self.tokens = (self.tokens + tokens_to_add).min(self.capacity);

        if self.tokens > 0 {
            self.tokens -= 1;
            true
        } else {
            false
        }
    }
}