pub(crate) mod cap;

/// first u64 is prev
/// second u64 last bit is recocable|first_badged
/// double linked list
/// default is null node
#[derive(Debug, Copy, Clone, Default)]
pub struct MdbNode {
    words: [u64; 2],
}

/// rust style
/// ignore sign extend
impl MdbNode {
    /// get next
    const MASK: u64 = 0xfffffffffffc;
    pub fn new(next: u64, revocable: u64, first_badged: u64, prev: u64) -> MdbNode {
        let word0 = prev;
        let word1 = (next & Self::MASK) | (revocable & 0x1) << 1 | (first_badged & 0x1);
        Self {
            words: [word0, word1],
        }
    }

    /// should be a ptr
    pub fn next(&self) -> u64 {
        self.words[1] & Self::MASK
    }

    pub fn set_next(&mut self, next: u64) {
        // reserve last 2 bits
        self.words[1] &= !Self::MASK;
        // set high bits
        self.words[1] |= (next >> 0) & Self::MASK;
    }

    /// should be a ptr
    pub fn prev(&self) -> u64 {
        self.words[0]
    }

    pub fn set_prev(&mut self, prev: u64) {
        self.words[0] = prev
    }

    pub fn revocable(&self) -> u64 {
        self.words[1] & 0x2
    }

    /// only one bit
    /// use bool
    pub fn set_revocable(&mut self, revocable: u64) {
        self.words[1] &= !0x2;
        self.words[1] |= (revocable << 1) & 0x2;
    }

    pub fn first_badged(&self) -> u64 {
        self.words[1] & 0x1
    }

    /// only one bit
    /// use bool
    pub fn set_first_badged(&mut self, first_badged: u64) {
        self.words[1] &= !0x1;
        self.words[1] |= first_badged & 0x1;
    }
}

#[cfg(test)]
mod tests {
    /// TODO add tests
    fn test() {}
}