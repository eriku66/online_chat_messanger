#[derive(Debug)]
pub struct UserSession {
    pub last_received_at: std::time::Instant,
}

impl UserSession {
    pub fn new() -> Self {
        Self {
            last_received_at: std::time::Instant::now(),
        }
    }
}
