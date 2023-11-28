#[derive(Debug)]
pub struct UniqueIdGen {
    id: u32,
}

impl UniqueIdGen {
    /// Create a new `UniqueIdGen` starting at 0.
    pub const fn new() -> Self {
        Self { id: 0 }
    }

    /// Set a minimum id. This ensures that all future ids are at least `min_id`.
    pub fn set_min(&mut self, min_id: u32) {
        if self.id < min_id {
            self.id = min_id;
        }
    }

    /// Generate the next id. This will always be greater than the previous id.
    pub fn next(&mut self) -> u32 {
        let res = self.id;
        self.id += 1;
        res
    }
}
