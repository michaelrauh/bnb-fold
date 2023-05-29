use itertools::Itertools;
use nohash_hasher::IntSet;

use std::{collections::HashMap, hash::Hasher};

fn suffixes(xs: Vec<String>) -> Vec<Vec<String>> {
    let mut acc = vec![];
    for i in 0..xs.len() {
        let sliced: Vec<String> = xs[i..].to_vec();
        acc.push(sliced);
    }
    acc
}

pub struct Codec {
    pub coder: HashMap<String, u32>,
    pub decoder: HashMap<u32, String>,
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

fn split_sentence(sentence: String) -> Vec<String> {
    sentence
        .split_ascii_whitespace()
        .map(|x| x.to_string())
        .collect()
}

pub fn corpus_to_set(corpus: &str, max_length: usize, codec: &Codec) -> IntSet<u64> {
    let mut s = IntSet::default();
    s.reserve(1000000);

    for sentence in split_corpus(corpus) {
        let sentence_vec = split_sentence(sentence);
        let phrases = phrases(sentence_vec);
        for phrase in phrases {
            if phrase.len() <= max_length {
                let mut h = ahash::AHasher::default();
                for word in phrase {
                    h.write_u32(codec.coder[&word]);
                }
                s.insert(h.finish());
            }
        }
    }
    s
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

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::string_handlers::{phrases, prefixes, split_corpus, split_sentence};

    use super::suffixes;

    #[test]
    fn it_calculates_suffixes() {
        assert_eq!(
            suffixes(vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string()
            ]),
            vec![
                vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "d".to_string()
                ],
                vec!["b".to_string(), "c".to_string(), "d".to_string()],
                vec!["c".to_string(), "d".to_string()],
                vec!["d".to_string()]
            ]
        )
    }

    #[test]
    fn it_calculates_prefixes() {
        assert_eq!(
            prefixes(vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string()
            ]),
            vec![
                vec!["a".to_string(),],
                vec!["a".to_string(), "b".to_string()],
                vec!["a".to_string(), "b".to_string(), "c".to_string()],
                vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "d".to_string()
                ]
            ]
        )
    }

    #[test]
    fn it_calculates_phrases() {
        assert_eq!(
            phrases(vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string()
            ]),
            [
                vec!["a"],
                vec!["a", "b"],
                vec!["b"],
                vec!["a", "b", "c"],
                vec!["b", "c"],
                vec!["c"],
                vec!["a", "b", "c", "d"],
                vec!["b", "c", "d"],
                vec!["c", "d"],
                vec!["d"]
            ]
        )
    }

    #[test]
    fn it_splits_a_corpus_to_sentences() {
        assert_eq!(
            split_corpus(&"a b! c d. e, f? g: h;".to_string()),
            vec![
                "a b".to_string(),
                "c d".to_string(),
                "e f".to_string(),
                "g h".to_string()
            ]
        );
    }

    #[test]
    fn it_splits_a_sentence_to_words() {
        assert_eq!(
            split_sentence("a b c d".to_string()),
            vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string()
            ]
        );
    }


    // #[test]
    // fn it_creates_a_vocabulary_from_a_corpus() {
    //     assert_eq!(
    //         vocabulary(&"a b c d a d".to_string()),
    //         vec![
    //             "a".to_string(),
    //             "b".to_string(),
    //             "c".to_string(),
    //             "d".to_string()
    //         ]
    //     );
    // }
}
