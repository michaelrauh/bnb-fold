
pub struct PhraseHash {
    h: u64,
    offset: u64
}
impl PhraseHash {
    pub fn new(offset: u64) -> Self {
        PhraseHash { h: 0, offset }
    }

    pub fn hash_in(&mut self, arg: u64) {
        self.h = (self.h * self.offset) + arg;
    }

    pub fn finish(&self) -> u64 {
        self.h
    }
}
