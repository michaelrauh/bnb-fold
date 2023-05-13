
pub fn suffixes(xs: Vec<String>) -> Vec<Vec<String>> {
    let mut acc = vec![];
    for i in 0..xs.len() {
        let sliced: Vec<String> = xs[i..].to_vec();
        acc.push(sliced);
    }
    acc
}

pub fn split_sentence(sentence: String) -> Vec<String> {
    sentence
        .split_ascii_whitespace()
        .map(|x| x.to_string())
        .collect()
}

pub fn split_corpus(x: String) -> Vec<String> {
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

    use crate::string_handlers::{split_corpus, split_sentence};

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
    fn it_splits_a_corpus_to_sentences() {
        assert_eq!(
            split_corpus("a b! c d. e, f? g: h;".to_string()),
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
}
