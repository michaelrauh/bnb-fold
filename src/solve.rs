use std::{collections::HashSet, fs::read_to_string};

use crate::{
    rule::{full, get_diagonals, get_impacted_phrase_locations, make_blank, next_open_position},
    string_handlers,
};

pub fn solve_for_dims(dims: Vec<usize>) {
    // let corpus = "a b c d a c b d".to_string();
    let corpus = read_to_string("example.txt").unwrap();
    let vocab = string_handlers::vocabulary(&corpus);
    let phrases = string_handlers::corpus_to_set(&corpus);
    let initial = make_blank(&dims);
    let mut stack = vec![initial];
    let impacted_phrase_locations = get_impacted_phrase_locations(&dims);
    let impacted_diagonals = get_diagonals(&dims);

    loop {
        let cur = stack.pop();
        if cur.is_none() {
            println!("no results");
            break;
        }
        let current_answer = cur.unwrap();

        if full(&current_answer) {
            print!("found result:");
            println!("{:?}", current_answer);
            break;
        }

        let next_index = next_open_position(&current_answer);
        let impacted_phrases = &impacted_phrase_locations[next_index];
        let impacted_diagonal = &impacted_diagonals[next_index];
        let forbidden_words: HashSet<String> = impacted_diagonal
            .iter()
            .map(|idx| current_answer[*idx].as_ref().unwrap())
            .cloned()
            .collect();

        fun_name(&vocab, forbidden_words, impacted_phrases, current_answer, &phrases, next_index, &mut stack);
    }
}

fn fun_name(vocab: &Vec<String>, forbidden_words: HashSet<String>, impacted_phrases: &Vec<Vec<usize>>, current_answer: Vec<Option<String>>, phrases: &HashSet<Vec<String>>, next_index: usize, stack: &mut Vec<Vec<Option<String>>>) {
    'outer: for new_word in vocab {
        for forbidden_word in &forbidden_words {
            if new_word == forbidden_word {
                continue 'outer;
            }
        }

        for phrase in impacted_phrases {
            let mut current_real_phrase: Vec<String> = vec![];
            for word in phrase {
                let thing = current_answer[*word].as_ref().unwrap();
                current_real_phrase.push(thing.to_string());
            }
            current_real_phrase.push(new_word.to_string());
            if !phrases.contains(&current_real_phrase) {
                continue 'outer;
            }
        }

        let mut res = current_answer.clone();
        res[next_index] = Some(new_word.to_string());
        stack.push(res);
    }
}