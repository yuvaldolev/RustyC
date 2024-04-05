pub struct LabelAllocator {
    // TODO: If we ever run of out of label numbers we can cache the next number
    // according to the label prefix.
    next: u64,
}

impl LabelAllocator {
    pub fn new() -> Self {
        Self { next: 0 }
    }

    pub fn allocate(&mut self, name: &str) -> String {
        let label = format!(".L.{}.{}", name, self.next);
        self.next += 1;

        label
    }
}
