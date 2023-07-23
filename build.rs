use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fs::{File, read_to_string};
use std::hash::Hasher;
use std::io::{BufWriter, Write};
use std::path::Path;
use itertools::Itertools;
use nohash_hasher::IntSet;


pub struct Codec {
    pub coder: HashMap<String, u32>,
    pub decoder: HashMap<u32, String>,
}

fn split_sentence(sentence: String) -> Vec<String> {
    sentence
        .split_ascii_whitespace()
        .map(|x| x.to_string())
        .collect()
}


fn split_corpus(x: &str) -> Vec<String> {
    x.split_terminator(&['.', '!', '?', ';'])
        .filter(|x| !x.is_empty())
        .map(|x| x.trim())
        .map(|sentence| {
            sentence
                .split_ascii_whitespace()
                .map(|s| {
                    s.chars()
                        .filter(|c| c.is_alphabetic())
                        .collect::<String>()
                        .to_lowercase()
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect()
}

pub fn make_codec(corpus: &str) -> Codec {
    let vocab: Vec<String> = split_corpus(corpus)
        .into_iter()
        .flat_map(split_sentence)
        .unique()
        .collect();
    let coder: HashMap<String, u32> = vocab
        .iter()
        .enumerate()
        .map(|(id, word)| (word.to_string(), id as u32))
        .collect();
    let decoder = coder
        .iter()
        .map(|(k, v)| (v.to_owned(), k.to_owned()))
        .collect();
    Codec {
        coder,
        decoder,
    }
}

fn suffixes(xs: Vec<String>) -> Vec<Vec<String>> {
    let mut acc = vec![];
    for i in 0..xs.len() {
        let sliced: Vec<String> = xs[i..].to_vec();
        acc.push(sliced);
    }
    acc
}

fn prefixes(xs: Vec<String>) -> Vec<Vec<String>> {
    let mut acc = vec![];
    for i in 1..xs.len() + 1 {
        let sliced: Vec<String> = xs[..i].to_vec();
        acc.push(sliced);
    }
    acc
}

fn phrases(xs: Vec<String>) -> Vec<Vec<String>> {
    prefixes(xs)
        .iter()
        .flat_map(|x| suffixes(x.to_vec()))
        .collect()
}

pub struct PhraseHash {
    h: u64,
    offset: u64
}
impl PhraseHash {
    fn new(offset: u64) -> Self {
        PhraseHash { h: 0, offset }
    }

    pub fn hash_in(&mut self, arg: u32) {
        self.h = (self.h * self.offset) + (arg as u64);
    }

    pub fn finish(&self) -> u64 {
        self.h
    }
}

pub fn corpus_to_set(corpus: &str, max_length: usize, codec: &Codec, length: usize) -> Vec<u64> {
    let mut s = vec![];

    for sentence in split_corpus(corpus) {
        let sentence_vec = split_sentence(sentence);
        let phrases = phrases(sentence_vec);
        for phrase in phrases {
            if phrase.len() <= max_length {
                let mut h = PhraseHash::new(length as u64);
                for word in phrase {
                    h.hash_in(codec.coder[&word]);
                }
                let to_add = h.finish();
                println!("{}", to_add);
                s.push(to_add);
            }
        }
    }
    s.into_iter().unique().collect_vec()
}

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let dims = vec![4,4];

    let max_length = *dims.iter().max().unwrap();
    let corpus = read_to_string("example.txt").unwrap();

    let codec = make_codec(&corpus);
    let vocab: Vec<&u32> = codec.coder.values().sorted().collect();
    let phrases = corpus_to_set(&corpus, max_length, &codec, vocab.len());

    
    let mut static_phrases = phf_codegen::Set::new();
    let mut v = phf_codegen::Set::new();
    let mut dc = phf_codegen::Map::new();
    
    write!(
        &mut file,
        "static DIMS: phf::Set<&'static str> = {}",
        phf_codegen::Set::new().entry(dims.into_iter().join(",")).build()
    )
    .unwrap();
    write!(&mut file, ";\n").unwrap();

    phrases.into_iter().for_each(|p|
    {
        static_phrases.entry(p);
    });

    vocab.into_iter().for_each(|word|
        {
            v.entry(word.to_owned());
        });

    codec.decoder.into_iter().for_each(|(k, v)| {
        dc.entry(k, &format!("\"{}\"", &v));
    });

    write!(
        &mut file,
        "static PHRASES: phf::Set<u64> = {}",
        static_phrases.build()
    )
    .unwrap();
    write!(&mut file, ";\n").unwrap();

    write!(
        &mut file,
        "static VOCAB: phf::Set<u32> = {}",
        v.build()
    )
    .unwrap();
    write!(&mut file, ";\n").unwrap();

    write!(
        &mut file,
        "static DECODER: phf::Map<u32, &str> = {}",
        dc.build()
    )
    .unwrap();
    write!(&mut file, ";\n").unwrap();
}
