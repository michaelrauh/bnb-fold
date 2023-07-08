
struct PhraseSet {
    base: Vec<bool>,
    offset: usize
 
}

struct PhraseHash {
    h: usize,
    offset: usize,
    depth: usize
}
impl PhraseHash {
    fn new(offset: usize) -> Self {
        PhraseHash { h: 0, offset, depth: 0 }
    }

    fn hash_in(&mut self, arg: usize) {
        self.h = (self.h * (self.offset * self.depth)) + arg;
        self.depth += 1;
    }

    fn finish(&self) -> usize {
        self.h
    }
}

impl PhraseSet {
    fn new(vocab_size: usize, max_phrase_length: usize) -> Self {
        let mut h = PhraseHash::new(vocab_size);
        for _ in 0..max_phrase_length {
            h.hash_in(vocab_size);
        }

        PhraseSet {base: vec![false; h.finish()], offset:vocab_size}
    }

    fn insert(&mut self, phrase: Vec<usize>) {
        let mut h = PhraseHash::new(self.offset);
        for cur in phrase {
            h.hash_in(cur);
        }
        self.base[h.finish()] = true;
    }

    fn contains(&self, h: PhraseHash) -> bool {
        self.base[h.finish()]
    }

    fn produce_empty_hash(&self) -> PhraseHash {
        PhraseHash::new(self.offset)
    }
}


#[test]
fn it_hashes_in_one() {
    let mut h = PhraseHash::new(10);
    h.hash_in(2);

    assert_eq!(h.finish(), 2);
}

#[test]
fn it_hashes_shifts_first_by_range_then_adds_second() {
    let mut h = PhraseHash::new(10);
    h.hash_in(2);
    h.hash_in(3);

    assert_eq!(h.finish(), 23);

    let mut h_2 = PhraseHash::new(10);
    h_2.hash_in(1);
    h_2.hash_in(4);

    assert_eq!(h_2.finish(), 14);

    let mut h_3 = PhraseHash::new(10);
    h_3.hash_in(3);
    h_3.hash_in(2);

    assert_eq!(h_3.finish(), 32);

    //////////////

    let mut h = PhraseHash::new(12);
    h.hash_in(2);
    h.hash_in(3);

    assert_eq!(h.finish(), 27);

    let mut h_2 = PhraseHash::new(12);
    h_2.hash_in(1);
    h_2.hash_in(4);

    assert_eq!(h_2.finish(), 16);

    let mut h_3 = PhraseHash::new(12);
    h_3.hash_in(3);
    h_3.hash_in(2);

    assert_eq!(h_3.finish(), 38);
}

#[test]
fn it_stores_phrases() {
    let s = 100;
    let mut subject = PhraseSet::new(s, 5);
    let mut h = subject.produce_empty_hash();
    
    subject.insert(vec![1,2,3]);
    
    h.hash_in(1);
    h.hash_in(2);
    h.hash_in(3);

    assert!(subject.contains(h));

    let mut other_h = subject.produce_empty_hash();
    other_h.hash_in(1);
    other_h.hash_in(2);
    other_h.hash_in(3);
    other_h.hash_in(4);
    assert!(!subject.contains(other_h));

    let mut third_h = subject.produce_empty_hash();
    third_h.hash_in(1);
    third_h.hash_in(2);
    third_h.hash_in(4);
    assert!(!subject.contains(third_h));
}

#[test]
fn it_does_not_overflow_if_initialized_correctly() {
    let s = 101;
    let mut subject = PhraseSet::new(s, 2);
    let mut h = subject.produce_empty_hash();
    
    subject.insert(vec![100, 100]);
    
    h.hash_in(100);
    h.hash_in(100);

    assert!(subject.contains(h));
}