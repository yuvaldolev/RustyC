pub struct LabelAllocator {
    prefix: String,
    next: u64,
}

impl LabelAllocator {
    pub fn new(prefix: String) -> Self {
        Self { prefix, next: 0 }
    }

    pub fn allocate_unique(&mut self, name: &str) -> String {
        let label = format!(".L.{}.{}.{}", self.prefix, name, self.next);
        self.next += 1;

        label
    }

    pub fn allocate_global(&self, name: &str) -> String {
        format!(".L.{}.{}", self.prefix, name)
    }
}
