
pub struct PhraseSet {
    base: Vec<bool>,
    offset: usize
 
}

pub struct PhraseHash {
    h: usize,
    offset: usize
}
impl PhraseHash {
    fn new(offset: usize) -> Self {
        PhraseHash { h: 0, offset }
    }

    pub fn hash_in(&mut self, arg: usize) {
        self.h = (self.h * self.offset) + arg;
    }

    pub fn finish(&self) -> usize {
        self.h
    }
}

impl PhraseSet {
    pub fn new(vocab_size: usize, max_phrase_length: usize) -> Self {
        let mut h = PhraseHash::new(vocab_size);
        for _ in 0..max_phrase_length {
            h.hash_in(vocab_size);
        }
        let total = h.finish();
        println!("offset is: {}", vocab_size);
        println!("size is: {}", total);
        PhraseSet {base: vec![false; total], offset:vocab_size}
    }

    pub fn insert(&mut self, phrase: Vec<usize>) {
        let mut h = PhraseHash::new(self.offset);
        for cur in phrase {
            h.hash_in(cur);
        }
        self.base[h.finish()] = true;
    }

    pub fn contains(&self, h: PhraseHash) -> bool {
        self.base[h.finish()]
    }

    pub fn len(&self) -> usize {
        self.base.len()
    }

    pub fn produce_empty_hash(&self) -> PhraseHash {
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

    let mut h_4 = PhraseHash::new(100);
    h_4.hash_in(1);
    h_4.hash_in(1);
    h_4.hash_in(1);

    assert_eq!(h_4.finish(), 10101);

    let mut h_5 = PhraseHash::new(10);
    h_5.hash_in(2);
    h_5.hash_in(3);
    h_5.hash_in(4);

    assert_eq!(h_5.finish(), 234);

    let mut h_6 = PhraseHash::new(100);
    h_6.hash_in(99);
    h_6.hash_in(99);
    h_6.hash_in(99);

    assert_eq!(h_6.finish(), 999999);
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