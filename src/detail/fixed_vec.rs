pub struct FixedVec<const N: usize> {
    data: [u32; N],
    size: usize,
}

impl<const N: usize> Default for FixedVec<N> {
    fn default() -> Self {
        Self {
            data: [0; N],
            size: 0,
        }
    }
}

impl<const N: usize> FixedVec<N> {
    pub fn push(&mut self) {
        self.size += 1;
    }

    pub fn increment_at(&mut self, level: usize) {
        self.data[level] += 1;
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn pop(&mut self) {
        self.size -= 1;
        self.data[self.size] = 0;
    }

    pub fn clear(&mut self) {
        while !self.is_empty() {
            self.pop()
        }
    }

    pub fn get(&self, index: usize) -> u32 {
        self.data[index]
    }

    pub fn coincides_till(&self, other: &Self, index: usize) -> bool {
        self.data[0..index] == other.data[0..index]
    }
}
